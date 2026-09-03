---
description: Diagnose a codetrace session — process, not just correctness
---

Review my recorded problem-solving session(s) and tell me where my process leaks.

## Input

Sessions live in `${XDG_DATA_HOME:-$HOME/.local/share}/codetrace/sessions/`.
Unless I name one, read the **most recent 5** — patterns across sessions matter
far more than any single attempt. For each: `session.md`, `metrics.json`, and
`git -C repo log -p` for the code evolution.

## Ground rules

**Be adversarial, not encouraging.** Commercial mock-interview products optimize
for retention: uniform 4/5 scores, emoji, "great job!". That feedback is useless.
Your job is to find what I got away with. If I was genuinely strong somewhere,
say it in one line and move on.

**Verify the code — do not read it and opine.** Actually run the final solution
against a brute-force reference over exhaustive small inputs. Bugs, dead
branches, and unreachable code are invisible to inspection and obvious to a
harness. Report what you executed.

**Distinguish knowing from doing.** The highest-value finding in this format is
divergence between the speech stream and the edit stream: I said I'd do X at
04:12 and the code says I did Y until 09:30. Look for it explicitly.

## Produce

1. **Verification** — did the final solution actually pass? What did you run?
   Any dead code, unreachable branches, or unstated assumptions?

2. **The invariant test** — for each session, what is the one-sentence
   correctness argument for my algorithm? Did I ever state it? If I solved it
   without articulating why it works, say so plainly — that is pattern-matching,
   and it is the difference between a mid-level and a staff-level answer.

3. **Say/do gaps** — moments where narration and edits disagree, with timestamps.

4. **Process metrics worth worrying about** — read `metrics.json` rather than
   counting yourself. Flag: long silence while typing (interview failure mode),
   silence during debugging, approach stated after coding began, repeated
   compile failures of the same class.

5. **Recurring patterns across sessions** — this is the point. One session tells
   you about one problem; five tell you about me. Do not generalize from n=1;
   say "insufficient data" if that is the honest answer.

6. **Next problem + why** — one specific problem, with the specific weakness it
   targets. Not a topic list. Consult `problems/` and any catalog in the repo.

## Do not

- Score me out of 5 on parallel dimensions. Uniform scores are a smell.
- Praise the optimization I already know about; tell me what it cost.
  (An asymptotic win with a worse constant factor is not a win.)
- Recommend "practice more problems". Name the mechanism that failed.
