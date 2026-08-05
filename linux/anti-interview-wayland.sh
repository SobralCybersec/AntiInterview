#!/usr/bin/env bash
# anti-interview-wayland — toggle Hyprland's no_screen_share (capture-hiding) per window.
#
# Hides selected windows from screen-share/recording as a BLACK BOX (Wayland's
# ceiling — the shape still leaks; see PORTING_LINUX.md). The app's GUI button
# calls the `toggle`/`menu` subcommands; nothing here is Hyprland-version-fragile.
#
# Why a managed include instead of `hyprctl keyword`: on Lua-config Hyprland
# (Omarchy/HyprMod) runtime `keyword` is refused ("non-legacy parser"). Rewriting
# a sourced file + `hyprctl reload` is the one mechanism that works everywhere.
#
# Usage:
#   anti-interview-wayland.sh setup            # one-time: wire the include (idempotent)
#   anti-interview-wayland.sh list             # identification: list capturable windows
#   anti-interview-wayland.sh status           # show currently hidden classes
#   anti-interview-wayland.sh toggle <class>   # flip hide on/off for a window class
#   anti-interview-wayland.sh on <class>       # hide it
#   anti-interview-wayland.sh off <class>      # unhide it
#   anti-interview-wayland.sh clear            # unhide everything
#   anti-interview-wayland.sh menu             # interactive picker (the "button")
#   anti-interview-wayland.sh selftest         # assert toggle logic (no GUI)
set -euo pipefail

HYPR_DIR="${HYPR_DIR:-$HOME/.config/hypr}"
MANAGED="$HYPR_DIR/anti-interview.lua"
ENTRY="$HYPR_DIR/hyprland.lua"
HEADER="-- anti-interview managed rules — DO NOT EDIT BY HAND (rewritten by anti-interview-wayland.sh)"

