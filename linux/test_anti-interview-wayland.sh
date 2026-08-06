#!/usr/bin/env bash
# Tests for anti-interview-wayland.sh — no real Hyprland needed.
# A mock `hyprctl` on PATH simulates clients/reload/getprop/configerrors, and
# crucially FAILS configerrors if the managed Lua is malformed — so the
# double-quote regression that broke a live config can never come back silently.
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
SCRIPT="$HERE/anti-interview-wayland.sh"
PASS=0; FAIL=0
ok()   { PASS=$((PASS+1)); echo "  ok   - $1"; }
bad()  { FAIL=$((FAIL+1)); echo "  FAIL - $1"; }
check(){ if eval "$2"; then ok "$1"; else bad "$1 ($2)"; fi; }
run()  { bash "$SCRIPT" "$@"; }

# ---- sandbox: temp HYPR_DIR + mock hyprctl on PATH --------------------------
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
export HYPR_DIR="$WORK/hypr"; mkdir -p "$HYPR_DIR"
: > "$HYPR_DIR/hyprland.lua"   # empty entry file so `setup` can wire the include
export HYPRLAND_INSTANCE_SIGNATURE="test-sig"

BIN="$WORK/bin"; mkdir -p "$BIN"
cat > "$BIN/hyprctl" <<MOCK
#!/usr/bin/env bash
MANAGED="$HYPR_DIR/anti-interview.lua"
case "\$1 \${2:-}" in
  "clients -j")
    echo '[{"class":"firefox","address":"0x1","title":"Aulas","hidden":false},{"class":"kitty","address":"0x2","title":"tmux","hidden":false}]' ;;
  "reload ") echo ok ;;
  "getprop "*)
    # naive: report true if the last arg's class appears in the managed file
    cls=""
    case "\$*" in *0x1*) cls=firefox;; *0x2*) cls=kitty;; esac
    if grep -q "o.window(\"\$cls\"" "\$MANAGED" 2>/dev/null; then echo true; else echo false; fi ;;
  "configerrors ")
    # FAIL LOUD on the exact malformed-Lua bug: a doubled quote in a rule line
    if grep -qE 'o\.window\("[^"]*""' "\$MANAGED" 2>/dev/null; then
      echo "config error: unfinished string"; exit 0
    fi ;;
  *) : ;;
esac
MOCK
chmod +x "$BIN/hyprctl"
export PATH="$BIN:$PATH"

echo "== anti-interview-wayland tests =="

# ---- offline unit logic -----------------------------------------------------
run selftest >/dev/null 2>&1 && ok "selftest passes" || bad "selftest passes"

# ---- setup wires the include once, idempotently -----------------------------
run setup >/dev/null
check "setup wires require" "grep -q 'require(\"hypr.anti-interview\")' '$HYPR_DIR/hyprland.lua'"
run setup >/dev/null
check "setup idempotent (one require line)" "[ \$(grep -c 'require(\"hypr.anti-interview\")' '$HYPR_DIR/hyprland.lua') -eq 1 ]"

# ---- on / status ------------------------------------------------------------
run on firefox >/dev/null
check "on firefox writes rule" "grep -q 'o.window(\"firefox\", { no_screen_share = true })' '$HYPR_DIR/anti-interview.lua'"
check "status shows firefox" "run status | grep -q firefox"
check "getprop reflects hidden" "[ \"\$(hyprctl getprop address:0x1 no_screen_share)\" = true ]"

# ---- idempotent on (no duplicate) -------------------------------------------
run on firefox >/dev/null
check "on is idempotent (no dup)" "[ \$(grep -c 'o.window(\"firefox\"' '$HYPR_DIR/anti-interview.lua') -eq 1 ]"

# ---- REGRESSION: managed file must never be malformed Lua --------------------
check "no doubled-quote regression" "! grep -qE 'o\\.window\\(\"[^\"]*\"\"' '$HYPR_DIR/anti-interview.lua'"
check "config clean per hyprctl" "[ -z \"\$(hyprctl configerrors)\" ]"

# ---- second class + off removes only the target -----------------------------
run on kitty >/dev/null
check "two classes hidden" "[ \$(grep -c 'o.window(' '$HYPR_DIR/anti-interview.lua') -eq 2 ]"
run off firefox >/dev/null
check "off removes firefox only" "! grep -q 'firefox' '$HYPR_DIR/anti-interview.lua' && grep -q 'kitty' '$HYPR_DIR/anti-interview.lua'"

# ---- toggle flips state -----------------------------------------------------
run toggle kitty >/dev/null   # kitty was on -> off
check "toggle off kitty" "! grep -q 'kitty' '$HYPR_DIR/anti-interview.lua'"
run toggle kitty >/dev/null   # off -> on
check "toggle on kitty" "grep -q 'o.window(\"kitty\"' '$HYPR_DIR/anti-interview.lua'"

# ---- clear ------------------------------------------------------------------
run clear >/dev/null
check "clear removes all rules" "[ \$(grep -c 'o.window(' '$HYPR_DIR/anti-interview.lua') -eq 0 ]"
check "clear keeps header" "grep -q 'DO NOT EDIT' '$HYPR_DIR/anti-interview.lua'"

# ---- list (identification) --------------------------------------------------
check "list shows firefox+kitty" "[ \$(run list | wc -l) -eq 2 ]"

echo "== $PASS passed, $FAIL failed =="
[ "$FAIL" -eq 0 ]
