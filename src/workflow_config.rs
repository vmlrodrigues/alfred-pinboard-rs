use chrono::prelude::*;
use core::fmt;
use dirs::home_dir;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::PathBuf;

use alfred_rs::Data;

use semver::{Version, VersionReq};

use crate::AlfredError;

pub(crate) const CONFIG_FILE_NAME: &str = "settings.json";
pub(crate) const CONFIG_KEY_NAME: &str = "settings";

const FILE_BUF_SIZE: usize = 4 * 1024 * 1024;

/// Debug is directly implemented so that we do not show the `Config.auth_token` in Alfred's debug window.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("alfred_version", &self.alfred_version)
            .field("pins_to_show", &self.pins_to_show)
            .field("tags_to_show", &self.tags_to_show)
            .field("tags_only_search", &self.tag_only_search)
            .field("fuzzy_search", &self.fuzzy_search)
            .field("private_new_pin", &self.private_new_pin)
            .field("toread_new_pin", &self.toread_new_pin)
            .field("suggest_tags", &self.suggest_tags)
            .field("page_is_bookmarked", &self.page_is_bookmarked)
            .field("show_url_vs_tags", &self.show_url_vs_tags)
            .field("auto_update_cache", &self.auto_update_cache)
            .field("update_time", &self.update_time)
            .field("workflow_data_dir", &self.workflow_data_dir)
            .field("workflow_cache_dir", &self.workflow_cache_dir)
            .finish_non_exhaustive()
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Which version of Alfred we are being executed under
    #[serde(skip, default = "get_alfred_version")]
    pub alfred_version: Version,
    /// Number of bookmarks to show in Alfred
    pub pins_to_show: u8,
    /// Number of tags to show in Alfred
    pub tags_to_show: u8,
    /// Flag to perform search only on `tag` fields of bookmarks
    pub tag_only_search: bool,
    /// Flag to perform a fuzzy search
    pub fuzzy_search: bool,
    /// Flag to save bookmarks as private
    pub private_new_pin: bool,
    /// Flag to save bookmarks as toread
    pub toread_new_pin: bool,
    /// Flag to suggest popular tags for the browser's current url
    pub suggest_tags: bool,
    /// Flag to check if browser's page is already bookmarked
    #[serde(default)]
    pub page_is_bookmarked: bool,
    /// Flag to show either url or tags of current bookmark
    #[serde(default)]
    pub show_url_vs_tags: bool,
    /// Flag to update cache after each bookmark saving automatically.
    pub auto_update_cache: bool,
    /// Authentication Token.
    ///
    /// Deserialised so an install that predates the move can still be migrated,
    /// but never serialised — the token belongs in the Keychain, and writing it
    /// here again would put it back in plaintext on disk. See `resolve_token`.
    #[serde(default, skip_serializing)]
    pub auth_token: String,
    /// Last time cache was updated
    #[serde(default = "get_epoch")]
    pub update_time: DateTime<Utc>,
    /// Whether the one-time copy of these settings into Alfred's workflow
    /// configuration sheet has already happened. See `migrate_to_alfred_config`.
    #[serde(default)]
    pub migrated_to_alfred_config: bool,

    /// Folder to store volatile data of the workflow
    workflow_data_dir: PathBuf,
    /// Folder to store data of the workflow
    workflow_cache_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        debug!("Starting in new");
        let mut cfg = Config {
            alfred_version: get_alfred_version(),
            pins_to_show: 10,
            tags_to_show: 10,
            tag_only_search: false,
            fuzzy_search: false,
            private_new_pin: true,
            toread_new_pin: false,
            suggest_tags: false,
            page_is_bookmarked: true,
            auto_update_cache: true,
            show_url_vs_tags: true,
            auth_token: String::new(),
            update_time: get_epoch(),
            // A fresh install has nothing to migrate: Alfred's configuration
            // sheet already holds the workflow's defaults.
            migrated_to_alfred_config: true,
            workflow_data_dir: PathBuf::default(),
            workflow_cache_dir: PathBuf::default(),
        };
        cfg.discover_dirs();
        cfg
    }

    pub fn setup() -> Result<Config, Box<dyn std::error::Error>> {
        debug!("Starting in setup");
        let (data_dir, cache_dir) = Config::get_workflow_dirs();
        let mut config = Config::read(data_dir, cache_dir)?;
        config.resolve_token();

        // An install that predates the move to Alfred's configuration sheet has
        // its preferences only in settings.json. Push them into the sheet before
        // reading the environment, otherwise this run — and every run after it —
        // would see the workflow's defaults instead of the user's choices.
        if !config.migrated_to_alfred_config {
            match config.migrate_to_alfred_config() {
                Ok(()) => {
                    info!("migrated settings into Alfred's workflow configuration");
                    config.migrated_to_alfred_config = true;
                    if let Err(e) = config.save() {
                        // Not fatal: we simply try again next time.
                        error!(
                            "couldn't record config migration: {}",
                            crate::redact_token(&e.to_string())
                        );
                    }
                }
                Err(e) => {
                    error!(
                        "couldn't migrate settings into Alfred's configuration: {}",
                        crate::redact_token(&e.to_string())
                    );
                }
            }
            // Either way this run keeps using the stored values: the environment
            // this process was launched with predates the change.
            return Ok(config);
        }

        config.apply_env_overrides();
        Ok(config)
    }

    fn read(
        mut data_dir: PathBuf,
        cached_dir: PathBuf,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        debug!("Starting in read");
        data_dir.push(CONFIG_FILE_NAME);
        if data_dir.exists() {
            let config = Data::load(&data_dir)?;
            let config: Result<Config, Box<dyn std::error::Error>> = config
                .get(CONFIG_KEY_NAME)
                .map_or_else(
                    || {
                        // println!("--> Resorting to old config file format.");
                        File::open(&data_dir).map_err(Into::into).and_then(|fp| {
                            let buf_reader = BufReader::with_capacity(FILE_BUF_SIZE, fp);
                            serde_json::from_reader(buf_reader)
                                .map_err(|_| From::from(AlfredError::ConfigFileErr))
                        })
                    },
                    Ok,
                )
                .map(|mut c: Config| {
                    assert!(data_dir.pop());
                    c.workflow_data_dir = data_dir;
                    c.workflow_cache_dir = cached_dir;
                    c
                });
            config
        } else {
            Err(From::from(AlfredError::MissingConfigFile))
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Starting in save");
        create_dir_all(self.data_dir())?;

        let mut mydata = Data::load(CONFIG_FILE_NAME)?;
        mydata.clear();
        mydata.set(CONFIG_KEY_NAME, self).map_err(Into::into)
    }

    pub fn discover_dirs(&mut self) {
        debug!("Starting in discover_dirs");
        let dirs = Config::get_workflow_dirs();
        self.workflow_data_dir = dirs.0;
        self.workflow_cache_dir = dirs.1;
    }

    /// Resolve the API token, moving it out of `settings.json` and into the
    /// macOS Keychain the first time.
    ///
    /// Older installs kept the token in plaintext in the workflow's data
    /// directory. On the first run after upgrading we copy it into the Keychain
    /// and blank the stored copy, so it stops sitting on disk in the clear.
    ///
    /// A Keychain failure is not fatal: we carry on with the value from
    /// `settings.json` so the workflow keeps working, and try again next run.
    pub fn resolve_token(&mut self) {
        debug!("Starting in resolve_token");
        let stored_in_plaintext = !self.auth_token.is_empty();

        if let Some(token) = crate::keychain::get() {
            self.auth_token = token;
        } else if stored_in_plaintext {
            match crate::keychain::set(&self.auth_token) {
                Ok(()) => info!("moved the API token into the Keychain"),
                // Not fatal: keep using the plaintext copy so the workflow still
                // works, and try again next run.
                Err(e) => {
                    error!("couldn't store the token in the Keychain: {e}");
                    return;
                }
            }
        } else {
            return;
        }

        // `auth_token` is `skip_serializing`, so rewriting the file is what
        // removes any lingering plaintext copy.
        if stored_in_plaintext {
            if let Err(e) = self.save() {
                error!(
                    "couldn't rewrite settings.json without the token: {}",
                    crate::redact_token(&e.to_string())
                );
            }
        }
    }

    /// Overlay Alfred's workflow configuration on top of what was read from
    /// `settings.json`.
    ///
    /// Alfred 5 keeps user-settable options in the workflow's configuration
    /// sheet and hands them to every script as environment variables, so the
    /// sheet is the source of truth for these ten. A variable that is missing
    /// or unparseable leaves the stored value alone rather than snapping to a
    /// default, so a malformed entry degrades instead of resetting preferences.
    pub fn apply_env_overrides(&mut self) {
        debug!("Starting in apply_env_overrides");
        if let Some(v) = env_bool(cfg_env::TAG_ONLY_SEARCH) {
            self.tag_only_search = v;
        }
        if let Some(v) = env_bool(cfg_env::FUZZY_SEARCH) {
            self.fuzzy_search = v;
        }
        // The sheet asks "share new bookmarks?", which is the inverse of how
        // this is stored. Keep the question positive for the user and flip here.
        if let Some(v) = env_bool(cfg_env::SHARED_NEW_PIN) {
            self.private_new_pin = !v;
        }
        if let Some(v) = env_bool(cfg_env::TOREAD_NEW_PIN) {
            self.toread_new_pin = v;
        }
        if let Some(v) = env_bool(cfg_env::SUGGEST_TAGS) {
            self.suggest_tags = v;
        }
        if let Some(v) = env_bool(cfg_env::PAGE_IS_BOOKMARKED) {
            self.page_is_bookmarked = v;
        }
        if let Some(v) = env_bool(cfg_env::SHOW_URL_VS_TAGS) {
            self.show_url_vs_tags = v;
        }
        if let Some(v) = env_bool(cfg_env::AUTO_UPDATE_CACHE) {
            self.auto_update_cache = v;
        }
        if let Some(v) = env_count(cfg_env::PINS_TO_SHOW) {
            self.pins_to_show = v;
        }
        if let Some(v) = env_count(cfg_env::TAGS_TO_SHOW) {
            self.tags_to_show = v;
        }
    }

    /// Copy the settings that used to live in `settings.json` into Alfred's
    /// workflow configuration sheet, exactly once.
    ///
    /// Before 0.19.0 this binary owned these options and `pset` wrote them to
    /// `settings.json`. They now live in Alfred's configuration sheet, which on
    /// upgrade starts out holding the workflow's *defaults* — so without this
    /// step an existing user's preferences would silently reset the first time
    /// they used the workflow after updating.
    ///
    /// Failure is not fatal: the stored values still drive this run, and the
    /// flag is only recorded when the copy actually succeeded, so it is retried
    /// next time.
    fn migrate_to_alfred_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        debug!("Starting in migrate_to_alfred_config");

        let bundle_id =
            std::env::var("alfred_workflow_bundleid").map_err(|_| AlfredError::MissingBundleId)?;

        // (key, value from settings.json, value the configuration sheet ships with)
        let d = Config::new();
        let settings: [(&str, String, String); 10] = [
            (
                cfg_env::TAG_ONLY_SEARCH,
                bool_str(self.tag_only_search),
                bool_str(d.tag_only_search),
            ),
            (
                cfg_env::FUZZY_SEARCH,
                bool_str(self.fuzzy_search),
                bool_str(d.fuzzy_search),
            ),
            // Stored inverted — see `apply_env_overrides`.
            (
                cfg_env::SHARED_NEW_PIN,
                bool_str(!self.private_new_pin),
                bool_str(!d.private_new_pin),
            ),
            (
                cfg_env::TOREAD_NEW_PIN,
                bool_str(self.toread_new_pin),
                bool_str(d.toread_new_pin),
            ),
            (
                cfg_env::SUGGEST_TAGS,
                bool_str(self.suggest_tags),
                bool_str(d.suggest_tags),
            ),
            (
                cfg_env::PAGE_IS_BOOKMARKED,
                bool_str(self.page_is_bookmarked),
                bool_str(d.page_is_bookmarked),
            ),
            (
                cfg_env::SHOW_URL_VS_TAGS,
                bool_str(self.show_url_vs_tags),
                bool_str(d.show_url_vs_tags),
            ),
            (
                cfg_env::AUTO_UPDATE_CACHE,
                bool_str(self.auto_update_cache),
                bool_str(d.auto_update_cache),
            ),
            (
                cfg_env::PINS_TO_SHOW,
                self.pins_to_show.to_string(),
                d.pins_to_show.to_string(),
            ),
            (
                cfg_env::TAGS_TO_SHOW,
                self.tags_to_show.to_string(),
                d.tags_to_show.to_string(),
            ),
        ];

        // One osascript invocation for all ten: spawning it ten times would add
        // a noticeable delay to the keystroke that happens to trigger this.
        let mut script = String::from("tell application id \"com.runningwithcrayons.Alfred\"\n");
        let mut wrote = 0usize;
        for (key, stored, shipped_default) in &settings {
            // Alfred hands us a value for every key, because the sheet ships with
            // defaults — so "is the variable set?" cannot tell us whether the user
            // has been in the panel. What it can tell us is whether the value still
            // equals the shipped default. Anything that differs was chosen
            // deliberately there and must survive this migration: reading the
            // release note, opening the panel and setting things up before the
            // first keystroke is exactly what a careful user would do.
            if std::env::var(*key).is_ok_and(|current| current.trim() != shipped_default) {
                debug!("{key} already customised in the configuration sheet, not overwriting");
                continue;
            }
            script.push_str(&format!(
                "set configuration \"{key}\" to value \"{stored}\" in workflow \"{bundle_id}\"\n"
            ));
            wrote += 1;
        }
        script.push_str("end tell");

        if wrote == 0 {
            debug!("nothing to migrate; the configuration sheet is already set up");
            return Ok(());
        }

        let output = Command::new("osascript").arg("-e").arg(&script).output()?;
        if output.status.success() {
            Ok(())
        } else {
            // Report what actually failed. Mapping this to ConfigFileErr told the
            // user to "set API token again" for a problem that has nothing to do
            // with their token, and threw away osascript's message.
            Err(Box::new(AlfredError::OsascriptError(
                String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            )))
        }
    }

    #[allow(dead_code)]
    pub fn cache_dir(&self) -> &PathBuf {
        &self.workflow_cache_dir
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.workflow_data_dir
    }

    pub fn can_use_json(&self) -> bool {
        // Alfred v3 & above support reading/writing Items in json format
        debug!("Starting in can_use_json");
        let version_3 =
            VersionReq::parse(">=3").expect("Couldn't parse >= 3, >=5.0.0-E version string");
        // SemVer only allows versions with prerelease to match with a comparator that has a
        // prerelease requirement in it. Alfred 5.0 early access is causing a lot of
        // issues, hence the ">=5.0.0-E" bit was added
        let version_ea = VersionReq::parse(">=5.0.0-E").unwrap();
        let r = version_3.matches(&self.alfred_version) || version_ea.matches(&self.alfred_version);
        debug!("alfred_version: {:?}", &self.alfred_version);
        debug!("can_use_json: {}", r);
        r
    }

    fn get_workflow_dirs() -> (PathBuf, PathBuf) {
        debug!("Starting in get_workflow_dirs");
        let cache_dir = alfred::env::workflow_cache().unwrap_or_else(|| {
            let mut dir = home_dir().unwrap_or_else(|| PathBuf::from(""));
            dir.push(".cache");
            dir.push("pinion");
            dir
        });
        debug!("cache_dir: {}", cache_dir.to_string_lossy());
        if !cache_dir.exists() {
            // If we can't create directories, workflow won't be able to work, so we panic!
            debug!("creating cache_dir");
            create_dir_all(&cache_dir).unwrap();
        }
        let data_dir = alfred::env::workflow_data().unwrap_or_else(|| {
            let mut dir = home_dir().unwrap_or_else(|| PathBuf::from(""));
            dir.push(".config");
            dir.push("pinion");
            dir
        });
        debug!("data_dir: {}", data_dir.to_string_lossy());
        if !data_dir.exists() {
            // If we can't create directories, workflow won't be able to work, so we panic!
            debug!("creating data_dir");
            create_dir_all(&data_dir).unwrap();
        }

        (data_dir, cache_dir)
    }
}

