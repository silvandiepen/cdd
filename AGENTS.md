# cdd agent notes

`cdd` is a small shell navigation tool. Read `docs/SPEC.md` before implementation. That file is the canonical product specification.

Global project conventions live in `~/Projects/Agents/` and still apply. In particular, read:

- `AGENTS.md`
- `working-principles.md`
- `task-workflow.md`
- `git-conventions.md`

## Product rule

The defining interaction is live inline directory filtering while the user is still editing a normal shell command:

```text
~/Repositories % cdd im

  imagekid
▸ image-tools
  simple-image
```

Do not simplify the product into an `fzf` wrapper or a picker that only appears after Enter. A post-Enter picker is a fallback mode, not the main experience.

## Initial platform

Build the first version for:

- macOS
- Zsh
- ZLE integration
- Rust binary
- Homebrew distribution

Do not prematurely implement Bash, Fish, Linux or Windows before the Zsh experience works properly.

## Technical boundaries

The Zsh integration owns shell-specific state and the actual directory change.

The Rust binary owns reusable logic such as:

- directory enumeration
- path parsing
- filtering and ranking
- safe result serialization
- fallback interactive mode
- shell init output

A child process cannot change its parent shell's working directory, so the final `cd` must occur in the shell adapter.

## UI constraints

The UI is deliberately restrained:

- inline only
- no alternate screen
- no full-screen TUI
- no hardcoded theme
- no decorative borders
- directories only
- maximum result list should stay compact
- selected item uses terminal-native background/inverse treatment
- clean up every rendered row on cancel, completion and error

Do not make the terminal feel like an application window.

## Dependencies

Do not use `fzf`.

Prefer a small Rust dependency graph. Avoid a full TUI framework unless direct terminal handling proves insufficient for the required inline behaviour.

No network services, telemetry, accounts or backend.

## Repository shape

Keep this a single Rust package initially.

Expected shape:

```text
src/
shell/
tests/
docs/
```

Do not introduce a monorepo/workspace until there is an actual need.

## Verification

At minimum, verify:

- exact, prefix, substring and fuzzy ranking
- path parsing for `.`, `..`, `~`, absolute and nested paths
- spaces and shell metacharacters
- Unicode paths
- hidden-directory behaviour
- symlinked directories
- no matches
- permission failures
- prompt cleanup
- ZLE widgets are active only for `cdd`
- normal Up/Down/Tab/Enter behaviour is untouched for other commands

Interactive shell behaviour should be tested in a real Zsh session in addition to unit tests.
