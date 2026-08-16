# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.0] - 2026-08-16
### Changed
- Replaced `structopt` with `clap` 4. `structopt` has been in maintenance mode since
  2022 (RUSTSEC-2022-0104) and pinned the tree to `clap` 2. Every flag, short, long,
  default and conflict is unchanged — the command-line surface was diffed
  subcommand-by-subcommand before and after, and all 18 invocations the workflow
  actually makes were replayed against the new binary.

### Security
- Dropped `structopt` (RUSTSEC-2022-0104) and `ansi_term` (RUSTSEC-2021-0139) from the
  dependency tree, along with `clap` 2, `textwrap`, `vec_map` and `proc-macro-error`.
- `atty` (RUSTSEC-2021-0145, RUSTSEC-2024-0375) is **not** yet gone. It no longer
  arrives through `structopt`, but `alfred-rs` 0.7.1 still depends on `env_logger` 0.9,
  which depends on it. Removing it needs a change in `alfred-rs`, which is published on
  crates.io by the original author and is not something this fork controls.

## [0.20.0] - 2026-08-16
### Added
- <kbd>⌘⏎</kbd> in search results copies whichever of the tags or URL is currently
  shown under <kbd>⌘</kbd>, and <kbd>⌃⏎</kbd> copies the bookmark's extended note.
  Both modifiers previously changed the subtitle but left <kbd>⏎</kbd> doing the same
  thing as no modifier at all, so two bindings did nothing. Nothing was taken away:
  <kbd>⌥⏎</kbd> and <kbd>⌘⌥⏎</kbd> behave exactly as before.
- A bookmark with no extended note marks the <kbd>⌃</kbd> alternate invalid, so
  <kbd>⌃⏎</kbd> does nothing instead of copying an empty string.

### Fixed
- The <kbd>⌃</kbd> alternate declared an `arg` that nothing consumed, because no
  connection carried the Control modifier. It now feeds the clipboard.
- Alfred's <kbd>⌥</kbd> route had no modifier subtext, so nothing hinted the modifier
  existed. All modifier routes out of `ps` are now labelled.

## [0.19.1] - 2026-08-16
### Fixed
- The "show URLs / show tags in search results" checkbox added in 0.19.0 was labelled
  backwards. The stored flag is named `show_url_vs_tags`, but `true` actually means
  *show tags* — so the checkbox is now labelled "Show tags in results" and matches
  what it does. Behaviour is unchanged; only the label was wrong.
- Corrected "Post all new bookmarksas as public" in the `pset shared` list.

### Changed
- The eight `pset` pickers now emit Alfred's JSON format instead of the Alfred 2-era
  XML they had used since 2018. That format is long deprecated and structurally could
  not carry modifier alternates, variables or typed icons, so this unblocks those.
  Every title, subtitle, icon and argument is unchanged.
- Those pickers no longer set `uid`, so the two choices always appear in the same
  order. Previously Alfred reordered them by how often each was picked, which makes a
  two-item toggle move under the cursor. The old values were inconsistent anyway
  (`1`/`2`, `1`/`10`, empty, and in one case the leftovers `tagonly`/`des`).
- Dropped the `autocomplete` values from those pickers. They were copy-paste noise
  ("Yes"/"No") and tab-completing a toggle to the literal text "Yes" served no purpose.

## [0.19.0] - 2026-08-16
### Added
- Settings now appear in Alfred's native **workflow configuration** panel: eight
  checkboxes and two number fields, instead of being reachable only through keywords.
  The configuration sheet is the source of truth and is passed to the workflow as
  environment variables.

### Changed
- The `pset` keywords all still work and are unchanged from a user's point of view, but
  they now write into Alfred's configuration sheet rather than into the workflow's own
  `settings.json`.
- On the first run after upgrading, existing preferences are copied from `settings.json`
  into the configuration sheet automatically. Without this an upgrade would have reset
  everyone's settings to the workflow defaults, since the sheet starts out empty.
  A value that Alfred does not supply, or that cannot be parsed, leaves the stored
  setting untouched rather than falling back to a default.
- The Pinboard API token deliberately does **not** move into the configuration sheet.
  It stays in `settings.json`, out of anything that gets exported or synced with the
  workflow.

## [0.18.2] - 2026-08-16
### Fixed
- Opera Developer and Opera Beta are detected again. Both branches were dead, for two
  independent reasons: `Opera Beta` was matched against the string `"Opera Beta.pp"`,
  missing the `a` in `.app`, and both branches asked `appIsRunning("Opera")` while the
  running processes are named `Opera Developer` and `Opera Beta`. AppleScript's list
  `contains` is exact membership rather than a substring test, so neither could ever
  match. Regular Opera was unaffected.

