---
description: Build a study plan from accumulated codetrace sessions
---

Read every session in `${XDG_DATA_HOME:-$HOME/.local/share}/codetrace/sessions/`
plus `fixtures/` and produce a study plan.

Requirements:

- **Aggregate before diagnosing.** Build the table of sessions first: problem,
  duration, compile failures, time-to-first-passing-test, whether the invariant
  was stated. Patterns come from the table, not from vibes.
- **Separate the two axes**: problem-solving ability and communication under
  observation. They dissociate — I can solve well and interview badly. Rate them
  independently and say which is the binding constraint.
- **Name mechanisms, not topics.** "You do not state loop invariants before
  coding, so you write branches for cases the problem forbids" is actionable.
  "Practice more string problems" is not.
- **Sequence by dependency.** If I cannot state invariants, more problems will
  reinforce the gap rather than close it. Say what must be fixed first.
- **Be explicit about sample size.** With fewer than ~5 sessions, say the plan is
  provisional and name what evidence would change it.

Output: a short diagnosis, a ranked list of at most 3 mechanisms to fix, and the
next 5 problems with the specific mechanism each one targets.
