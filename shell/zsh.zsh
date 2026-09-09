# cdd — inline directory navigation for zsh
#
#   eval "$(cdd init zsh)"
#
# The preview is rendered through ZLE's own POSTDISPLAY rather than by writing
# to the terminal directly. That is what keeps it inline, off the alternate
# screen, independent of the prompt in use, and free of stale rows: ZLE already
# knows how to erase everything it drew.

typeset -g  CDD_BIN=${CDD_BIN:-@CDD_BIN@}
typeset -gi CDD_MAX_ROWS=${CDD_MAX_ROWS:-8}

# Parsed from the command buffer.
typeset -g  _CDD_PREFIX=''          # everything up to the argument, e.g. 'cdd '
typeset -g  _CDD_ARG=''             # the argument, exactly as typed

# The current result set.
typeset -ga _CDD_PATHS=()
typeset -ga _CDD_DISPLAYS=()
typeset -ga _CDD_INSERTS=()
typeset -g  _CDD_EXACT=''
typeset -gi _CDD_MORE=0
typeset -g  _CDD_ERR=''

# Interaction state.
typeset -gi _CDD_SEL=1              # 1-based index into _CDD_PATHS
typeset -gi _CDD_MOVED=0            # has the user moved the selection themselves?
typeset -gi _CDD_ACTIVE=0
typeset -g  _CDD_LAST=''            # cache key of the last resolved query
typeset -gr _CDD_MEMO='cdd'         # tags our region_highlight entry as ours
typeset -g  _CDD_POST=''            # the POSTDISPLAY we set, so we only clear our own
typeset -gA _CDD_ORIG=()            # key bindings we took over

typeset -gi _CDD_SERVER=0

#
# Talking to the binary
#

# Escape a field for the request line: tabs separate fields and newlines
# separate records, so both have to survive as text.
_cdd_escape() {
  local s=$1
  s=${s//\\/\\\\}
  s=${s//$'\t'/\\t}
  s=${s//$'\n'/\\n}
  REPLY=$s
}

# Keep one `cdd serve` process per shell so the directory listing stays cached
# between keystrokes. Purely an optimisation — every failure path falls back to
# a one-shot `cdd list`.
_cdd_server_start() {
  (( _CDD_SERVER )) && return 0
  [[ -n ${CDD_NO_SERVER-} ]] && return 1

  setopt local_options no_monitor no_notify
  coproc ${CDD_BIN} serve 2>/dev/null || return 1
  _CDD_SERVER=1
  return 0
}

_cdd_server_stop() {
  (( _CDD_SERVER )) || return 0
  _CDD_SERVER=0
  # Closing the coprocess pipe lets the binary exit on its own.
  exec {COPROC[1]}>&- 2>/dev/null
}

# Fill `reply` with the response lines, or return non-zero.
_cdd_ask_server() {
  _cdd_server_start || return 1

  local REPLY cwd arg
  _cdd_escape "$PWD"; cwd=$REPLY
  _cdd_escape "$_CDD_ARG"; arg=$REPLY

  local t=$'\t'
  print -p -r -- "LIST$t$cwd$t$arg$t${COLUMNS:-80}$t$CDD_MAX_ROWS" 2>/dev/null || {
    _cdd_server_stop
    return 1
  }

  reply=()
  local line
  while true; do
    # A hung helper must never hang the shell; time out and fall back instead.
    if ! read -r -p -t 2 line; then
      _cdd_server_stop
      return 1
    fi
    [[ $line == END ]] && break
    reply+=( "$line" )
  done
  return 0
}

_cdd_ask_binary() {
  local out
  out=$(${CDD_BIN} list --cwd "$PWD" --query "$_CDD_ARG" \
        --width ${COLUMNS:-80} --rows $CDD_MAX_ROWS 2>/dev/null) || return 1
  reply=( ${(f)out} )
  return 0
}

# Parse a response into the result-set variables.
_cdd_parse() {
  _CDD_PATHS=(); _CDD_DISPLAYS=(); _CDD_INSERTS=()
  _CDD_EXACT=''; _CDD_MORE=0; _CDD_ERR=''

  local line
  local -a f
  for line in "$@"; do
    f=( "${(@ps:\t:)line}" )
    case $f[1] in
      R)
        # Paths are zsh-quoted by the binary, so ${(Q)} gives back the exact
        # name even when it contains spaces, quotes or newlines.
        _CDD_PATHS+=( "${(Q)f[2]}" )
        _CDD_INSERTS+=( "${(Q)f[3]}" )
        _CDD_DISPLAYS+=( "$f[4]" )
        ;;
      EXACT) _CDD_EXACT="${(Q)f[2]}" ;;
      MORE)  _CDD_MORE=$f[2] ;;
      ERR)   _CDD_ERR=$f[2] ;;
    esac
  done
}

