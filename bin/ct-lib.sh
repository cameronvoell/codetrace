# shared helpers for the codetrace shell harness
# shellcheck shell=bash

CT_SESSIONS="${CT_SESSIONS:-${XDG_DATA_HOME:-$HOME/.local/share}/codetrace/sessions}"
CT_STATE="${CT_STATE:-${XDG_STATE_HOME:-$HOME/.local/state}/codetrace}"
CT_CURRENT="$CT_STATE/current"

ct_now_ms() { date +%s%3N; }

ct_session_dir() {
  [ -f "$CT_CURRENT" ] || { echo "no active session (run: ct-session start <problem>)" >&2; return 1; }
  local d; d="$(cat "$CT_CURRENT")"
  [ -d "$d" ] || { echo "active session missing: $d" >&2; return 1; }
  printf '%s' "$d"
}

# ct_emit <session_dir> <type> [json_object_fragment]
# fragment must be a JSON object, e.g. '{"exit":1,"ms":240}'
ct_emit() {
  local dir="$1" type="$2" extra="${3:-\{\}}"
  local start now t
  start="$(cat "$dir/.start_ms")"
  now="$(ct_now_ms)"
  t=$(( now - start ))
  jq -cn --argjson t "$t" --arg type "$type" --argjson extra "$extra" \
    '{t:$t, type:$type} + $extra' >> "$dir/session.jsonl"
}

# ct_commit <repo> <message> -> prints short sha (empty if nothing changed)
ct_commit() {
  local repo="$1" msg="$2"
  git -C "$repo" add -A
  if git -C "$repo" diff --cached --quiet; then
    git -C "$repo" rev-parse --short HEAD 2>/dev/null || true
  else
    git -C "$repo" commit -q -m "$msg"
    git -C "$repo" rev-parse --short HEAD
  fi
}
