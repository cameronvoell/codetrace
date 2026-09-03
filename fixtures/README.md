# fixtures

Recorded sessions used as regression tests for the analysis prompts.

## hackerrank-mock-interview-2026-09-03.md

A HackerRank AI mock interview (Java, "removable indices", rated 4/5 across all
four rubric dimensions). Captured before codetrace existed, so it has a speech
transcript but no code timeline — which is precisely what makes it useful.

**Ground truth the platform missed.** `/review-session` on this fixture should
surface at least:

1. The forward loop in `includeAdjacent` is **dead code**. Reaching it requires
   `str1[first+1] == str2[first]` while `str1[first] != str2[first]`, so
   `str1[first+1] != str1[first]` always and the loop breaks on its first
   iteration. Verified: 0 executions across 1,804,530 exhaustive cases.
2. The **correctness invariant was never stated** — that removing index j or k
   yields the same string iff `str1[j..k]` is a constant run. The dead loop is a
   direct symptom: with the invariant explicit, you would not search forward.
3. The O(n log n) → O(n) "optimization" (ArrayList+sort → LinkedList+addFirst)
   is asymptotically correct but plausibly **slower in wall-clock** at n=2·10⁵ —
   200k node objects holding boxed Integers. The real insight is that the answer
   is a contiguous range, so no list-building strategy is needed at all.
4. The `solution.isEmpty()` branch is **removable**, not (as the platform
   suggested) in need of a comment.

The solution is otherwise **correct** — 1.8M exhaustive cases, zero mismatches.

A review that returns only "ask about Collections.sort" and "list edge cases
upfront" has failed this fixture.
