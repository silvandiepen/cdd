#!/usr/bin/env zsh
#
# Interactive acceptance tests for the zsh integration.
#
# These drive a real zsh line editor over a pty, because the whole point of
# `cdd` is what happens before Enter is pressed — and that cannot be observed
# by running the binary.
#
#   tests/zle.zsh <path-to-cdd-binary> <scratch-directory>
#
# Assertions run against the full screen, obtained by asking zle to redraw
# (^L): the pty otherwise carries only incremental updates, which say nothing
# about what is actually on screen.

emulate -L zsh
setopt extended_glob

zmodload zsh/zpty
zmodload zsh/datetime

BIN=${1:?usage: zle.zsh <binary> <scratch dir>}
ROOT=${2:?usage: zle.zsh <binary> <scratch dir>}

#
# Fixture
#

rm -rf -- $ROOT
mkdir -p -- \
  $ROOT/components $ROOT/composables $ROOT/config $ROOT/core \
  $ROOT/.secret $ROOT/'my dir' $ROOT/プロジェクト \
  $ROOT/src/alpha/inner $ROOT/src/beta \
  $ROOT/locked/child $ROOT/many
for i in {1..12}; do mkdir -p -- $ROOT/many/d$i; done
ln -sfn $ROOT/src/alpha $ROOT/linked
: > $ROOT/notadir.txt
chmod 000 $ROOT/locked

cleanup() {
  chmod 755 -- $ROOT/locked 2>/dev/null
  zpty -d Z 2>/dev/null
}
trap cleanup EXIT INT TERM

#
# Harness
#

drain() {
  local acc='' chunk
  local -F end=$(( EPOCHREALTIME + ${1:-0.35} ))
  while (( EPOCHREALTIME < end )); do
    if zpty -r -t Z chunk 2>/dev/null; then acc+=$chunk; else sleep 0.02; fi
  done
  REPLY=$acc
}

# Type characters into the line editor without pressing Enter.
send() { zpty -w -n Z "$1"; drain ${2:-0.35} }
# Run a command and wait for the prompt.
line() { zpty -w Z "$1"; drain ${2:-0.4} }

strip() { LC_ALL=C sed -E $'s/\033\\[[0-9;?]*[a-zA-Z]//g; s/\033[()][A-Z]//g; s/\r//g' }

# The whole screen as text.
screen() {
  zpty -w -n Z $'\014'
  drain 0.45
  SCREEN=$(print -r -- "$REPLY" | strip)
}

# The whole screen as the terminal received it, escape sequences intact. Needed
# when the bug *is* an escape sequence: strip() would remove the evidence.
raw() {
  zpty -w -n Z $'\014'
  drain 0.45
  RAW=$REPLY
}
rawhas() { [[ $RAW == *$1* ]] && ok "$2" || bad "$2" "${(V)RAW}" }

# The whole screen with the selection marked, so highlighting can be asserted.
selected() {
  zpty -w -n Z $'\014'
  drain 0.45
  SEL=$(print -r -- "$REPLY" \
    | LC_ALL=C sed -E $'s/\033\\[7m/<INV>/g; s/\033\\[27m/<\\/INV>/g; s/\033\\[[0-9;?]*[a-zA-Z]//g; s/\r//g')
  # A terminal re-asserts standout whenever some *other* attribute changes
  # mid-run — a colour code between two characters of the selected row makes zsh
  # emit standout again for the next one. Stripping the colours above leaves
  # those repeats behind, so a correct screen can arrive as `<INV>c<INV>o<INV>m`.
  # Turning standout on while it is already on changes nothing on screen, and
  # neither does turning it off while it is already off: drop both.
  local -a parts
  local seg out=''
  local -i on=0
  parts=( ${(ps:\0:)${SEL//(#b)(<INV>|<\/INV>)/$'\0'$match[1]$'\0'}} )
  for seg in $parts; do
    case $seg in
      '<INV>')  (( on )) || { out+=$seg; on=1 } ;;
      '</INV>') (( on )) && { out+=$seg; on=0 } ;;
      *)        out+=$seg ;;
    esac
  done
  SEL=$out
}

