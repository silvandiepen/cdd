# cdd specification

This document is the canonical product and implementation specification for `cdd`.

## 1. Product definition

`cdd` is an interactive directory-change command for shells.

The core interaction is:

```text
~/Repositories % cdd ti

▸ tiko
  tiko-media
  tiko-talk
  tiko-plans
```

The results are visible while the user is still editing the command line. Each additional character narrows or reorders the directory matches. The user can continue typing, move through the results with the arrow keys, or press Enter to change directory.

The product should feel like `cd` gained an inline directory browser.

## 2. Core requirements

### 2.1 Live while typing

The result list MUST update before the user presses Enter.

This is the main product distinction. Implementing only an interactive picker that starts after `cdd` is executed is useful as a fallback, but it is not the primary `cdd` experience.

For Zsh, live mode MUST integrate with ZLE.

### 2.2 Inline rendering

Results MUST render directly below the active shell prompt.

`cdd` MUST NOT:

- switch to the terminal alternate screen
- open a full-screen TUI
- clear the existing terminal
- create a modal fuzzy-finder experience
- leave stale result rows behind after the command completes or is cancelled

The shell should look normal again immediately after leaving `cdd`.

### 2.3 Directories only

The default result set contains directories only.

Regular files are outside the scope of `cdd`.

A symbolic link whose target is a directory SHOULD be treated as navigable.

### 2.4 Normal path semantics

`cdd` should preserve the mental model of `cd`.

These inputs must work:

```sh
cdd .
cdd ..
cdd ../..
cdd ~/Projects
cdd /Users/example
cdd src/components
```

Environment and shell expansion should remain the shell's responsibility where possible.

The final incomplete path segment is treated as the filter query. All complete path segments before it define the directory being browsed.

Example:

```text
cdd src/co
```

resolves `src/` as the base directory and filters its child directories using `co`.

### 2.5 Keyboard interaction

When the command buffer is in `cdd` mode:

| Key | Required behaviour |
| --- | --- |
| Printable characters | Update the query and matches |
| `Backspace` | Update the query and matches |
| `↓` | Select the next visible match |
| `↑` | Select the previous visible match |
| `Enter` | Change into the selected match |
| `Tab` | Complete the selected path segment without changing directory |
| `Ctrl+C` | Cancel, clear the preview and return to a clean prompt |

The first visible result is selected by default.

Selection wraps only if testing shows that wrapping feels natural. Default implementation should stop at the first and last rows.

Arrow keys MUST retain normal shell behaviour when the current command is not `cdd`.

### 2.6 Enter behaviour

Enter uses this resolution order:

1. If a result is actively selected, enter it.
2. If the typed path exactly resolves to an existing directory, enter it.
3. If the query has matches and no explicit selection has been moved, enter the first result.
4. Otherwise report that no matching directory exists and remain in the current working directory.

`cdd` MUST never silently change to an unrelated directory.

### 2.7 Tab behaviour

Tab accepts the selected directory into the current command buffer and appends `/`.

Example:

```text
~/app % cdd src/co

▸ components
  composables
```

After Tab:

```text
~/app % cdd src/components/

  Button
  Header
  Modal
```

This allows fast multi-level browsing without leaving the prompt.

## 3. Matching

Matching should be predictable rather than clever.

Ranking order:

1. exact name
2. case-sensitive prefix
3. case-insensitive prefix
4. case-sensitive substring
5. case-insensitive substring
6. ordered fuzzy match

The implementation MAY adjust scoring after real-world testing, but an exact or prefix match must never rank below a weaker fuzzy match.

Every additional query character should normally reduce or preserve the candidate set. Fuzzy scoring must not make results appear unrelated to the typed characters.

### Hidden directories

Directories beginning with `.` are hidden when the current path segment does not begin with `.`.

Typing `.` as the first character of the segment makes hidden directories eligible.

This matches normal shell expectations without adding another control.

## 4. Result rendering

