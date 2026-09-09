# cdd

`cdd` is an interactive alternative to `cd`.

It behaves like normal shell navigation, but while you type it shows matching directories directly below the prompt. Keep typing to narrow the list, use the arrow keys to select a directory, then press Enter to move there.

```text
~/Repositories % cdd im

  imagekid
▸ image-tools
  simple-image
```

No separate fuzzy-finder screen. No full-screen interface. No need to run `ls`, copy a directory name, or tab through a long completion list.

## The idea

Normal navigation often looks like this:

```sh
ls
cd some-directory
```

or:

```sh
cd som<Tab>
```

`cdd` combines those steps.

```text
~/Projects % cdd ti

▸ tiko
  tiko-media
  tiko-talk
  tiko-plans
```

As the query changes, the directory list changes with it.

### Controls

| Input | Behaviour |
| --- | --- |
| Type | Filter directories |
| `↑` / `↓` | Move through matches |
| `Enter` | Change into the selected directory |
| `Tab` | Complete the selected directory and keep navigating |
| `Backspace` | Broaden the results again |
| `Ctrl+C` | Cancel and return to a clean prompt |

The first result is selected by default, so when the first match is correct you can simply type and press Enter.

## Paths still behave like paths

`cdd` is not a replacement syntax for paths.

```sh
cdd ..
cdd ../Projects
cdd ~/Repositories
cdd src/components
```

For a partial final path segment, `cdd` lists and filters the directories inside the resolved parent:

```text
~/app % cdd src/co

▸ components
  composables
  config
```

Pressing `Tab` on `components` turns the input into:

```text
~/app % cdd src/components/
```

and immediately shows that directory's children.

## How it works

The live experience happens while the shell is editing the command line. That means `cdd` has two parts:

1. A small Rust binary for directory discovery, matching and shared terminal behaviour.
2. A shell integration that hooks into the shell's line editor.

For Zsh, the integration uses ZLE so `cdd` can update the results before Enter is pressed and temporarily use `↑`, `↓`, `Tab` and `Enter` for navigation.

The binary also provides a fallback interactive picker for environments where live shell integration is unavailable.

## Terminal support

`cdd` is terminal-emulator agnostic. It should work in normal ANSI-compatible terminals such as:

- Terminal.app
- iTerm2
- Ghostty
- Kitty
- WezTerm
- Alacritty

The live integration depends on the shell, not the terminal.

Initial target:

| Shell | Support |
| --- | --- |
| Zsh | Primary, live inline integration |
| Bash | Planned |
| Fish | Planned |

macOS with Zsh is the first supported environment. Linux support should follow without changing the product model.

## Installation

The intended installation is Homebrew:

```sh
brew install silvandiepen/tap/cdd
```

Then enable the shell integration:

```sh
eval "$(cdd init zsh)"
```

Add that line to `~/.zshrc` to enable `cdd` in future sessions.

The exact Homebrew formula and release process will be added when the first binary is ready.

## Design principles

`cdd` should feel like a small extension of the shell, not another terminal application.

It should:

- stay inline under the current prompt
- never use the alternate screen
- use the terminal's existing colours by default
- render only directories
- react immediately while typing
- leave the prompt completely clean when it exits
- support spaces, Unicode, symlinks and normal shell paths
- require no network connection
- have no telemetry
- have no dependency on `fzf`

## Scope

Version 1 is intentionally small.

It is a better way to change directories. It is not a file manager, command launcher, shell replacement or general-purpose fuzzy finder.

See [`docs/SPEC.md`](docs/SPEC.md) for the canonical behaviour and implementation requirements.

## Website

The initial landing-page direction is documented in:

- [`docs/WEBSITE.md`](docs/WEBSITE.md)
- [`docs/website-copy.md`](docs/website-copy.md)
- [`docs/assets/website-hero-concept.svg`](docs/assets/website-hero-concept.svg)