/// Keys of the workflow-configuration entries Alfred exports to us as
/// environment variables. These must match the `variable` of each entry in
/// `userconfigurationconfig` in `res/workflow/info.plist`.
pub(crate) mod cfg_env {
    pub const PINS_TO_SHOW: &str = "pins_to_show";
    pub const TAGS_TO_SHOW: &str = "tags_to_show";
    pub const TAG_ONLY_SEARCH: &str = "tag_only_search";
    pub const FUZZY_SEARCH: &str = "fuzzy_search";
    pub const SHARED_NEW_PIN: &str = "shared_new_pin";
    pub const TOREAD_NEW_PIN: &str = "toread_new_pin";
    pub const SUGGEST_TAGS: &str = "suggest_tags";
    pub const PAGE_IS_BOOKMARKED: &str = "page_is_bookmarked";
    pub const SHOW_URL_VS_TAGS: &str = "show_url_vs_tags";
    pub const AUTO_UPDATE_CACHE: &str = "auto_update_cache";
}

/// How a boolean is written into Alfred's configuration sheet. Alfred's own
/// checkboxes use "1"/"0", so match that.
fn bool_str(b: bool) -> String {
    if b { "1" } else { "0" }.to_string()
}

/// Parse a checkbox value from Alfred's configuration sheet.
///
/// Alfred writes "1"/"0", but the sheet is user-editable and these values can
/// also be set by hand or by AppleScript, so accept the obvious spellings.
/// Anything else — including empty — returns `None` so the caller keeps the
/// value it already had.
pub(crate) fn parse_bool(s: &str) -> Option<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

