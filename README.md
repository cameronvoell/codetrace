# codetrace

Record how you actually solve a problem — not just the answer you landed on.

`codetrace` captures a problem-solving session as a single aligned timeline:

- **what you said** — mic audio, transcribed with timestamps
- **what you did** — compile / test / run / idle events, with the code at each point
- **what you meant** — hotkey markers for "approach stated", "complexity stated", "stuck"

The output is a plain-text artifact you own, designed to be handed to an LLM for
diagnosis: *not* "was your answer right", but "where does your process leak".

## Why

A final solution tells you whether you got it right. It cannot tell you that you
said "I'll use a hashmap for O(n)" at 02:31 and then wrote nested loops until
08:40. That gap — between what you know and what you do under pressure — is the
thing worth fixing, and it only exists in the timeline.

## Status

Early. The shell harness (`bin/ct-run`) and the review prompts work today with no
Rust involved. The recorder itself is in progress.

## Quick start (no build required)

```bash
export PATH="$PWD/bin:$PATH"
ct-session start removable-indices     # begins a session, prints its id
cd problems/removable-indices
$EDITOR Solution.java
ct-run                                 # compile + test, emits events, commits
ct-session end
```

Then, in Claude Code:

```
/review-session
```

## Format

See [SPEC.md](SPEC.md). The format is the point — the recorder is a reference
implementation of it. Sessions are JSONL event streams plus a git repo holding
the code at each event.

## Layout

```
SPEC.md          the session format (start here)
bin/             shell harness — works today, no build
src/             the Rust recorder (TUI dashboard, audio, PTY capture)
problems/        example problems (original only — see note in SPEC)
prompts/         Claude prompts for review + study planning
fixtures/        recorded sessions used as regression tests
```

## License

MIT