### 4.1 Rows

Default maximum visible results: 8.

If there are more matches than fit:

```text
▸ components
  composables
  config
  content
  controllers
  core
  commands
  common
  + 12 more
```

The selected row should be visually distinct using terminal-native inverse/background treatment. Do not depend on a hardcoded colour scheme.

### 4.2 Width

Long directory names should be truncated to the available terminal width without corrupting Unicode.

The full path remains the value used for navigation.

### 4.3 Empty state

When no directory matches:

```text
  No matching directories
```

Keep this visually quiet. No error styling is necessary while the user is still typing.

### 4.4 Errors

Filesystem errors should be concise and actionable.

Examples:

```text
  Permission denied
```

or after Enter:

```text
cdd: permission denied: private
```

The current directory must remain unchanged when navigation fails.

## 5. Architecture

### 5.1 Components

`cdd` consists of:

```text
Zsh line editor
      │
      │ query / cwd
      ▼
shell integration
      │
      ▼
   cdd binary
      │
      ├── filesystem discovery
      ├── path parsing
      ├── filtering / ranking
      └── terminal-safe output
```

The shell integration owns behaviours that must occur inside the parent shell:

- reading the current command buffer
- reacting while the command is being edited
- temporarily overriding ZLE key behaviour
- changing the shell's working directory
- refreshing the prompt

The Rust binary owns logic that can remain shell-independent:

- directory enumeration
- path resolution helpers
- matching and ranking
- result serialization
- fallback interactive mode
- `cdd init <shell>` output

### 5.2 Why shell integration is required

A child process cannot change the working directory of its parent shell.

More importantly, a normal executable does not receive the shell command buffer while the user is still typing it.

Therefore the primary live experience cannot be implemented as only a standalone binary.

For Zsh, `cdd init zsh` should output the integration script needed to register ZLE hooks and widgets.

### 5.3 Binary interface

The internal CLI should expose stable primitives for shell adapters.

Proposed commands:

```sh
cdd init zsh
cdd list --cwd "$PWD" --query "src/co"
cdd pick
cdd --version
cdd --help
```

`cdd list` should support a machine-readable output format so the shell plugin never has to reproduce filesystem ranking logic.

The exact wire format is an implementation decision. It should be simple, versionable and safe for arbitrary Unicode path names. Newline-delimited raw paths are not sufficient because path names may contain newline characters.

### 5.4 Changing directory

The final `cd` MUST happen in the current shell process.

The Zsh adapter should call Zsh's builtin `cd` rather than spawning another shell.

All selected paths must be quoted/escaped correctly. Directory names containing spaces, quotes, brackets, emoji or shell metacharacters must work.

## 6. Zsh integration

Zsh is the first implementation target.

The adapter should use ZLE hooks/widgets and only activate when the command buffer is recognized as a `cdd` invocation.

It must not globally replace normal shell editing behaviour.

Expected responsibilities:

- detect whether the current buffer is `cdd` or begins with `cdd `
- extract the editable path/query portion
- request ranked directory matches
- render the preview below the prompt
- track selected result index
- intercept Up/Down/Tab/Enter only while `cdd` mode is active
- clear preview rows before each redraw
- restore the original widgets when outside `cdd`
- preserve a clean prompt after cancellation or navigation

The implementation must coexist with common Zsh setups as far as practical, including Oh My Zsh and Starship. It should avoid assuming a specific prompt.

## 7. Fallback interactive mode

Running the binary without live shell integration should still be useful.

After:

```sh
cdd
```

the fallback can open an inline interactive picker that starts from the current directory.

This mode may receive keystrokes itself because the command has already been executed.

It must still:

- remain inline
- avoid the alternate screen
- list directories only
- support typing, arrows, Enter and cancellation

Because a binary cannot change its parent shell directory, fallback installation still needs a small shell function that captures the chosen path and calls the shell builtin `cd`.

