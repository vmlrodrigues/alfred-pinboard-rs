use clap::{Parser, Subcommand};

/// Pinboard API token that never appears in `Debug` output.
///
/// `SubCommand` derives `Debug` and is logged in `main`, so holding the token
/// in a bare `String` would print it into Alfred's debug window.
#[derive(Clone)]
pub struct AuthToken(pub String);

impl std::str::FromStr for AuthToken {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl std::fmt::Debug for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

/// Spelled out rather than relying on clap's `FromStr` fallback, so the token
/// type keeps its redacting `Debug` without depending on inference.
fn parse_auth_token(s: &str) -> Result<AuthToken, std::convert::Infallible> {
    Ok(AuthToken(s.to_owned()))
}

#[derive(Parser, Debug)]
// `propagate_version` keeps `-V/--version` available on every subcommand, which
// is what structopt/clap 2 did. clap 4 only puts it on the root by default.
#[command(name = "alfred-pinboard", version, propagate_version = true)]
/// Command line component of Alfred Workflow for Pinboard (Written in Rust!)
pub struct Opt {
    #[arg(long = "debug", default_value = "0")]
    pub _debug_level: i8,
    /// Show exact user query at the top of Alfred's item list
    #[arg(short = 'q', long = "query-as-item")]
    pub query_as_item: bool,
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// CLI verbs/commands and their options.
///
/// Note that the `Option<bool>` options below take an explicit `true`/`false`
/// value rather than being flags — the workflow invokes them as
/// `--suggest false`, `--shared "$shared"` and so on.
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Configures options and settings of interacting with API and searching items.
    #[command(name = "config")]
    Config {
        /// Show all the configuration settings, after setting any given config options.
        #[arg(short = 'd', long = "display")]
        display: bool,

        /// Set API authorization token.
        /// (Obtain it from your Pinboard account's setting page).
        #[arg(short = 'a', long = "authorization", value_parser = parse_auth_token)]
        auth_token: Option<AuthToken>,

        /// Number of bookmarks to show in Alfred's window. [default: 10]
        #[arg(short = 'p', long = "bookmark-numbers")]
        number_pins: Option<u8>,

        /// Number of tags to show in Alfred's window. [default: 10]
        #[arg(short = 'l', long = "tag-numbers")]
        number_tags: Option<u8>,

        /// By default, make all new bookmarks public/shared. [default: false]
        #[arg(short = 's', long = "shared")]
        shared: Option<bool>,

        /// By default, set all new bookmarks' toread flag. [default: false]
        #[arg(short = 'r', long = "toread")]
        toread: Option<bool>,

        /// When searching tags/bookmarks, enable 'fuzzy' searching. (similar to `selecta`) [default: false]
        #[arg(short = 'f', long = "fuzzy")]
        fuzzy: Option<bool>,

        /// When searching, only look up query in 'tag' field of bookmarks. [default: false]
        #[arg(short = 't', long = "tags-only")]
        tags_only: Option<bool>,

        /// After posting a bookmark to Pinboard, update the local cache files. [default: true]
        #[arg(short = 'u', long = "auto-update")]
        auto_update: Option<bool>,

        /// When posting a new bookmark, show 3 popular tags for the URL (if available). [default: true]
        #[arg(short = 'o', long = "suggest-tags")]
        suggest_tags: Option<bool>,

        /// Check if the current browser page is already pinned. [default: false]
        #[arg(short = 'b', long = "check-bookmarked-page")]
        check_bookmarked_page: Option<bool>,

        /// Show urls (or tags) in search results' subtitle. [default: false (url)]
        #[arg(short = 'e', long = "show-urls-vs-tags")]
        show_url_vs_tags: Option<bool>,
    },

    /// Lists all bookmarks (default) or tags.
    #[command(name = "list")]
    List {
        /// Only list tags
        #[arg(short = 't', long = "tags")]
        tags: bool,
        /// Retrieve suggestion for tags from Pinboard API. Will be ignored if user is not listing
        /// tags.
        #[arg(short = 's', long = "suggest")]
        suggest: Option<bool>,
        /// Optional query word used to narrow the output list.
        /// Only works with --tags option! To narrow down bookmarks, use `search` sub-command
        query: Option<String>,
        /// Do not check if current page is bookmarked. Useful when renaming tags.
        #[arg(short = 'n', long = "no-existing-page")]
        no_existing_page: bool,
    },

    /// Creates a bookmark for the current page of the active browser.
    #[command(name = "post")]
    Post {
        /// Space-delimited list of tags for the url
        #[arg(short = 't', long = "tags")]
        tags: Vec<String>,
        /// Extra description note for the url
        #[arg(short = 'd', long = "description")]
        description: Option<String>,
        /// Mark this bookmark shared (overrides user's settings)
        #[arg(short = 's', long = "shared")]
        shared: Option<bool>,
        /// Mark this bookmark as toread (overrides user's settings)
        #[arg(short = 'b', long = "toread")]
        toread: Option<bool>,
    },

    /// Deletes a bookmark for the current page of the active browser.
    /// Or deletes a tag (see TODO item in delete.rs)
    #[command(name = "delete")]
    Delete {
        /// Url/bookmark to be deleted.
        /// If not given, the bookmark for active browser's tab will be returned.
        #[arg(short = 'u', long = "url")]
        url: Option<String>,
        /// Tag to be deleted.
        #[arg(short = 't', long = "tag")]
        tag: Option<String>,
    },

    /// Renames a tag.
    #[command(name = "rename")]
    Rename {
        /// tags for renaming (tag1 -> tag2)
        #[arg(num_args = 2)]
        tags: Vec<String>,
    },

    /// Searches bookmarks.
    #[command(name = "search")]
    Search {
        /// Only search within tags, can be combined with other flags.
        #[arg(short = 't', long = "tags")]
        tags: bool,

        /// Only search within title field, can be combined with other flags.
        #[arg(short = 'T', long = "title")]
        title: bool,

        /// Only search within description field, can be combined with other flags.
        #[arg(short = 'd', long = "description")]
        description: bool,

        /// Only search within url field, can be combined with other flags.
        #[arg(short = 'u', long = "url")]
        url: bool,

        /// Only include URLs in the output. By default search returns json suitable for Alfred.
        /// This flags only outputs the URL of results (one per line)
        #[arg(short = 'U', long = "show-only-url")]
        showonlyurl: bool,

        /// Find pins that have a tag exactly matching the given query.
        /// 'query' must be only one word.
        /// Cannot be used with other flags: -t -T -d -u
        #[arg(
            short = 'e',
            long = "exact-tag",
            conflicts_with_all = ["tags", "title", "description", "url"]
        )]
        exacttag: bool,

        /// Query string to look for in all fields of bookmarks, unless modified by -t, -T or -u
        /// flags (space delimited). Bookmarks that have all of query strings will be
        /// returned.
        #[arg(required = true)]
        query: Vec<String>,
    },

    /// Update Workflow's cache by doing a full download from Pinboard.
    #[command(name = "update")]
    Update,

    /// Check for or download the latest version of this workflow
    #[command(name = "self")]
    SelfUpdate {
        /// Check if a new version is available
        #[arg(short = 'c')]
        check: bool,

        /// Download the latest version of thir workflow and save it to its cache folder
        #[arg(short = 'd')]
        download: bool,
    },
}
