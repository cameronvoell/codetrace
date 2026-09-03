# codetrace session format — v0.1

A **session** is one recorded attempt at one problem. It is a directory:

```
2026-09-03T14-02-11-removable-indices/
├── manifest.json     what/when/where, clock anchors, media references
├── session.jsonl     the event stream — the core artifact
├── session.wav       mic audio (optional)
├── session.md        rendered view for LLM consumption (derived)
├── metrics.json      precomputed measures (derived)
├── pty.log           raw terminal capture (optional)
└── repo/             a git repo; code state at every event
```

`session.jsonl` and `manifest.json` are the format. Everything else is optional
or derived and may be regenerated at any time.

---

## 1. Clock

Every event carries `t`: **integer milliseconds since session start**.

`t = 0` is the moment audio capture begins. If there is no audio, it is the
moment `session.start` is written. This single rule is what makes speech and code
events comparable, so it must hold even when a stream is absent.

`manifest.json` records the wall-clock anchor so sessions can be correlated with
external recordings (OBS, a screen capture, a video call):

```json
{
  "spec": "0.1",
  "id": "2026-09-03T14-02-11-removable-indices",
  "problem": "removable-indices",
  "lang": "java",
  "started_utc": "2026-09-03T14:02:11.412Z",
  "duration_ms": 2_681_000,
  "media": { "audio": "session.wav", "video": null, "video_offset_ms": 0 },
  "tool": { "name": "codetrace", "version": "0.1.0" }
}
```

`video_offset_ms` is how far into the video file `t=0` falls, so any event can be
turned into a seek position. This is how the format stays useful once models can
watch video directly: the text stays the cheap index, the video stays reachable.

---

## 2. Events

One JSON object per line, ordered by `t` non-decreasing. Unknown event types must
be ignored by readers, not rejected — this is how the format stays extensible.

Every event has `t` and `type`. Everything else is per-type.

### Lifecycle

```jsonc
{"t":0,"type":"session.start","problem":"removable-indices","lang":"java"}
{"t":2681000,"type":"session.end","reason":"user"}   // user | timeout | crash
```

### Speech

```jsonc
{"t":183400,"type":"speech","dur":8100,
 "text":"the sort is only there because I walk backwards"}
```

Segment-level, not word-level. Word timestamps are noise for this purpose.
Absent when the session had no audio.

### Code state

Code is **never inlined**. Each event references a commit in `repo/`:

```jsonc
{"t":190100,"type":"edit","sha":"a3f9c21","files":1,"added":12,"removed":3}
```

Retrieve the code at any event with `git -C repo show <sha>`, and the change with
`git -C repo show <sha> -p`. Diffs, ordering, and dedup come free from git; the
event stream stays small enough to read.

### Build and test

```jsonc
{"t":190100,"type":"compile.error","sha":"a3f9c21","exit":1,
 "diags":[{"file":"Solution.java","line":42,"col":9,
           "msg":"incompatible types: List<Integer> cannot be converted to LinkedList<Integer>"}]}

{"t":241800,"type":"compile.ok","sha":"b7c2f08","exit":0,"ms":1240}

{"t":259000,"type":"test.run","sha":"b7c2f08",
 "passed":6,"failed":2,"total":8,
 "failing":[{"case":"04","expected":"3\n4\n5","actual":"3\n4"}]}

{"t":301200,"type":"run","sha":"b7c2f08","exit":0,"ms":88}
```

Adapters should prefer machine-readable toolchain output over regex where it
exists (`cargo --message-format=json`, `tsc --pretty false`, JUnit XML).

### Attention

```jsonc
{"t":301500,"type":"idle","dur":71000}
```

Emitted when no edit, keystroke, or speech occurs for longer than the idle
threshold (default 30s). Recorded at the *end* of the gap, with its duration.
Silence is signal — an idle gap during debugging is the strongest stuck-indicator
in the format.

### Markers

User-pressed, one keystroke each. These exist because interview rubrics grade on
them and they are almost free to capture:

```jsonc
{"t":41200,"type":"marker","name":"approach.stated"}
{"t":62800,"type":"marker","name":"complexity.stated"}
{"t":455000,"type":"marker","name":"stuck"}
{"t":903000,"type":"marker","name":"note","text":"forgot Collections.sort again"}
```

### Debugging (v0.2)

Captured by proxying the Debug Adapter Protocol, which gets every language and
every DAP-speaking editor at once:

```jsonc
{"t":510000,"type":"debug.start","adapter":"java"}
{"t":512400,"type":"breakpoint.set","file":"Solution.java","line":57}
{"t":518900,"type":"breakpoint.hit","file":"Solution.java","line":57}
{"t":602000,"type":"debug.end"}
```

---

## 3. Derived outputs

Neither is authoritative; both are regenerated from `session.jsonl`.

### `metrics.json`

Precomputed because LLMs are unreliable at counting across a long transcript and
reliable at interpreting numbers handed to them.

```json
{
  "duration_ms": 2681000,
  "time_to_first_compile_ms": 412000,
  "time_to_approach_stated_ms": 41200,
  "compile_attempts": 7,
  "compile_failures": 3,
  "test_runs": 4,
  "time_to_first_passing_test_ms": 1204000,
  "longest_idle_ms": 71000,
  "idle_total_ms": 214000,
  "silence_while_typing_ms": 388000,
  "speech_segments": 96,
  "edit_churn_ratio": 1.9,
  "stated_complexity_before_coding": true
}
```

`silence_while_typing_ms` is worth calling out: crossing the speech and edit
streams gives four quadrants, and two of them are interview failure modes.

|              | talking             | silent                       |
|--------------|---------------------|------------------------------|
| **typing**   | normal              | ⚠ silent implementation      |
| **not typing** | thinking aloud ✅  | ⚠ stuck and hiding it        |

### `session.md`

A flat human- and LLM-readable rendering: events in order, speech interleaved,
diffs inlined at compile/test boundaries. This is what gets pasted or piped into
a model. It is a *view* — never edit it, regenerate it.

---

## 4. Compatibility notes

**ProgSnap2.** Event names deliberately echo the ProgSnap2 vocabulary used in
computing-education research (`Compile`, `Compile.Error`, `Run.Program`,
`File.Edit`, `Session.Start`). A session should be mechanically convertible to
ProgSnap2 CSV, minus the speech stream, which ProgSnap2 has no notion of.

**Problem statements are not stored.** `problems/*/meta.json` holds a title,
topic tags, difficulty, and a URL — never the statement text. Most interview
problem text is copyrighted and must not be redistributed. Test cases in this
repo are original.

**Privacy.** A session contains a recording of your voice and the code you wrote.
Sessions default to `$XDG_DATA_HOME/codetrace/sessions` and are never written
inside this repo. Transcription runs locally. Nothing is uploaded by any part of
this tool.

**Versioning.** `manifest.spec` is required. This format will change; readers
should check it and refuse versions they do not understand.