/// Parse a "how many rows to show" value. Zero is rejected rather than honoured:
/// it would render an empty Alfred window that looks like a broken workflow, and
/// is far more likely to be a mistake than an intent.
pub(crate) fn parse_count(s: &str) -> Option<u8> {
    s.trim().parse::<u8>().ok().filter(|n| *n > 0)
}

fn env_bool(key: &str) -> Option<bool> {
    std::env::var(key).ok().as_deref().and_then(parse_bool)
}

fn env_count(key: &str) -> Option<u8> {
    std::env::var(key).ok().as_deref().and_then(parse_count)
}

fn get_alfred_version() -> Version {
    debug!("Starting in get_alfred_version");
    let min_version = 2;
    let v2 = Version::new(min_version, 0, 0); // If alfred_version env. cannot be found or parsed according to
                                              // semver.org, we will return this version.
    alfred::env::version().map_or(v2.clone(), |ref s| {
        Version::parse(s).unwrap_or_else(|_| {
            // Alfred version is not semver compliant, thus
            s.find('.') // find first dot
                .map_or(v2, |idx| {
                    let m = s[..idx].parse::<u64>().unwrap_or(min_version); // and parse the number before it
                    Version::new(m, 0, 0)
                })
        })
    })
}

fn get_epoch() -> DateTime<Utc> {
    debug!("Starting in get_epoch");
    "1970-01-01T23:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("Couldn't create UTC epoch time")
}