#
# Reading the command line
#

# Recognise a `cdd` invocation and split it into prefix and argument.
_cdd_scan() {
  setopt local_options extended_glob

  [[ $BUFFER == (#b)([[:blank:]]#)cdd(|[[:blank:]]*) ]] || return 1

  local lead=$match[1] rest=$match[2]
  if [[ -z $rest ]]; then
    _CDD_PREFIX="${lead}cdd "
    _CDD_ARG=''
  else
    [[ $rest == (#b)([[:blank:]]##)(*) ]] || return 1
    _CDD_PREFIX="${lead}cdd${match[1]}"
    _CDD_ARG=$match[2]
  fi
  return 0
}

_cdd_mode() {
  (( _CDD_ACTIVE )) && _cdd_scan
}

#
# Rendering
#

# Drop our highlight, leaving every other plugin's entries alone.
#
# It has to be found by its memo tag rather than by value: zsh shifts the
# offsets of region_highlight entries as the buffer changes, so the string we
# appended is not the string we would read back a keystroke later.
_cdd_unhighlight() {
  region_highlight=( "${(@)region_highlight:#*memo=$_CDD_MEMO*}" )
}

# Remove the preview, but only if it is still ours — another plugin may own
# POSTDISPLAY (zsh-autosuggestions does) once we are out of cdd mode.
_cdd_clear() {
  _cdd_unhighlight
  if [[ -n $_CDD_POST && $POSTDISPLAY == $_CDD_POST ]]; then
    POSTDISPLAY=''
  fi
  _CDD_POST=''
  _CDD_ACTIVE=0
  _CDD_LAST=''
  _CDD_PATHS=(); _CDD_DISPLAYS=(); _CDD_INSERTS=()
  _CDD_EXACT=''; _CDD_MORE=0; _CDD_ERR=''
  _CDD_SEL=1; _CDD_MOVED=0
}

_cdd_render() {
  local -a rows
  local -i i

  if [[ -n $_CDD_ERR ]]; then
    rows=( "  $_CDD_ERR" )
  elif (( ${#_CDD_DISPLAYS} == 0 )); then
    rows=( "  No matching directories" )
  else
    for (( i = 1; i <= ${#_CDD_DISPLAYS}; i++ )); do
      if (( i == _CDD_SEL )); then
        rows+=( "▸ $_CDD_DISPLAYS[i]" )
      else
        rows+=( "  $_CDD_DISPLAYS[i]" )
      fi
    done
    (( _CDD_MORE > 0 )) && rows+=( "  + $_CDD_MORE more" )
  fi

  _cdd_unhighlight
  _CDD_POST=$'\n\n'${(pj:\n:)rows}
  POSTDISPLAY=$_CDD_POST

  # Mark the selection with the terminal's own inverse video rather than a
  # colour, so it fits whatever theme the user already has.
  if (( ${#_CDD_DISPLAYS} > 0 )); then
    local -i start=$(( ${#BUFFER} + 2 ))
    for (( i = 1; i < _CDD_SEL; i++ )); do
      (( start += ${#rows[i]} + 1 ))
    done
    region_highlight+=( "$start $(( start + ${#rows[_CDD_SEL]} )) standout memo=$_CDD_MEMO" )
  fi
}

_cdd_update() {
  if ! _cdd_scan; then
    _cdd_clear
    return
  fi

  # Re-query only when the answer could have changed.
  local key="$PWD"$'\x01'"$_CDD_ARG"$'\x01'"${COLUMNS:-80}"
  if [[ $key != $_CDD_LAST ]]; then
    local -a reply
    if _cdd_ask_server || _cdd_ask_binary; then
      _cdd_parse "${(@)reply}"
      _CDD_LAST=$key
      _CDD_SEL=1
      _CDD_MOVED=0
    else
      _cdd_clear
      return
    fi
  fi

  _CDD_ACTIVE=1
  _cdd_render
}

#
# Widgets
#

# Hand a key back to whatever was bound to it before cdd was installed.
_cdd_passthrough() {
  local widget=${_CDD_ORIG[$1]:-}
  if [[ -n $widget ]] && (( ${+widgets[$widget]} )); then
    zle $widget -- "${@:3}"
  else
    zle $2 -- "${@:3}"
  fi
}

_cdd_down_widget() {
  if _cdd_mode; then
    (( _CDD_SEL < ${#_CDD_PATHS} )) && (( _CDD_SEL++ ))
    _CDD_MOVED=1
    _cdd_render
  else
    _cdd_passthrough down .down-line-or-history "$@"
  fi
}

_cdd_up_widget() {
  if _cdd_mode; then
    (( _CDD_SEL > 1 )) && (( _CDD_SEL-- ))
    _CDD_MOVED=1
    _cdd_render
  else
    _cdd_passthrough up .up-line-or-history "$@"
  fi
}

_cdd_complete_widget() {
  if ! _cdd_mode || (( ${#_CDD_INSERTS} == 0 )); then
    _cdd_passthrough complete .expand-or-complete "$@"
    return
  fi

  BUFFER="${_CDD_PREFIX}${_CDD_INSERTS[$_CDD_SEL]}"
  CURSOR=${#BUFFER}
  _CDD_LAST=''          # the base directory changed; re-query on redraw
  _CDD_SEL=1
  _CDD_MOVED=0
}

_cdd_accept_widget() {
  if ! _cdd_mode; then
    _cdd_passthrough accept .accept-line "$@"
    return
  fi

  # Resolution order: an explicit selection, then the path as literally typed,
  # then the best match. Never anything else.
  local target=''
  if (( _CDD_MOVED && ${#_CDD_PATHS} > 0 )); then
    target=${_CDD_PATHS[$_CDD_SEL]}
  elif [[ -n $_CDD_EXACT ]]; then
    target=$_CDD_EXACT
  elif (( ${#_CDD_PATHS} > 0 )); then
    target=${_CDD_PATHS[1]}
  fi

  if [[ -z $target ]]; then
    zle -M "cdd: no matching directory: $_CDD_ARG"
    return
  fi

  if ! builtin cd -- "$target" 2>/dev/null; then
    if [[ -d $target ]]; then
      zle -M "cdd: permission denied: ${target:t}"
    else
      zle -M "cdd: no such directory: ${target:t}"
    fi
    return
  fi

  print -s -- "${_CDD_PREFIX}${(q-)target}"
  _cdd_clear
  BUFFER=''
  CURSOR=0
  zle .accept-line
}

#
# Hooks
#

_cdd_line_pre_redraw_widget() {
  # Run first so plugins that rewrite region_highlight or POSTDISPLAY (syntax
  # highlighting, autosuggestions) do their work before we add ours on top.
  (( ${+widgets[_cdd_orig_line_pre_redraw]} )) && zle _cdd_orig_line_pre_redraw -- "$@"
  _cdd_update
}

_cdd_line_init_widget() {
  _CDD_LAST=''
  _CDD_POST=''
  _CDD_ACTIVE=0
  _CDD_SEL=1
  _CDD_MOVED=0
  (( ${+widgets[_cdd_orig_line_init]} )) && zle _cdd_orig_line_init -- "$@"
}

# The last chance to take the preview back out of the display before zsh
# finalises an accepted line.
_cdd_line_finish_widget() {
  _cdd_clear
  (( ${+widgets[_cdd_orig_line_finish]} )) && zle _cdd_orig_line_finish -- "$@"
}

#
# Installation
#

# Add a hook handler without displacing anyone else's.
#
# zsh ships `add-zle-hook-widget` for precisely this: several plugins can hold
# the same hook, and each runs in registration order. Registering last means
# the preview is composed after syntax highlighting and autosuggestions have
# had their turn.
_cdd_install_hook() {
  local hook=$1 ours=$2

  if (( ${+functions[add-zle-hook-widget]} )); then
    add-zle-hook-widget "${hook#zle-}" "$ours"
    return
  fi

  # Without the helper, chain by hand. `zle -A` cannot alias a special widget,
  # so the existing implementation is re-registered under a name of our own.
  case ${widgets[$hook]-} in
    user:_cdd_*) return 0 ;;                       # already installed
    user:*) zle -N "_cdd_orig_${${hook#zle-}//-/_}" "${widgets[$hook]#user:}" ;;
  esac
  zle -N "$hook" "$ours"
}

# Remember what a key did before we take it over, so non-cdd lines keep the
# user's own bindings — including plugin widgets like history-substring-search.
_cdd_capture() {
  local name=$1 seq=$2 line
  [[ -n ${_CDD_ORIG[$name]-} ]] && return 0
  line=$(bindkey -M main -- "$seq" 2>/dev/null) || return 0
  local widget=${line##* }
  [[ -n $widget && $widget != undefined-key ]] && _CDD_ORIG[$name]=$widget
  return 0
}

_cdd_install() {
  autoload -Uz add-zle-hook-widget 2>/dev/null

  zle -N _cdd_down_widget
  zle -N _cdd_up_widget
  zle -N _cdd_complete_widget
  zle -N _cdd_accept_widget

  local seq
  for seq in '^[[B' '^[OB' "${terminfo[kcud1]-}"; do
    [[ -n $seq ]] || continue
    _cdd_capture down "$seq"
    bindkey -M main -- "$seq" _cdd_down_widget
  done
  for seq in '^[[A' '^[OA' "${terminfo[kcuu1]-}"; do
    [[ -n $seq ]] || continue
    _cdd_capture up "$seq"
    bindkey -M main -- "$seq" _cdd_up_widget
  done

  _cdd_capture complete '^I'
  bindkey -M main -- '^I' _cdd_complete_widget

  _cdd_capture accept '^M'
  bindkey -M main -- '^M' _cdd_accept_widget
  _cdd_capture accept '^J'
  bindkey -M main -- '^J' _cdd_accept_widget

  _cdd_install_hook zle-line-pre-redraw _cdd_line_pre_redraw_widget
  _cdd_install_hook zle-line-init       _cdd_line_init_widget
  _cdd_install_hook zle-line-finish     _cdd_line_finish_widget
}

# Ctrl+C never reaches us as a key: the terminal still has ISIG enabled while
# ZLE is reading, so it raises SIGINT and zsh abandons the line without running
# zle-line-finish. ZLE tears down its own display, preview included, but our
# bookkeeping would survive into the next line — so reset it here.
TRAPINT() {
  _CDD_POST=''
  _CDD_ACTIVE=0
  _CDD_LAST=''
  return $(( 128 + $1 ))
}

# The command itself, for when the line was executed rather than previewed:
# a directory that already resolves is entered straight away, anything else
# opens the inline fallback picker.
cdd() {
  emulate -L zsh
  setopt local_options no_nomatch

  local arg="$*"

  if [[ -n $arg ]]; then
    local direct=${~arg}
    if [[ -d $direct ]]; then
      builtin cd -- "$direct"
      return $?
    fi
  fi

  local chosen
  chosen=$(${CDD_BIN} pick ${arg:+"$arg"}) || return 1
  [[ -n $chosen ]] || return 1
  builtin cd -- "${(Q)chosen}"
}

_cdd_install
