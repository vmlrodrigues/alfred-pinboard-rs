<div align="center">
  <img src="res/workflow/icon.png" width="160" height="160" alt="" />
  <h1>Alfred Workflow for Pinboard</h1>
  <p>Manage, post and <strong>preview</strong> your <a href="https://pinboard.in">Pinboard</a> bookmarks from <a href="https://www.alfredapp.com">Alfred</a>.</p>

  [![Build](https://github.com/vmlrodrigues/alfred-pinboard-rs/actions/workflows/macos-universal.yml/badge.svg)](https://github.com/vmlrodrigues/alfred-pinboard-rs/actions/workflows/macos-universal.yml)
  [![Clippy](https://github.com/vmlrodrigues/alfred-pinboard-rs/actions/workflows/lint.yml/badge.svg)](https://github.com/vmlrodrigues/alfred-pinboard-rs/actions/workflows/lint.yml)
  [![Latest release](https://img.shields.io/github/v/release/vmlrodrigues/alfred-pinboard-rs?label=latest)](https://github.com/vmlrodrigues/alfred-pinboard-rs/releases/latest)
  ![Alfred](https://img.shields.io/badge/Alfred-5-blueviolet)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

  <a href="https://github.com/vmlrodrigues/alfred-pinboard-rs/releases/latest/download/AlfredPinboardRust.alfredworkflow">
    <img src="https://img.shields.io/badge/Download_Workflow-007AFF?style=for-the-badge&logo=apple&logoColor=white" alt="Download the workflow" height="40">
  </a>
</div>

---

> **This is a maintained fork** of [spamwax/alfred-pinboard-rs](https://github.com/spamwax/alfred-pinboard-rs),
> originally written by Hamid R. Ghadyani, whose last release was in June 2023.
>
> Relative to upstream this fork adds Helium browser support and Alfred 5's native
> settings panel, fixes two ways the Pinboard API token could leak, repairs several
> crashes and dead browser integrations, and rebuilds the CI so releases actually
> publish. See the [changelog](CHANGELOG.md) for the detail. Issues and pull requests
> are welcome here.

Download the workflow with the button above, double-click it to install in Alfred, then
follow [Setup](#installation--setup).

## Features

Pinboard is a fast, no-nonsense bookmarking service. This workflow lets you:

- _**post**_ a bookmark to Pinboard right from Alfred, with:
  - bookmark information fetched from the active browser window
  - _tag_ auto-completion against your existing tags
  - _popular_ tags for the current URL
- _**search**_ your bookmarks
  - Tap <kbd>Shift</kbd> to preview the selected item without opening a browser
  - Tap <kbd>Command+L</kbd> for a _Large Type_ view of the title
  - Hold <kbd>Command</kbd> to show the item's _tags_, or <kbd>Control</kbd> for its
    extended note — and press <kbd>Return</kbd> while holding either to copy what you
    are looking at
- Bookmark deletion and tag renaming, as Universal Actions
- Automatic [updates](#misc) of the workflow itself
- Settings in Alfred's own configuration panel, or from the search bar with `pset`
  (see [Settings](#config))

### TL;DR

After the initial [setup](#installation--setup):

- To post, type `p` in Alfred followed by a few tags and an optional description. The
  workflow posts the active browser tab to Pinboard.
- To search, type `ps` followed by your search keywords.

### Supported browsers

Safari · Safari Technology Preview · Chrome · Chromium · Brave · Brave Beta · Brave
Nightly · Microsoft Edge · Vivaldi · Opera · Opera Beta · Opera Developer · Arc ·
Orion · Helium · Firefox _(see [known issues](#known-issues))_ ·
Firefox Developer Edition · qutebrowser _(see [known issues](#known-issues))_

## Installation / Setup

[Download the latest release](https://github.com/vmlrodrigues/alfred-pinboard-rs/releases/latest/download/AlfredPinboardRust.alfredworkflow)
and open it to install into Alfred. Then do a one-time authentication.

This workflow only uses the username/token method, so you never enter your password —
this is Pinboard's suggested way of using their API. If you don't have a token, get one
from Pinboard's [settings page](https://pinboard.in/settings/password).

Invoke Alfred and enter your `username:token` after the **`pa`** keyword:

![Authenticating with the pa keyword](./res/images/authentication.png)

The workflow keeps a local cache of your tags and bookmarks and, by default, updates it
automatically. See [manual updates](#manual-cache-update).

---

## Usage: post a bookmark

```
p tag1 tag2 tag3 ; some optional note (semi-colon & note are optional)
```

The workflow shows your existing tags as you type:

![Tag list while posting](./res/images/non-fuzzy-search-tags.png)

The number below each tag is how many times you've used it. Move Alfred's highlight to a
tag and hit <kbd>Tab</kbd> to autocomplete it. Press <kbd>Enter</kbd> to finish.

If tag suggestion is enabled (`pset suggest_tags`), three popular tags for the current
page are added to the list. These come from Pinboard's API and are often helpful, but the
first keystroke costs about a second while they are fetched. Subsequent keystrokes are not
delayed, as the result is cached.

![Popular tags](./res/images/popular-tags.png)

### Modifiers while posting

Hold a modifier to change a setting for this bookmark only:

- <kbd>Control ⌃</kbd> — toggle the `toread` setting
- <kbd>Option ⌥</kbd> — toggle the `shared` setting
- <kbd>Option ⌥</kbd> + <kbd>Control ⌃</kbd> — toggle both

To add a description, put it after a semi-colon:

![Adding a note](./res/images/adding-notes.png)

### Already-saved bookmarks

If the current page is already saved you'll be told. Note that the workflow treats these
as three different bookmarks and won't warn you about the overlap:

- `http://example.com/list.html`
- `https://example.com/list.html`
- `http://example.com/list.html#fragment`

![Already-saved notice](./res/images/already-saved.png)

---

## Usage: search bookmarks

```
ps query1 query2 query3 ...
```

Results contain **all** of your search keywords, matched across the bookmark's
description, tags, URL and extended notes. Which fields are searched is adjustable — see
[Settings](#config). So **the more** keywords you enter, **the fewer** results you get.

Results are ordered newest first.

![Search results](./res/images/bookmarks-search-results.png)

To show tags instead of URLs in the subtitles, use `pset url_tag`:

![URL versus tag subtitles](./res/images/url_vs_tag.png)

### Modifiers while searching

Hold a modifier to see more about the highlighted bookmark; press <kbd>Return ⏎</kbd>
while still holding it to copy exactly what you are looking at.

- <kbd>Command ⌘</kbd> — show the bookmark's tags, or its URL if you have subtitles set
  to show tags. <kbd>⌘⏎</kbd> copies whichever is shown.
- <kbd>Control ⌃</kbd> — show the bookmark's extended note. <kbd>⌃⏎</kbd> copies the note.
  Bookmarks without a note simply do nothing.
- <kbd>Option ⌥</kbd> + <kbd>Return ⏎</kbd> — open the bookmark on [Pinboard's website](https://pinboard.in)
- <kbd>Command ⌘</kbd> + <kbd>Option ⌥</kbd> + <kbd>Return ⏎</kbd> — copy the bookmark's URL
- <kbd>Shift ⇧</kbd> — **tap** to preview the bookmark without opening a browser 😎 ⤵︎

![Quick Look preview](./res/images/quicklook-preview.png)

---

## Usage: delete a bookmark

Use `pind` while the bookmark you want to delete is open in your active browser:

![Deleting with pind](./res/images/delete-pin.png)

Or use the `Delete Pinboard Bookmark` Universal Action on a bookmark item anywhere in this
workflow, or on any URL typed into Alfred:

![Delete Universal Action](./res/images/delete_action.gif)

The usual flow is to find the bookmark with `ps`, hit Enter to open it, then run `pind`.

## Usage: rename a tag

Use `pr` to search your tags, select one, and hit Enter. You'll be prompted for the new
name — either pick an existing tag or type a new one.

![Renaming a tag](./res/images/tag_rename.gif)

Additionally, you can **Action** a tag item anywhere in this workflow and use the
`Rename Pinboard Tag` action for the same result. Requires Alfred 4.5+.

![Rename Universal Action](./res/images/tag_rename_action.gif)

**Note:** see [known issues](#known-issues) for limitations caused by Pinboard's API.

## Settings<a name="config"></a>

Settings live in Alfred's own **workflow configuration** panel — open Alfred Preferences →
Workflows → this workflow → **Configure Workflow**, and you get checkboxes and fields for
everything below.

You can still change any of them without leaving Alfred's search bar, using the `pset`
keywords listed further down; they write straight into that same configuration panel.

If you are upgrading from 0.18.x or earlier, your existing preferences are copied into the
configuration panel automatically the first time you use the workflow. Your Pinboard token
is not affected — it stays where it was and never goes into the panel.

To review the current values, enter `pconf` in Alfred:

![Configuration list](./res/images/configuration.png)

Select a setting and hit <kbd>Enter</kbd> to adjust it:

![Adjusting the fuzzy setting](./res/images/set-fuzzy.png)

You can also set these directly:

- `pset fuzzy` — enable/disable fuzzy search
- `pset suggest_tags` — when posting, list popular tags for the active page. Fetched from
  Pinboard, and sometimes not very accurate.
- `pset shared` — mark all new bookmarks as _shared_
- `pset toread` — mark all new bookmarks as _toread_
- `pset check_bookmarked` — notify if the active page is already bookmarked
- `pset tagonly` — only search the _tag_ field
- `pset auto` — update the local cache automatically after posting
- `pset tags` — number of tags to show, e.g. `pset tags 25`
- `pset bookmarks` — number of bookmarks to show, e.g. `pset bookmarks 12`
- `pset url_tag` — show URLs or tags in search-result subtitles

### Manual cache update

Use **`pu`** to update the cache by hand:

![Updating the cache](./res/images/update.png)

This forces a full download, so it's also the way to rebuild a cache you suspect is stale
or corrupt.

---

Most settings are self-explanatory, but `fuzzy` search deserves a demo. With fuzzy search
on, tags and bookmarks containing the query letters *in order* are shown:

![Fuzzy tag search](./res/images/fuzzy-search-tags.png)

Otherwise, normal search looks for consecutive characters:

![Non-fuzzy tag search](./res/images/non-fuzzy-search-tags.png)

---

## Misc.<a name="misc"></a>

The workflow checks for a newer version of itself every 24 hours. The check only happens
when you actually use one of its keywords — no background service is ever run. You can
also check manually with `pcheck`.

![Upgrade available](./res/images/upgrade_available.png)

It tries to show helpful errors when things go wrong:

![Error example](./res/images/error-1.png)
![Error example](./res/images/error-2.png)
![Error example](./res/images/error-3.png)

---

## Known Issues<a name="known-issues"></a>

- **Firefox**: with tag suggestions and "check if page is bookmarked" enabled, posting
  from Firefox is broken, and `pind` won't work while Firefox is the active browser.
  Firefox doesn't properly support being driven programmatically. As a workaround, install
  the [alfred-firefox](https://github.com/deanishe/alfred-firefox) workflow — you won't use
  it directly, this workflow just borrows one of its functions.
- If you get `cannot be opened because the developer cannot be verified`, see
  [upstream issue #120](https://github.com/spamwax/alfred-pinboard-rs/issues/120) and this
  [Alfred forum post](https://www.alfredforum.com/topic/13824-workflow-fail-with-developer-cannot-be-verified-errors-in-catalina/?do=findComment&comment=72101).
- Renaming a tag isn't reflected in the local cache immediately — Pinboard can take up to
  a minute to update internally. Pinboard's rename API also reports `success` even when
  `old_tag` doesn't exist, so no error is surfaced.
- This workflow targets **Alfred 5**. It may work on earlier versions, but is not tested
  against them.

## Alfred 4 support

- Alfred 5 changed the internal structure of workflows, so a workflow updated in place via
  automatic update may stop working. If that happens, delete and reinstall it.
- Alfred 4 users should use upstream's
  [0.16.12 release](https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.12),
  the last version in Alfred 4 format. This fork does not target Alfred 4.

## Feedback / Bugs

[Open an issue](https://github.com/vmlrodrigues/alfred-pinboard-rs/issues) — feedback and
bug reports are welcome.

## Building from source

Requires Rust — the toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml),
so `cargo build` will fetch the right version automatically.

```bash
cargo build --release
```

The two Pinboard/Alfred libraries this depends on, `rusty-pin` and `alfred-rs`, are
vendored under [`vendor/`](vendor/README.md) rather than fetched from crates.io or git.
Both are unmaintained upstream, and `rusty-pin` in particular is not published anywhere
and had a `master` branch incompatible with this code — so any `cargo update` used to
break the build. Vendoring makes the repository self-contained. See
[vendor/README.md](vendor/README.md) for provenance and the handful of local changes.

Releases are built by GitHub Actions on a pushed tag; see
[`.github/workflows/macos-universal.yml`](.github/workflows/macos-universal.yml).

## Credits

Originally created by [Hamid R. Ghadyani](https://github.com/spamwax) as
[alfred-pinboard-rs](https://github.com/spamwax/alfred-pinboard-rs). Almost all of the
workflow's design and implementation is his work, as are the `rusty-pin` and `alfred-rs`
libraries vendored here; this fork exists to keep it maintained and released. Thank you
for building it and open-sourcing it under MIT.

## License

[MIT](LICENSE). Copyright © 2018 Hamid R. Ghadyani, © 2026 Victor Rodrigues.