#[cfg(test)]
mod tests {
    use super::{bool_str, parse_bool, parse_count};

    #[test]
    fn parses_alfreds_own_checkbox_values() {
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("0"), Some(false));
    }

    #[test]
    fn parses_hand_written_checkbox_values() {
        for s in ["true", "TRUE", " yes ", "On"] {
            assert_eq!(parse_bool(s), Some(true), "{s:?} should parse as true");
        }
        for s in ["false", "FALSE", " no ", "Off"] {
            assert_eq!(parse_bool(s), Some(false), "{s:?} should parse as false");
        }
    }

    /// An unparseable value must not be read as `false`, or a typo in the
    /// configuration sheet would silently turn a setting off.
    #[test]
    fn rejects_unparseable_booleans_rather_than_defaulting() {
        for s in ["", "  ", "maybe", "2", "y"] {
            assert_eq!(parse_bool(s), None, "{s:?} should not parse");
        }
    }

    #[test]
    fn round_trips_booleans_through_alfreds_format() {
        assert_eq!(parse_bool(&bool_str(true)), Some(true));
        assert_eq!(parse_bool(&bool_str(false)), Some(false));
    }

    #[test]
    fn parses_row_counts() {
        assert_eq!(parse_count("10"), Some(10));
        assert_eq!(parse_count(" 25 "), Some(25));
        assert_eq!(parse_count("255"), Some(255));
    }

    /// Zero would render an empty Alfred window that reads as a broken
    /// workflow, and out-of-range values must not wrap.
    #[test]
    fn rejects_zero_and_out_of_range_counts() {
        for s in ["0", "256", "-1", "", "ten", "1.5"] {
            assert_eq!(parse_count(s), None, "{s:?} should not parse");
        }
    }
}
