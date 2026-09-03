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

# names of real capture devices (monitors excluded — those are desktop audio)
ct_list_mics() {
  pactl list sources 2>/dev/null | awk '
    /^\tName:/            { name=$2 }
    /^\tMonitor of Sink:/ { if ($4 == "n/a") print name }'
}

# Resolve which source to record from, at call time so hotplug is picked up.
#   CT_AUDIO_SOURCE  exact source name, wins outright
#   CT_AUDIO_PREFER  colon-separated regexes, highest priority first;
#                    first one matching a connected mic wins
#   otherwise        the system default
ct_resolve_source() {
  if [ -n "${CT_AUDIO_SOURCE:-}" ]; then
    printf '%s\t%s\n' "$CT_AUDIO_SOURCE" "CT_AUDIO_SOURCE is set"; return
  fi
  if [ -n "${CT_AUDIO_PREFER:-}" ]; then
    local mics pat hit
    mics="$(ct_list_mics)"
    while IFS= read -r pat; do
      [ -n "$pat" ] || continue
      hit="$(grep -m1 -E "$pat" <<<"$mics" || true)"
      if [ -n "$hit" ]; then
        printf '%s\t%s\n' "$hit" "CT_AUDIO_PREFER matched /$pat/"; return
      fi
    done <<< "$(tr ':' '\n' <<<"$CT_AUDIO_PREFER")"
  fi
  printf '%s\t%s\n' "$(pactl get-default-source 2>/dev/null || echo default)" "system default"
}
