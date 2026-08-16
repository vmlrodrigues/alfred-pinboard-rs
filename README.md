<div align="center">
  <img src="res/workflow/icon.png" width="128" height="128" alt="" />
  <h1>Pinion</h1>
  <p>Manage, post and preview your <a href="https://pinboard.in">Pinboard</a> bookmarks from <a href="https://www.alfredapp.com">Alfred</a>.</p>

  [![Build](https://github.com/vmlrodrigues/pinion/actions/workflows/macos-universal.yml/badge.svg)](https://github.com/vmlrodrigues/pinion/actions/workflows/macos-universal.yml)
  [![Clippy](https://github.com/vmlrodrigues/pinion/actions/workflows/lint.yml/badge.svg)](https://github.com/vmlrodrigues/pinion/actions/workflows/lint.yml)
  [![Latest release](https://img.shields.io/github/v/release/vmlrodrigues/pinion?label=latest)](https://github.com/vmlrodrigues/pinion/releases/latest)
  ![Alfred](https://img.shields.io/badge/Alfred-5-blueviolet)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

  <a href="https://github.com/vmlrodrigues/pinion/releases/latest/download/Pinion.alfredworkflow">
    <img src="https://img.shields.io/badge/Download_Workflow-007AFF?style=for-the-badge&logo=apple&logoColor=white" alt="Download the workflow" height="40">
  </a>
</div>

---

> A fork of [spamwax/alfred-pinboard-rs](https://github.com/spamwax/alfred-pinboard-rs)
> by Hamid R. Ghadyani, which has not had a release since June 2023. This one is kept
> current: Helium support, the Alfred 5 settings panel, and assorted fixes. It was called
> *Rusty Pin* upstream. The [changelog](CHANGELOG.md) has the details.

## The name

**Pinion** — for three reasons, all of them true at once.

A **pinion** is the small gear that meshes with a larger one and drives it. That is the
job here: a little thing you barely notice, turning a much bigger wheel. It is also
quietly apt for something written in Rust, which is a language people reach for when they
care about the machinery.

To **pinion** something is to bind it, to hold it fast so it cannot get away. Which is
what you are doing every time you save a bookmark you would otherwise lose.

And a **pinion** is the outermost set of feathers on a bird's wing — the ones that
actually do the flying.

The word also just contains *pin*, which is the whole point.

## Setup

Download Pinion with the button above and open it to install into Alfred.

Then authenticate once. Pinion only uses Pinboard's username/token method, so you
never type your password. Get your token from
[Pinboard's settings page](https://pinboard.in/settings/password) — it looks like
`yourname:A1B2C3D4E5F6G7H8I9J0` — and give it to the `pa` keyword:

```
pa yourname:A1B2C3D4E5F6G7H8I9J0
```

Pinion keeps a local cache of your bookmarks and tags, and by default refreshes it
automatically after you post. To rebuild it by hand at any time, use `pu`.

### Where the token is kept

In your **macOS Keychain**, under the workflow's bundle id — not in the workflow's own
files, and not in Alfred's configuration panel. That means it is encrypted at rest, and
it is not carried along if you export the workflow or sync your Alfred preferences.

If you are upgrading from a version that stored it in `settings.json`, it is moved into
the Keychain the first time you use the workflow and the plaintext copy is removed. You
do not need to do anything.

To remove it entirely, delete the `pinboard-api-token` entry for Pinion in
Keychain Access, or:

```bash
security delete-generic-password -s com.victorrodrigues.pinion -a pinboard-api-token
```

## Keywords

| Keyword | What it does |
|---|---|
| `p` | Post the current browser tab as a bookmark |
| `ps` | Search your bookmarks |
| `pt` | Find a tag, then list that tag's bookmarks |
| `pr` | Rename a tag |
| `pind` | Delete the bookmark for the current browser tab |
| `pa` | Set your Pinboard API token |
| `pu` | Rebuild the local cache from Pinboard (`pupdate` does the same) |
| `pconf` | Show current settings |
| `pcheck` | Check for a newer version of the workflow |
| `pset …` | Change a setting — see [Settings](#settings) |

Two **Universal Actions** are also registered, so you can act on things elsewhere in
Alfred: *Delete Pinboard Bookmark* (on a URL) and *Rename Pinboard Tag* (on text).

## Posting a bookmark

Type `p` followed by tags. The bookmark's URL and title come from your active browser
tab, so you never type those.

```
p rust async
```

As you type, the workflow lists your existing tags with a count of how often you have
used each one. Press <kbd>Tab</kbd> to autocomplete the highlighted tag, then keep
typing to add more. Press <kbd>Return</kbd> to post.

To add a description, put it after a semicolon:

```
p rust async ; the tokio chapter is the useful part
```

If tag suggestions are enabled, three popular tags for the current page are fetched from
Pinboard and shown alongside your own. The first keystroke costs about a second while
that request runs; later keystrokes are served from cache.

If the page is already bookmarked, the workflow tells you. Note that it treats these as
three distinct bookmarks and will not warn about the overlap:

```
http://example.com/page
https://example.com/page
https://example.com/page#section
```

### Modifiers while posting

Hold a modifier to override a setting for this one bookmark, without changing your
configuration:

| Hold | Effect |
|---|---|
| <kbd>⌃</kbd> | Flip the `toread` setting |
| <kbd>⌥</kbd> | Flip the `shared` (public/private) setting |
| <kbd>⌥⌃</kbd> | Flip both |

## Searching bookmarks

```
ps rust async
```

Results are bookmarks containing **all** your keywords, matched across title, tags, URL
and extended notes, newest first. More keywords means fewer results. Which fields are
searched is configurable — see [Settings](#settings).

Each result shows the bookmark's title, with its tags underneath (or its URL, depending
on your setting). Press <kbd>Return</kbd> to open it in your browser.

### Modifiers while searching

Hold a modifier to see more about the highlighted bookmark. Press <kbd>Return</kbd>
while still holding it to copy exactly what you are looking at.

| Hold | Shows | <kbd>Return</kbd> does |
|---|---|---|
| <kbd>⌘</kbd> | The tags — or the URL, if your subtitles already show tags | Copies what is shown |
| <kbd>⌃</kbd> | The extended note | Copies the note. Nothing happens if there is no note |
| <kbd>⌥</kbd> | "Show bookmark in pinboard.in" | Opens the bookmark on Pinboard's website |
| <kbd>⌘⌥</kbd> | — | Copies the URL |
| <kbd>⇧</kbd> | **Tap** it for a Quick Look preview of the page, without opening a browser | — |
| <kbd>⌘L</kbd> | Large Type view of the title | — |

### Searching a single tag

`pt` searches your tags rather than your bookmarks. Pick one and press
<kbd>Return</kbd> to list every bookmark carrying it:

```
pt rust
```

## Deleting a bookmark

Open the bookmark in your browser, then:

```
pind
```

Or use the *Delete Pinboard Bookmark* Universal Action on any URL — including on a
result from this workflow's own search.

## Renaming a tag

```
pr oldtag
```

Pick the tag from the list, press <kbd>Return</kbd>, then type the new name. You can
also pick an existing tag as the new name, which merges the two.

The *Rename Pinboard Tag* Universal Action does the same from anywhere in Alfred.

> Pinboard's API takes up to a minute to reflect a rename, so the change will not appear
> in your cache immediately. Their API also reports success even when the old tag does
> not exist, so a typo is silently accepted.

## Settings

Settings live in Alfred's own configuration panel — **Alfred Preferences → Workflows →
Pinion → Configure Workflow** — which gives you checkboxes and fields for
everything below.

You can also change any of them without leaving Alfred's search bar. The `pset` keywords
write into that same panel:

| Command | Setting |
|---|---|
| `pset fuzzy` | Match query letters in order rather than consecutively |
| `pset suggest_tags` | Offer Pinboard's popular tags for the current page when posting |
| `pset shared` | Post new bookmarks as public rather than private |
| `pset toread` | Mark new bookmarks as unread |
| `pset check_bookmarked` | Tell me when the current page is already bookmarked |
| `pset tagonly` | Search only the tag field, not title/URL/notes |
| `pset auto` | Refresh the cache automatically after posting |
| `pset url_tag` | Show tags in search subtitles, rather than URLs |
| `pset tags 25` | How many tags to list |
| `pset bookmarks 12` | How many bookmarks to list |

Each `pset` toggle offers an explicit *Enable* / *Disable* choice rather than flipping
blindly, so you can always see the current intent before committing to it.

One setting has no `pset` keyword and lives only in the configuration panel:

- **Notify on successful post** — off by default. Posting a bookmark is silent unless
  something goes wrong; failures always notify. Turn it on if you would rather be told
  every time, which is the behaviour older versions had.

`pconf` lists every setting with its current value.

**Upgrading from 0.18.x or earlier:** your existing preferences are copied into the
configuration panel automatically the first time you use the workflow. Anything you had
already changed in the panel yourself is left alone.

Your API token is not part of any of this — see below.

### Fuzzy versus normal search

With fuzzy search **off**, `pset tgs` matches nothing, because it looks for those letters
consecutively. With fuzzy search **on**, it matches the tag `tags`, because the letters
appear in that order with gaps allowed.

## Supported browsers

Safari · Safari Technology Preview · Chrome · Chromium · Brave · Brave Beta · Brave
Nightly · Microsoft Edge · Vivaldi · Opera · Opera Beta · Opera Developer · Arc ·
Orion · Helium · Firefox · Firefox Developer Edition · qutebrowser

Firefox and qutebrowser have caveats — see below.

## Known issues

**Firefox.** Firefox does not support being driven by AppleScript the way other browsers
do, so Pinion falls back to synthesising keystrokes and reading the clipboard. With
tag suggestions or "check if bookmarked" enabled, posting from Firefox is unreliable, and
`pind` will not work while Firefox is frontmost. Installing
[alfred-firefox](https://github.com/deanishe/alfred-firefox) works around this — you do
not use that workflow directly, this one borrows one of its functions.

**qutebrowser.** Same clipboard-based approach, and it clears your clipboard as a side
effect.

**Gatekeeper.** If macOS says the developer cannot be verified, see
[upstream issue #120](https://github.com/spamwax/alfred-pinboard-rs/issues/120) and this
[Alfred forum thread](https://www.alfredforum.com/topic/13824-workflow-fail-with-developer-cannot-be-verified-errors-in-catalina/?do=findComment&comment=72101).

**Alfred 4.** This fork targets Alfred 5 and is not tested against 4. Alfred 4 users
should use upstream's [0.16.12 release](https://github.com/spamwax/alfred-pinboard-rs/releases/tag/0.16.12),
the last version in Alfred 4 format.

## Updates

Pinion checks for a newer version of itself once every 24 hours, and only when you
actually use one of its keywords — there is no background process. `pcheck` checks
immediately.

## Building from source

The toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml), so `cargo` will
fetch the right version automatically.

```bash
cargo build --release
```

The two libraries this depends on, `rusty-pin` and `alfred-rs`, are vendored under
[`vendor/`](vendor/README.md) rather than fetched from crates.io or git. Both are
unmaintained upstream, and `rusty-pin` in particular is published nowhere and had a
`master` branch incompatible with this code, so any `cargo update` used to break the
build. Vendoring makes this repository self-contained. See
[vendor/README.md](vendor/README.md) for provenance and the handful of local changes.

Releases are built by GitHub Actions from a pushed tag — see
[`.github/workflows/macos-universal.yml`](.github/workflows/macos-universal.yml).

## Feedback

[Open an issue](https://github.com/vmlrodrigues/pinion/issues).

## Credits

Originally created by [Hamid R. Ghadyani](https://github.com/spamwax) as
[alfred-pinboard-rs](https://github.com/spamwax/alfred-pinboard-rs). Almost all of the
workflow's design and implementation is his work, as are the `rusty-pin` and `alfred-rs`
libraries vendored here; this fork exists to keep it maintained and released. Thank you
for building it and open-sourcing it under MIT.

## License

[MIT](LICENSE). Copyright © 2018 Hamid R. Ghadyani, © 2026 Victor Rodrigues.