typeset -gi PASS=0 FAIL=0
ok()  { (( PASS++ )); print -r -- "  ok   $1" }
bad() { (( FAIL++ )); print -r -- "  FAIL $1"; print -r -- "       screen: ${(V)2}" }

has()   { [[ $SCREEN == *$1* ]] && ok "$2" || bad "$2" "$SCREEN" }
hasnt() { [[ $SCREEN != *$1* ]] && ok "$2" || bad "$2" "$SCREEN" }
selis() { [[ $SEL == *"<INV>$1"* ]] && ok "$2" || bad "$2" "$SEL" }

# Abandon whatever is on the line, then assert where the shell ended up.
pwdis() {
  send $'\003' 0.3
  line 'print -r -- "PWD=$PWD"'
  [[ $REPLY == *"PWD=$1"* ]] && ok "$2" || bad "$2" "$REPLY"
}

#
# A shell with a competing ZLE plugin already installed, so the integration is
# tested the way it will actually be used.
#
RC=$ROOT/.rc
cat > $RC <<RCEOF
PS1="P> "
# Stands in for zsh-autosuggestions and friends: owns zle-line-pre-redraw and
# POSTDISPLAY, and must keep working once cdd is installed on top.
typeset -gi OTHER_RAN=0
# Stands in for zsh-autosuggestions specifically: puts a suggestion in
# POSTDISPLAY and colours it with a region_highlight entry that reaches past
# the end of BUFFER. Enabled per-test, because a real suggestion engine would
# otherwise take part in every other assertion here.
typeset -gi SUGGEST_ON=0
typeset -g  SUGGESTION='-suggested'
_other_pre_redraw() {
  (( OTHER_RAN++ ))
  if [[ \$BUFFER == other* ]]; then
    POSTDISPLAY=\$'\n[other-plugin]'
  fi
  if (( SUGGEST_ON )); then
    POSTDISPLAY=\$SUGGESTION
    region_highlight+=( "\${#BUFFER} \$(( \${#BUFFER} + \${#SUGGESTION} )) fg=8 memo=stand-in-suggestions" )
  fi
}
zle -N zle-line-pre-redraw _other_pre_redraw
RCEOF

zpty -b Z zsh -f -i
drain 0.8
line "source ${(q)RC}"
line "cd ${(q)ROOT}"
line "eval \"\$(${(q)BIN} init zsh)\""
line 'print -r -- READY'
[[ $REPLY == *READY* ]] || { print -r -- "harness: init failed: ${(V)REPLY}"; exit 1 }

print "— live preview while typing"
send 'cdd comp'; screen
has 'components'    'shows components before Enter'
has 'composables'   'shows composables before Enter'
hasnt 'config'      'a non-matching directory is filtered out'
hasnt 'notadir.txt' 'files are never listed'

print "— narrowing and broadening"
send 'onent'; screen
has 'components'    'components survives the narrower query'
hasnt 'composables' 'composables is dropped by the narrower query'
send $'\177\177\177\177\177'; screen
has 'composables'   'composables returns after backspace'

print "— selection"
selected
selis '▸ components'  'the first result is selected by default'
send $'\033[B'; selected
selis '▸ composables' 'down arrow moves the selection'
send $'\033[A'; selected
selis '▸ components'  'up arrow moves it back'
send $'\033[A'; selected
selis '▸ components'  'selection stops at the first row'
send $'\003' 0.3

print "— Enter"
send 'cdd comp'; send $'\r' 0.5
pwdis "$ROOT/components" 'Enter took the first match'

line "cd ${(q)ROOT}"
send 'cdd comp'; send $'\033[B'; send $'\r' 0.5
pwdis "$ROOT/composables" 'down then Enter took the selected match'

