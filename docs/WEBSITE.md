# Website documentation

This document defines the first marketing website for `cdd`.

The website should feel calm, native, and developer-focused. It should explain the product quickly, show the interaction immediately, and avoid sounding like generic developer-tool marketing.

## Goal

Help visitors understand in a few seconds:

- what `cdd` is
- why it is better than normal `cd` for discovery and navigation
- how it works
- how to install it

Secondary goals:

- make the project feel real and polished before v1 ships
- give the implementation a clear visual direction
- create a clear canonical source for copy and structure

## Audience

Primary audience:

- terminal users
- developers using Zsh on macOS first
- people who already understand `cd`, `ls`, and shell workflows

Secondary audience:

- curious open-source users
- people comparing it to `fzf`, `zoxide`, `broot`, and other shell navigation tools

## Product positioning

`cdd` is not a file manager and not a full-screen fuzzy finder.

It is a more fluid way to change directories.

Core message:

> Type, filter, arrow down, enter.

Supporting message:

> Navigate directories without breaking your flow.

## Visual direction

The site should feel:

- restrained
- light
- premium
- native to the terminal/developer world
- confident, but not loud

Important notes:

- generous whitespace
- soft off-white background rather than pure white
- dark text and dark product surfaces
- one soft accent color, preferably muted blue
- rounded UI, but not bubbly
- no noisy gradients or decorative clutter
- no fake dashboard sections unrelated to the product

## Information architecture

Recommended page structure:

1. Hero
2. Why it exists
3. How it works
4. Key behaviours
5. Installation
6. Shell support and status
7. Open source / GitHub CTA
8. Footer

## Hero

The hero is the defining section of the page.

### Layout

Two-column layout on desktop.

Left side:

- top navigation
- product name `cdd`
- hero headline system
- short supporting copy
- primary and secondary CTAs
- compact metadata row

Right side:

- a dark terminal mockup showing `cdd` filtering directory names live

### Headline concept

The hero uses the product name and its expandable meaning as the main visual idea.

Structure:

- large `cdd`
- static line: `Change Directory`
- dynamic third-word ticker that rotates through the possible final `d`

Ticker options to support:

- Directly
- Dynamically
- Discovery
- Display
- Down
- Deluxe
- Differently
- Discover
- with Discovery
- Dynamic

Implementation note:

The static part should stay anchored as `Change Directory`, while the final fragment rotates or ticks.

The cleanest hero presentation is:

- `Change Directory` as the stable phrase
- the final `D` meaning shown in a vertical ticker, slot-machine list, or soft animated selector beside it

### Hero copy

Primary supporting copy:

> Type, filter, arrow down, enter.
>
> Navigate directories without breaking your flow.

### Hero CTAs

Primary CTA:

- `Install with Homebrew`

Secondary CTA:

- `Read the spec`

Optional tertiary text links:

- `View on GitHub`
- `Read the docs`

### Hero metadata row

Small muted metadata below the CTAs:

- `Open source`
- `macOS first`
- `Zsh support`
- `No telemetry`

## Navigation

Recommended top navigation:

- Docs
- GitHub
- Changelog

Optional right-side microcopy:

- `A more intentional terminal.`

## Terminal mockup

The terminal mockup should show the product immediately and without explanation.

Suggested terminal content:

```text
~/Repositories % cdd im

▸ imagekid
  image-tools
  simple-image
```

Guidelines:

- dark terminal surface
- subtle shadow
- macOS window controls are fine
- keep the mockup clean and believable
- no code noise
- make the selection state obvious
- use a monospaced font look

## Section: Why it exists

Purpose: explain the friction with normal shell navigation.

Suggested copy direction:

- `cd` works, but directory discovery is awkward
- you often need to run `ls`, scan a list, then type again
- autocomplete helps, but it does not show the shape of the directory as clearly as a live list
- `cdd` combines typing, seeing, filtering, and selecting into one flow

## Section: How it works

This section should be simple and visual.

Recommended sequence:

1. Type a path or partial directory name
2. Watch the directory list update live
3. Use arrow keys to choose
4. Press Enter to move there

Possible helper examples:

- `cdd ti`
- `cdd src/co`
- `cdd ~/Projects`
- `cdd ..`

## Section: Key behaviours

A compact features grid or short list is enough.

Suggested items:

- Live filtering while typing
- Inline results under the prompt
- Directories only
- Tab to keep drilling down
- Works with normal paths
- Clean shell integration
- Local only
- No dependency on `fzf`

## Section: Installation

Keep this direct.

Recommended content:

```sh
brew install silvandiepen/tap/cdd
```

Then:

```sh
eval "$(cdd init zsh)"
```

Note that Zsh on macOS is the first target.

## Section: Shell support and status

Recommended simple status list:

- Zsh: in progress / first-class target
- Bash: planned
- Fish: planned
- Linux: planned after macOS launch

Do not oversell unsupported platforms.

## Section: Open source

Keep this light.

Suggested points:

- public GitHub repository
- spec-first build process
- small focused utility
- no backend, no account, no telemetry

## Footer

Simple footer is enough.

Suggested content:

- `cdd`
- GitHub
- Docs
- Changelog
- License

Optional footer note:

- `Same terminal. A smoother path.`

## Motion

Motion should stay subtle.

Recommended motion opportunities:

- ticker/selector animation for the final `D`
- small caret blink in the terminal mockup
- slight focus transition on the selected result row
- soft appearance transitions when code examples change

Avoid:

- parallax
- large floating elements
- aggressive gradient animation
- fake cursor theatrics everywhere

## Content tone

Write like a developer who cares about good tools.

The copy should be:

- clear
- slightly opinionated
- concise
- human
- not overhyped

Avoid words like:

- revolutionize
- supercharge
- game-changing
- seamless experience

## Implementation notes

The website does not need to ship as a large marketing site.

A simple static site is enough for v1.

Suggested content priorities:

1. hero done well
2. install instructions
3. clear explanation
4. GitHub/docs links

## Design reference

The hero concept documented here is also captured as an SVG reference asset:

- `docs/assets/website-hero-concept.svg`

This SVG is a lightweight design reference for the structure and mood of the landing page. It is not intended to be the final production implementation.