die() { echo "anti-interview: $*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

require_hyprland() {
  have hyprctl || die "hyprctl not found — not a Hyprland session"
  [ -n "${HYPRLAND_INSTANCE_SIGNATURE:-}" ] || die "no HYPRLAND_INSTANCE_SIGNATURE — run inside Hyprland"
}

# ---- managed-file <-> hidden-class-set ---------------------------------------

# Emit the set of currently-hidden classes, one per line.
current_classes() {
  [ -f "$MANAGED" ] || return 0
  # pull CLASS out of: o.window("CLASS", { no_screen_share = true })
  sed -nE 's/^o\.window\("([^"]+)".*/\1/p' "$MANAGED" 2>/dev/null || true
}

# Rewrite the managed file from a class list (args), then reload Hyprland.
write_classes() {
  local tmp; tmp="$(mktemp)"
  printf '%s\n' "$HEADER" > "$tmp"
  local c
  for c in "$@"; do
    [ -n "$c" ] && printf 'o.window(%s, { no_screen_share = true })\n' "\"$c\"" >> "$tmp"
  done
  mv "$tmp" "$MANAGED"
  hyprctl reload >/dev/null
}

# ---- subcommands -------------------------------------------------------------

cmd_setup() {
  mkdir -p "$HYPR_DIR"
  [ -f "$MANAGED" ] || printf '%s\n' "$HEADER" > "$MANAGED"
  # require path must be "hypr.anti-interview" (module dir is ~/.config/hypr) — a
  # bare "anti-interview" resolves to ~/.config/ and fails to load. Regression-tested.
  if [ -f "$ENTRY" ] && ! grep -q 'require("hypr.anti-interview")' "$ENTRY"; then
    printf '\n-- anti-interview automation (managed rules)\nrequire("hypr.anti-interview")\n' >> "$ENTRY"
    hyprctl reload >/dev/null 2>&1 || true
    echo "wired require(\"anti-interview\") into $ENTRY"
  else
    echo "already wired (or non-Lua config — put 'source = $MANAGED' in your hyprland.conf)"
  fi
}

cmd_list() {
  require_hyprland
  # identification: class + title of every real, capturable window
  hyprctl clients -j | python3 -c '
import json, sys
seen = set()
for w in sorted(json.load(sys.stdin), key=lambda w: w.get("class","")):
    c = w.get("class","")
    if not c or w.get("hidden"): continue
    if c in seen: continue
    seen.add(c)
    t = w.get("title","")[:60]
    print(c + "\t" + t)
'
}

cmd_status() {
  local cs; cs="$(current_classes | sort -u)"
  if [ -z "$cs" ]; then echo "hidden: (none)"; else echo "hidden:"; echo "$cs" | sed 's/^/  /'; fi
}

# Raw hidden classes, one per line — for programmatic callers (the GUI).
cmd_classes() { current_classes | sort -u; }

_has_class() { current_classes | grep -qxF "$1"; }

cmd_on()  { require_hyprland; [ $# -ge 1 ] || die "on: need a class"
  local set; mapfile -t set < <(current_classes | sort -u)
  _has_class "$1" || set+=("$1")
  write_classes "${set[@]}"; echo "hidden: $1"; }

cmd_off() { require_hyprland; [ $# -ge 1 ] || die "off: need a class"
  local keep=(); local c
  while IFS= read -r c; do [ "$c" = "$1" ] || keep+=("$c"); done < <(current_classes | sort -u)
  write_classes "${keep[@]}"; echo "shown: $1"; }

cmd_toggle() { require_hyprland; [ $# -ge 1 ] || die "toggle: need a class"
  if _has_class "$1"; then cmd_off "$1"; else cmd_on "$1"; fi; }

cmd_clear() { require_hyprland; write_classes; echo "all windows shown"; }

# The "button": pick a window from a menu; toggle it. Uses whatever picker exists.
cmd_menu() {
  require_hyprland
  local picker=""
  for p in walker wofi rofi fuzzel; do have "$p" && { picker="$p"; break; }; done
  [ -n "$picker" ] || die "no menu tool (walker/wofi/rofi/fuzzel) found"
  local rows; rows="$(cmd_list)"
  # annotate current state so the menu reads as on/off
  local menu=""; local cls st
  while IFS=$'\t' read -r cls title; do
    if _has_class "$cls"; then st="●hidden"; else st="○shown "; fi
    menu+="$st  $cls  —  $title"$'\n'
  done <<< "$rows"
  local pick
  case "$picker" in
    rofi)   pick="$(printf '%s' "$menu" | rofi -dmenu -p 'toggle capture-hide')" ;;
    wofi)   pick="$(printf '%s' "$menu" | wofi --dmenu -p 'toggle capture-hide')" ;;
    fuzzel) pick="$(printf '%s' "$menu" | fuzzel --dmenu -p 'toggle capture-hide ')" ;;
    walker) pick="$(printf '%s' "$menu" | walker --dmenu -p 'toggle capture-hide')" ;;
  esac
  [ -n "$pick" ] || exit 0
  cls="$(awk '{print $2}' <<< "$pick")"
  [ -n "$cls" ] && cmd_toggle "$cls"
}

# ---- selftest: assert the class-set logic without touching a live session ----
cmd_selftest() {
  local d; d="$(mktemp -d)"; HYPR_DIR="$d"; MANAGED="$d/anti-interview.lua"
  hyprctl() { :; }  # stub reload
  write_classes firefox chrome
  [ "$(current_classes | sort | tr '\n' ',')" = "chrome,firefox," ] || die "selftest: write/read mismatch"
  # off removes only the target
  local keep=(); local c
  while IFS= read -r c; do [ "$c" = "firefox" ] || keep+=("$c"); done < <(current_classes | sort -u)
  write_classes "${keep[@]}"
  [ "$(current_classes | tr '\n' ',')" = "chrome," ] || die "selftest: off logic broken"
  # clear empties it
  write_classes
  [ -z "$(current_classes)" ] || die "selftest: clear logic broken"
  rm -rf "$d"; echo "selftest OK"
}

case "${1:-menu}" in
  setup)   cmd_setup ;;
  list)    cmd_list ;;
  status)  cmd_status ;;
  classes) cmd_classes ;;
  on)      shift; cmd_on "$@" ;;
  off)     shift; cmd_off "$@" ;;
  toggle)  shift; cmd_toggle "$@" ;;
  clear)   cmd_clear ;;
  menu)    cmd_menu ;;
  selftest) cmd_selftest ;;
  *) die "unknown command: $1 (try: setup|list|status|on|off|toggle|clear|menu|selftest)" ;;
esac