print "— Tab"
line "cd ${(q)ROOT}"
send 'cdd src/a'; send $'\t' 0.5; screen
has 'cdd src/alpha/' 'Tab completed the segment into the buffer'
has 'inner'          'children of the completed directory are listed'
send 'i'; send $'\r' 0.5
pwdis "$ROOT/src/alpha/inner" 'navigation continued after Tab'

print "— paths behave like paths"
line "cd ${(q)ROOT}"
send 'cdd src/alpha/'; send $'\r' 0.5
pwdis "$ROOT/src/alpha" 'a fully typed path is entered exactly as typed'

line "cd ${(q)ROOT}/src"
send 'cdd ..'; send $'\r' 0.5
pwdis "$ROOT" 'cdd .. went to the parent'

line "cd ${(q)ROOT}/src"
send 'cdd ~'; send $'\r' 0.5
pwdis "$HOME" 'cdd ~ went home'

line "cd ${(q)ROOT}/src"
send "cdd ${(q)ROOT}/comp"; send $'\r' 0.5
pwdis "$ROOT/components" 'an absolute path filtered its final segment'

line "cd ${(q)ROOT}"
send 'cdd link'; send $'\r' 0.5
pwdis "$ROOT/linked" 'a symlinked directory is navigable'

print "— hidden directories"
line "cd ${(q)ROOT}"
send 'cdd '; screen
hasnt '.secret' 'hidden directories stay hidden'
send '.'; screen
has '.secret'   'a leading dot reveals hidden directories'
send $'\003' 0.3

print "— spaces and unicode"
line "cd ${(q)ROOT}"
send 'cdd my'; screen
has 'my dir' 'a directory containing a space is listed'
send $'\r' 0.5
pwdis "$ROOT/my dir" 'entered a directory containing a space'

line "cd ${(q)ROOT}"
send $'cdd プ'; screen
has 'プロジェクト' 'a unicode directory is listed'
send $'\r' 0.5
pwdis "$ROOT/プロジェクト" 'entered a unicode directory'

print "— long lists"
line "cd ${(q)ROOT}/many"
send 'cdd d'; screen
has '+ 4 more' 'matches beyond the row limit are counted, not dropped'
send $'\003' 0.3
line 'CDD_MAX_ROWS=3'
send 'cdd d'; screen
has '+ 9 more' 'CDD_MAX_ROWS changes how many rows are shown'
send $'\003' 0.3
line 'CDD_MAX_ROWS=8'

print "— failure states"
line "cd ${(q)ROOT}"
send 'cdd locked/'; screen
has 'Permission denied' 'an unreadable directory says so inline'
send $'\003' 0.3

send 'cdd zzzz'; screen
has 'No matching directories' 'an empty result is reported quietly'
selected
[[ $SEL != *'<INV>'* ]] && ok 'nothing is highlighted when nothing matches' \
                        || bad 'nothing is highlighted when nothing matches' "$SEL"
send $'\r' 0.5
pwdis "$ROOT" 'Enter with no match left the directory unchanged'

print "— without the helper process"
line 'CDD_NO_SERVER=1'
line "cd ${(q)ROOT}"
send 'cdd comp'; screen
has 'components' 'the preview still works with one-shot invocations'
send $'\r' 0.5
pwdis "$ROOT/components" 'and Enter still navigates'
line 'unset CDD_NO_SERVER'

print "— other commands keep normal behaviour"
line "cd ${(q)ROOT}"
line 'print -r -- MARKERCMD'
send 'print'; send $'\033[A' 0.5; screen
has 'MARKERCMD' 'up arrow still searches history outside cdd'
send $'\003' 0.3
send 'cd sr'; send $'\t' 0.5; screen
has 'cd src/' 'Tab still completes filenames outside cdd'
send $'\003' 0.3
send 'print -r -- ECHOWORKS'; send $'\r' 0.5
[[ $REPLY == *ECHOWORKS* ]] && ok 'Enter still runs other commands' \
                            || bad 'Enter still runs other commands' "$REPLY"