The UI should write to `/dev/tty` or stderr while the selected path is returned through a dedicated machine-readable channel.

## 8. Shell support

### Version 1

- macOS
- Zsh
- live inline integration
- Homebrew installation

### Next

- Bash
- Fish
- Linux

The architecture should make shell adapters small, but do not create abstractions for shells that are not being implemented yet.

Windows shells are out of scope for the initial project.

## 9. Installation and distribution

Primary distribution target:

```sh
brew install silvandiepen/tap/cdd
```

Shell setup:

```sh
eval "$(cdd init zsh)"
```

Users add the init line to `~/.zshrc`.

The binary should not modify shell configuration files automatically unless an explicit installer command is added later. Installation must remain understandable and reversible.

Future releases may also provide:

- GitHub release binaries
- Cargo installation if useful
- packages for other Unix package managers

## 10. Performance

The preview should feel instantaneous.

Requirements:

- no network access
- no recursive filesystem scan
- enumerate only the directory represented by the current completed path
- avoid re-reading the filesystem when only the filter text changed
- cache the current base directory listing for the duration of the interaction
- discard or refresh the cache when the browsed base directory changes

Large directories should not block shell input noticeably.

Filtering and rendering should be performed without spawning one process per candidate.

## 11. Filesystem behaviour

### Symlinks

Symlinks resolving to directories are navigable.

Do not recursively resolve or scan symlink trees.

### Permissions

Unreadable directories may still appear if the parent directory exposes them. Navigation failure should be reported when selected.

### Unicode

Paths must be treated as OS-native paths. Do not assume valid ASCII.

Rendering logic must not split a displayed Unicode grapheme in the middle when truncating.

### Special names

Spaces and shell metacharacters are fully supported.

Newlines in filenames must not corrupt the binary-to-shell protocol.

## 12. Configuration

Version 1 should work without a config file.

Do not add themes, plugins or a large preference surface before there is a real need.

Reasonable environment variables may be introduced for small behaviour changes, for example maximum result rows, but defaults should be sufficient for nearly everyone.

## 13. Privacy and dependencies

`cdd` is local-only.

It must have:

- no telemetry
- no analytics
- no account
- no backend
- no network requirement

`cdd` should not depend on `fzf`.

Prefer a small dependency graph. A direct terminal library such as `crossterm` is a better fit than a full-screen TUI framework if it keeps inline rendering straightforward.

## 14. Non-goals

Version 1 is not:

- a replacement for `ls`
- a general file browser
- a file picker
- a command palette
- a shell history tool
- a frecency database
- a bookmarks manager
- a full-screen TUI
- an `fzf` wrapper
- a replacement shell

These may be interesting adjacent ideas, but they should not dilute the primary interaction.

## 15. Suggested repository structure

```text
cdd/
  AGENTS.md
  README.md
  Cargo.toml
  src/
    main.rs
    cli.rs
    filesystem.rs
    matcher.rs
    path.rs
    output.rs
    picker.rs
  shell/
    zsh.zsh
  tests/
  docs/
    SPEC.md
```

Keep the Rust core single-package until the project has a concrete reason to split into a workspace.

## 16. Acceptance criteria for v0.1

A v0.1 build is successful when, on macOS with Zsh:

1. Installing the binary and evaluating `cdd init zsh` enables live `cdd` mode.
2. Typing `cdd` shows child directories below the current prompt before Enter is pressed.
3. Typing characters updates the visible matches.
4. `↑` and `↓` change the selected directory without invoking shell history.
5. Enter changes the current shell directory to the selected item.
6. Tab completes the highlighted directory and allows continued nested navigation.
7. `cdd ..`, absolute paths, `~` paths and multi-segment paths work.
8. Spaces and Unicode directory names work.
9. Hidden directories appear when the active segment begins with `.`.
10. Cancelling leaves no stale preview rows.
11. Commands other than `cdd` retain normal Zsh behaviour.
12. The implementation performs no network requests and has no telemetry.