## [0.18.1] - 2026-08-16
### Fixed
- Helium now uses its native AppleScript dictionary (`net.imput.helium`) instead of
  reading the accessibility tree. Helium is a Chromium fork and answers the same
  `active tab of first window` as Chrome, so the previous approach was not only more
  fragile but unnecessary. It required an Accessibility permission that macOS never
  prompts for — without it Helium silently never worked, because the surrounding
  `try` swallowed the permission error — and it returned the browser *window* title
  rather than the tab title, so bookmarks saved from Helium had worse titles than
  those from any other browser.

## [0.18.0] - 2026-08-16
This is the first release from the maintained fork at
[vmlrodrigues/alfred-pinboard-rs](https://github.com/vmlrodrigues/alfred-pinboard-rs).

### Security
- The Pinboard API token is no longer printed into Alfred's debug window. It is passed as
  a token type whose `Debug` implementation redacts it, so it cannot leak by accident again.
- The API token can no longer reach the UI, logs, or disk through error messages. The
  token travels as a URL query parameter and network errors quote the full URL, so all
  error text is now redacted before it is displayed, logged, or cached.
- A failed popular-tags fetch is no longer stored as a bookmark tag. The error was written
  to the tag cache and shown as a selectable Alfred row.

### Fixed
- Self-update now targets this fork. It previously polled the upstream repository, so an
  upstream release would have replaced this workflow with theirs.
- No longer panics when a query contains non-ASCII whitespace, such as the non-breaking
  space you get when pasting from a web page.
- No longer panics when listing tags on an account that has no tags.
- A configuration save that fails now exits non-zero instead of reporting success.
- `pu` / `pupdate` now forces a full download, so a corrupt cache can actually be rebuilt.

### Changed
- `rusty-pin` is now sourced from a fork and pinned by revision, so `cargo update` can no
  longer break the build or silently pull in unreviewed upstream changes.
- Release bundles are built from the files git tracks rather than a blanket copy, dropping
  about 532 KB of orphaned images and stray files that shipped in every previous release.
- The release asset is now named `AlfredPinboardRust.alfredworkflow` without the version,
  making `releases/latest/download/…` a permanent link.
- Workflow identity (`createdby`, `webaddress`, in-Alfred readme) now points at this fork.
  The `bundleid` is deliberately unchanged so existing installs keep their cache and token.
- Rewrote the README, added a download button, and added Dependabot for CI and crates.

### Removed
- Deleted dead CI: `appveyor.yml`, `disable-travis`, `ci/`, `.circleci-disabled/`.
- Deleted `res/workflow/foo.txt` and the unused `identify-browser.applescript`.
- Removed `FUNDING.yml`, which pointed sponsorship at the original author's account.

## [0.17.2] - 2026-06-29
### Added
- Add support for Helium browser in active tab detection.
### Changed
- Replace retired `macos-11`/`macos-12` runners and archived `actions-rs/*` actions;
  update `checkout`, `upload-artifact`, `download-artifact` to v4 and `action-gh-release`
  to v2; pin `MACOSX_DEPLOYMENT_TARGET` to 11.0.
- Fix pre-existing `clippy::pedantic` and `rustfmt` errors across `src/`.

## [0.17.1] - 2023-06-22
### Added
- Add support for Arch browser
- Use newer rustc version in github's Actions.
- Update workflow's icon.

## [0.17.0] - 2022-09-10
### Added
- First release for new Alfred 5
- Add support for Orion browser.
- Update upstream (rusty-pin) to fix the permissions for tags cache file as well.
### Changed
- Add a flag to update() function to control a force update of the cache.

## [0.16.12] - 2022-07-13
### Changed
- Improve notifications messages
- Use codegen=1 option of cargo to improve lto

## [0.16.11] - 2022-07-10
### Changed
- 'pconf' can now output both json & xml
- 'pset' commands (again) use xml format

## [0.16.10] - 2022-07-10
### Fixed
- Don't print auth_token when printing Config

## [0.16.9] - 2022-07-10
### Fixed
- Workaround for #143. Alfred early access has prerelease in its version. It breaks the logic of checking for minimum Alfred version with json support. (Since SemVer doesn't like comparing non-prerelease versions with standard ones)
- CI builds will not run twice for commits with a tag. Hopefully!

## [0.16.8] - 2022-07-04
### Added
- Improve alfred_version env. variable parsing.
- Prepare the workflow for Alfred-5.0

## [0.16.6] - 2022-05-26
### Fixed
- Posting not working if user enters duplicate tags
- Normalize unicode characters before searching/comparing

## [0.16.5] - 2021-08-29
### Changed
- Workflow bundle now contains fat binaries for x86_64 and aarch64 (apple is genius, PPC to x86 to arm)
- Switch to github actions for CI automation

## [0.16.4] - 2021-08-29
### Added
- Add -e flag to search command to find pins with exact tags

## [0.16.3] - 2021-08-28
### Added
- Use --query-as-item global flag to always add an Alfred item based on user's exact entry.
- Alwyas show a tag that exactly matches user's input at the top of list.
### Fixed
- search urls in lowercase

## [0.16.0] - 2021-08-23
Bumped minor version since new fuzzy search engine may produce different search results.

### Added
- Use a new fuzzy search engine.
- Support tag renaming using a keyword.
- Add tag renaming and bookmark deletion to Universal Actions.

### Changed
- Add Urls to default search when tag_only is false
- Don't show 'you have latest version' unless user checks for update.
- Use conditional objects in workflow's canvas.

## [0.15.14] - 2020-07-19
### Added
Improve error messages dusing post/delete/search operations.

## [0.15.13] - 2020-07-19
### Fixed
- Use rusty-pin 0.5.3 to fix #78 (empty tag list on Pinboard)

## [0.15.12] - 2020-04-03
### Fixed
- Use rusty-pin 0.5.1 to fix #46 (empty tag list on Pinboard)

## [0.15.11] - 2020-03-22
### Added
- Add basic support for tag renaming.

## [0.15.10] - 2020-03-12
### Fixed
- Trying to address issue [#47](https://github.com/spamwax/alfred-pinboard-rs/issues/47) (Catalina osascript premissions)
### Added
- Suport Microsoft Edge Browser
### Changed
- Don't use `sed` hack to set username for url search on [pinboard](https://pinboard.in). A `username` environment variable is now passed to Alfred.

## [0.15.8] - 2019-08-29
### Changed
- Holding `Control`/`Option` keys while posting a bookmark will now momentarily toggle `toread`/`shared` settings. ([Closes #38](https://github.com/spamwax/alfred-pinboard-rs/issues/38)) 

## [0.15.7] - 2019-07-14
- Preserve upper/lowercase of titles/urls/description.

## [0.15.6] - 2019-07-11
### Added
- Holding CMD in search results now correctly shows either tags or URL based on users' settings.
### Fixed
- Fix appveyor CI issue with directory names.

## [0.15.4] - 2019-06-17
### Added
- Add option to either show TAGs or URLs in search results.
- Add a combo modifier for search result to copy URL to clipboard.
### Fixed
- Fix multiple issues related to release of Alfred 4
- `pcheck` should now force a network call regardless of when last update check was done.
- Fix: deleting a bookmark was not working.

## [0.14.9] - 2019-02-13
### Added
- Add settings for notifying if page is already bookmarked.

## [0.14.8] - 2019-02-13
### Fixed
- Workaround for Firefox ([Fixes #25](https://github.com/spamwax/alfred-pinboard-rs/issues/25))

## [0.14.7] - 2019-01-30
### Added
- Support [Brave Browser](brave.com)

## [0.14.6] - 2019-01-22
### Added
- Minor improvements

## [0.14.5] - 2019-01-15
### Added
- Show whether current page is already bookmarked.

## [0.14.4] - 2018-11-22
### Fixed
- Fixes issue [#21](https://github.com/spamwax/alfred-pinboard-rs/issues/21)

## [0.14.1 - 0.14.3] - 2018-08-27 - 2018-10-31
### Fixed
- Re-enable auto cache update
- Using `;` to add description was broken
- Recompile binary to fix an upstream bug

### Added
- Add Opera support

## [0.14.0] - 2018-06-04
### Added
- Workflow can notify and auto update itself.

## [0.13.3] - 2018-05-29
### Fixed
- Fixes issue [#7](https://github.com/spamwax/alfred-pinboard-rs/issues/7)

<!-- Releases from 0.17.2 onward are published by this fork and tagged vX.Y.Z.
     Earlier releases live in the upstream repository and are tagged bare. -->
[Unreleased]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.21.0...HEAD
[0.21.0]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.20.0...v0.21.0
[0.20.0]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.19.1...v0.20.0
[0.19.1]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.19.0...v0.19.1
[0.19.0]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.18.2...v0.19.0
[0.18.2]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.18.1...v0.18.2
[0.18.1]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.18.0...v0.18.1
[0.18.0]: https://github.com/vmlrodrigues/alfred-pinboard-rs/compare/v0.17.2...v0.18.0
[0.17.2]: https://github.com/vmlrodrigues/alfred-pinboard-rs/releases/tag/v0.17.2
[0.17.1]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.17.1
[0.17.0]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.17.0
[0.16.12]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.12
[0.16.11]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.11
[0.16.10]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.10
[0.16.9]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.9
[0.16.8]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.8
[0.16.6]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.6
[0.16.5]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.5
[0.16.4]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.4
[0.16.3]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.3
[0.16.0]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.0
[0.15.14]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.14
[0.15.13]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.13
[0.15.12]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.12
[0.15.11]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.11
[0.15.10]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.10
[0.15.8]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.8
[0.15.7]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.7
[0.15.6]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.6
[0.15.4]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.15.4
[0.14.9]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.9
[0.14.8]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.8
[0.14.7]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.7
[0.14.6]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.6
[0.14.5]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.5
[0.14.4]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.4
[0.14.0]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.14.0
[0.13.3]: https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.13.3