print "— coexistence with another ZLE plugin"
send 'other'; screen
has '[other-plugin]' "another plugin's POSTDISPLAY is left alone"
send $'\003' 0.3
line 'print -r -- "OTHER_RAN=$OTHER_RAN"'
if [[ $REPLY == (#b)*OTHER_RAN=([0-9]##)* ]] && (( match[1] > 0 )); then
  ok "the pre-existing pre-redraw hook still runs"
else
  bad "the pre-existing pre-redraw hook still runs" "$REPLY"
fi

print "— cancelling leaves a clean prompt"
send 'cdd comp'; screen
has 'components' 'preview is up before cancelling'
send $'\003' 0.5; screen
hasnt 'composables' 'no stale preview rows after Ctrl+C'
hasnt 'components'  'no stale first row after Ctrl+C either'
pwdis "$ROOT" 'the directory is unchanged after Ctrl+C'

print "— the cdd command itself, when the line was executed rather than previewed"
line "cd ${(q)ROOT}"
# The buffer does not start with `cdd`, so the widget passes it through and the
# shell function runs — the same path a script or a non-ZLE shell would take.
line 'go() { cdd src/alpha }; go'
line 'print -r -- "PWD=$PWD"'
[[ $REPLY == *"PWD=$ROOT/src/alpha"* ]] && ok 'the cdd function enters a path that already resolves' \
                                        || bad 'the cdd function enters a path that already resolves' "$REPLY"

print "— the fallback picker"
line "cd ${(q)ROOT}"
zpty -w Z "CHOSEN=\$(${(q)BIN} pick)"
drain 0.6
send 'comp' 0.5
PICK=$(print -r -- "$REPLY" | strip)
[[ $PICK == *components* && $PICK == *composables* ]] \
  && ok 'the picker filters inline as you type' || bad 'the picker filters inline as you type' "$PICK"
send $'\033[B' 0.4
send $'\r' 0.6
line 'print -r -- "CHOSEN=${(Q)CHOSEN}"'
[[ $REPLY == *"CHOSEN=$ROOT/composables"* ]] && ok 'the picker returns the selected path on stdout' \
                                             || bad 'the picker returns the selected path on stdout' "$REPLY"

zpty -w Z "${(q)BIN} pick > /dev/null; print -r -- \"PICKSTATUS=\$?\""
drain 0.6
send 'comp' 0.4
send $'\003' 0.8
[[ $REPLY == *PICKSTATUS=1* ]] && ok 'cancelling the picker chooses nothing' \
                              || bad 'cancelling the picker chooses nothing' "$REPLY"
screen
hasnt 'composables' 'the picker leaves no rows behind'

print "— a plugin that highlights its own POSTDISPLAY"
# zsh-autosuggestions colours its suggestion with a region_highlight entry
# pointing past the end of BUFFER. Once the preview replaces that suggestion the
# entry describes text that is gone, and it lands on our rows instead: the
# selected row came out half in the suggestion's grey.
line "cd ${(q)ROOT}"
line 'SUGGEST_ON=1'
send 'cdd comp'; raw
# One unbroken standout run over the whole row. When the stale entry survives,
# its colour cuts the run in two and the row comes out half grey.
rawhas $'\033[7m▸ components' "the selection is not recoloured by the suggestion's highlight"
screen
hasnt '-suggested' 'the preview replaced the suggestion rather than joining it'
send $'\003' 0.3
line 'SUGGEST_ON=0'

print "— a multi-line prompt"
line $'PS1=$\'\\n%~\\n> \''
send 'cdd comp'; screen
has 'components' 'the preview renders under a multi-line prompt'
send $'\003' 0.5; screen
hasnt 'components' 'and cancelling still leaves nothing behind'

print -r -- "PASS=$PASS FAIL=$FAIL"
(( FAIL == 0 ))
