# HackerRank AI Mock Interview — Session Record

**Date:** 2026-09-03
**Platform:** HackerRank AI Mock Interview (Coding / Software Engineer)
**Candidate:** Cameron Voell
**Language:** Java
**Duration:** ~60 min
**Overall rating:** 4/5

> **Note on fidelity:** Cameron's turns are reproduced verbatim, including
> speech-to-text artifacts ("I'm I'm thinking", "If If you don't"). These are
> preserved deliberately — the disfluencies mark where he was thinking aloud
> and are signal, not noise. Do not clean them up.

> **Purpose:** first test fixture for the session-recorder project. See the
> companion analysis for a ground-truth diagnosis to check tool output against.

---

## Problem Statement

Given two strings, `str1` and `str2`, where `str1` contains exactly one character
more than `str2`, find the indices of the characters in `str1` that can be removed
to make `str1` equal to `str2`. Return the array of indices in increasing order.
If it is not possible, return the array `[-1]`.

**Note:** Use 0-based indexing.

**Example**

```
str1 = "abdgggda"
str2 = "abdggda"
```

Any `"g"` character at positions 3, 4, or 5 can be deleted to obtain `str2`.
Return `[3, 4, 5]`.

**Function Description**

Complete the function `getRemovableIndices` in the editor below.

`getRemovableIndices` has the following parameters:

- `string str1`: the string to modify
- `string str2`: the target string

**Output Format**

`int[]`: the indices of characters that can be removed from `str1` in ascending
order, or `[-1]` if it is not possible to match `str2`

**Constraints**

```
2 <= |str1| <= 2 * 10^5
1 <= |str2| <= 2 * 10^5
|str1| = |str2| + 1
str1 and str2 only contain lowercase English letters.
```

---

## Transcript

**Interviewer:** Hey there, nice to meet you Cameron! 😊 Interviews can be intimidating so good job on taking this mock interview powered by HackerRank. Here is a little bit about me — I am an AI Mock Interviewer. You can discuss your problem solving approach with me, ask me questions when you are stuck. Remember to keep communicating with me. We will start with your technical introduction post which I will provide you with the problem statement and then we will discuss the problem statement together. Good Luck!🎯

**Interviewer:** It's great to have you here! Tell me a little about yourself and your background — what have you been working on lately?😊

**Cameron:** hello! im a staff level engineer who has worked on mobile app codebases, building group chat apps, and maintained a e2ee chat sdk that has core in rust and then uses uniffi bindings for our sdks in kotlin for android, swift for ios, and typescript for react native. I prefer to do coding exercises in java because that is the language I have memorized best

**Interviewer:** That's really impressive! 🌟 Building an E2EE chat SDK with Rust at the core and maintaining bindings across multiple platforms is quite a feat — that's a lot of complexity to manage!

As a staff engineer, you must have dealt with some really interesting architectural challenges. What's been the most technically challenging part of maintaining those cross-platform bindings? 🤔

**Cameron:** there are some exceptions that have to be made when rust is compiled to wasm for web bindings versus uniffi bindings for android or iOS. making CI run efficiently against those environments was one of the big challenges

**Interviewer:** That sounds like a fascinating challenge! Managing cross-platform CI pipelines with WASM and native targets simultaneously is no small feat. 💪

Anyway, let's get into the interview! Here is the Interview Question. Feel free to explain your approach first before starting to code.

*(problem statement delivered — see above)*

**Cameron:** All right, I'm just going to read the problem description and let you know when I'm done.

**Interviewer:** Take your time! 👍

**Cameron:** Okay, thinking through my algorithm. I'm I'm thinking that you can just step through both strings, comparing each character. And the first time you see one that's different, you can remove it from string one. you continue to compare. Each character in the two strings. you find a second character that differs return negative one. If If you don't. Then Then return the index of the first character that you found. No, that is not quite right. That is how you find the first. Character that can be removed. But you want to know all of the indices of characteristic can be removed. Let me think a little bit

**Interviewer:** Take your time! 🤔 You're on the right track with the initial thinking — keep going!

**Cameron:** I see now. My first algorithm was correct. But the first time you find a character that's different. You should return the indices of all adjacent characters that are the same. Or at least you should cash those. And then continue with the comparison to make sure there are no more differences. there are return negative one. If If there's not, return the array of indices that you cached or saved

**Interviewer:** That sounds like a solid approach! 🎯 Go ahead and start coding it up!

**Cameron:** So the initial solution could be to. Justify the first character that isn't the same at a given index between the two strings. then afterwards calculate. How many adjacent characters in string. One exist. And And then an optimization later on would be that as you're stepping through each string. You save adjacent indices of the current character that you're looking at. that way you dont need a second processing over the list to calculate adjacent indices

**Interviewer:** Good start on the skeleton! 💪 You've got the loop going. Now think about what happens when you find the first differing character — what should you do at that point in the code?

**Interviewer:** ⏰ You still have plenty of time remaining! If you're still planning your approach, consider moving to implementation soon. Keep going — you're doing well!💪

**Interviewer:** I see that you are making some progress in the code, you can brainstorm your approach or ask me anything in case you get stuck. 🤔

**Interviewer:** One thing to consider — think about what happens when you reach the end of str2 without finding any difference. Is your current logic handling that case correctly? 🧐

**Cameron:** i forget the basic utility for sorting an arraylist of integers in java, do you mind reminding me

**Interviewer:** You can use `Collections.sort(list)` to sort an ArrayList of integers!👍

**Interviewer:** I see all the sample test cases have passed, lets move on and try Submit Code to run across the hidden test cases.

**Cameron:** so I definitely know I dont have an ideal solution yet. O(n) for the initial comparison + O(n) for finding adjacent + O(nlogn) for the sort. but we should be able to just get O(n) if I add a little more storage for tracking adjacent items during the main traversal. should I still submit now or update the solution first?

**Interviewer:** Good thinking on the complexity analysis! 💡 I'd suggest submitting the current solution first to see how it performs against all the hidden test cases, and then we can discuss the optimization.😊

**Interviewer:** I see all the test cases have passed, I have a couple of questions for you, are you ready?

**Cameron:** yep

**Interviewer:** Great! 🎯 You mentioned your current solution is O(n log n) due to the sort. Can you walk me through why exactly the sort is needed and what the overall time and space complexity of your solution is?

**Cameron:** right now the sort is in the solution because im calculating adjacent tiles that match our character in the string by walking backwards in the string. I supposed I could store the solution in a linked list / deque and just add to the front for that first traversal and that would avoid the sort. that would get me from nlogn back to O(n) times some constant since its not all in one traversal

**Interviewer:** That's a good observation! 👍 Can you also tell me what the space complexity of your current solution is?

**Cameron:** space complexity is just O(n) since I just store the indices once in the includeAdjacent function.

**Interviewer:** Correct! 👍 Now, thinking about edge cases — can you think of any specific inputs that might be tricky for your solution to handle?

**Cameron:** yea if all the items are the same, then my include adjacent funciton needs to traverse the whole list even though we already traversed it. so that makes the solution time complexity 2*n, where if I used some extra space to store adjacent characters to current in the first traversal we could get that down to just n

**Interviewer:** Good thinking! 🎯 That's a valid observation about the repeated traversal. Now, thinking about the correctness of your solution — can you think of a case where your includeAdjacent function might return incorrect results?

**Interviewer:** ⏰ You still have 15minutes remaining. Focus on addressing the core requirements of the problem if you haven't already.

**Cameron:** includeAdjacent looks pretty good to me.

**Interviewer:** Fair enough! 🤔 Let me ask you this — what happens in your solution when the differing character is at the very beginning of str1 (index 0)? Does your includeAdjacent function handle that correctly?

**Cameron:** guess the question is on for this for loop, if the initial value of the variable does not return true for the condition of the for loop in the middle argument is the loop just skipped or does it run once? I thought it was skipped so we would be fine and it would work as expected: `for (int i = first - 1; i >= 0; i--) {`

**Interviewer:** You're correct! 🎯 In Java, if the initial condition is false (e.g., `first - 1 = -1`, so `-1 >= 0` is false), the loop is skipped entirely. So your includeAdjacent function handles that edge case correctly!

**Interviewer:** That's all the questions I had for you, you can end the interview or ask me any doubt you may have. 😊

**Cameron:** would it improve my score if I edited my code to track adjacent during the initial traversal in order to save time complexity? or is just describing that enough for now?

**Interviewer:** For the purposes of this interview, describing the optimization clearly and understanding the trade-offs is valuable! 💡 Implementing it would definitely demonstrate stronger coding skills, but it's your call on how you'd like to spend the remaining time.😊

**Cameron:** ill cut in the middle and just make the solution O(n) instead of O(nlogn) by storing the solution in a linkedList, and adding items at the front when traversing backwards

**Interviewer:** Sounds good! Go ahead and implement it.💪

**Interviewer:** I see that you are making some progress in the code, you can brainstorm your approach or ask me anything in case you get stuck.🤔

**Interviewer:** It looks like there may be a compilation error — notice that your includeAdjacent method signature now takes a `LinkedList<Integer>` but you're calling it with a `List<Integer>` in some places. Can you check that?🧐

**Interviewer:** I see all the sample test cases have passed, lets move on and try Submit Code to run across the hidden test cases.

**Interviewer:** I see all the test cases have passed, I have a couple of questions for you, are you ready?

**Cameron:** yes

**Interviewer:** Great! 🎯 Now that you've optimized to use a LinkedList to avoid the sort, what is the final time and space complexity of your updated solution?

**Interviewer:** ⏰5 minutes remaining. Time to wrap up your solution and prepare to discuss your approach.

**Cameron:** final time complexity is big O(n) with more precise time complexity being 2n because we traverse one to find mismatch, and then once more to find adjacent. space complexity is only Big O(n) or just n, because we only populate the list of adjacent elements once.

**Interviewer:** Correct! 🎯 Well done on the analysis. The 2n simplifies to O(n) which is the optimal time complexity for this problem since you need to at least read both strings once.

**Interviewer:** That's all the questions I had for you, you can end the interview or ask me any doubt you may have.😊

**Interviewer:** Good job completing the interview. You can now ask me any questions about the feedback you received or click on the 'Practice Challenge' button to continue solving the question.

---

## Final Submitted Code

```java
import java.io.*;
import java.math.*;
import java.security.*;
import java.text.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.regex.*;
import java.util.stream.*;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.toList;


class Result {

    /*
     * Complete the 'getRemovableIndices' function below.
     *
     * The function is expected to return an INTEGER_ARRAY.
     * The function accepts following parameters:
     *  1. STRING str1
     *  2. STRING str2
     */

    public static List<Integer> getRemovableIndices(String str1, String str2) {
    // Step 1 step through str1 and str2 until you find a character that is different between the two OR you get to the end of str 2
        LinkedList<Integer> solution = new LinkedList<Integer>();
        List<Integer> notFound = new ArrayList<>();
        notFound.add(-1);
        int offset = 0;

        for (int i = 0; i < str2.length(); i++) {
            char char1 = str1.charAt(i + offset);
            char char2 = str2.charAt(i);
            if (char1 == char2) {
                continue;
            }
            // if we get here we've found a character that is different.
            // let's just add it's index to solution
            if (offset != 0) {
                return notFound;
            }
            solution.add(i);
            offset = 1;
            if (str1.charAt(i + 1) != str2.charAt(i)) {
                return notFound;
            }
        }
        if (solution.isEmpty()) {
            // str2 matches first length - 1 characters of str1
            LinkedList<Integer> list = new LinkedList<Integer>();
            list.add(str1.length() - 1);
            list = includeAdjacent(list, str1);
            return list;
        } else {
            solution = includeAdjacent(solution, str1);
        }

        return solution;
    }

    private static LinkedList<Integer> includeAdjacent(LinkedList<Integer> list, String str1) {
        int first = list.get(0);
        char toMatch = str1.charAt(first);
        for (int i = first - 1; i >= 0; i--) {
            if (str1.charAt(i) == toMatch) {
                list.addFirst(i);
            } else {
                break;
            }
        }
        for (int i = first + 1; i < str1.length(); i++) {
            if (str1.charAt(i) == toMatch) {
                list.add(i);
            } else {
                break;
            }
        }
        return list;
    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));

        String str1 = bufferedReader.readLine();

        String str2 = bufferedReader.readLine();

        List<Integer> result = Result.getRemovableIndices(str1, str2);

        System.out.println(
            result.stream()
                .map(Object::toString)
                .collect(joining("\n"))
        );

        bufferedReader.close();
    }
}
```

---

## HackerRank Feedback

**Overall rating: 4/5**

*A copy of this feedback was sent to the candidate by email.*

### Code Quality — 4/5

Overall, your code was clean, well-structured, and readable. You made good use of
helper methods like `includeAdjacent` to separate concerns, which is a strong
software engineering practice. Your variable naming was clear and your comments
were helpful in guiding the reader through your logic. You also proactively
optimized your solution from O(n log n) to O(n) by switching from an ArrayList
with a sort to a LinkedList with `addFirst`, which demonstrates good attention to
code quality. One area for improvement is the `notFound` list returning -1 — it
would be cleaner to return an empty list or throw an exception to signal no valid
removal exists, as returning -1 in a list is a bit of an unconventional pattern.
Additionally, the `solution.isEmpty()` branch handles the case where str2 matches
the first (n-1) characters of str1, but this logic could be made more explicit
with a comment or a named method to improve readability further. Overall a strong
effort with minor areas to polish.

### Problem Solving — 4/5

You demonstrated solid problem solving skills throughout the interview. You
correctly identified the core approach — stepping through both strings
simultaneously, finding the first mismatch, and then collecting all adjacent
identical characters as valid removable indices. You also proactively identified
the optimization opportunity to go from O(n log n) to O(n) without being prompted,
which is a strong signal of engineering maturity. You needed no hints during the
implementation and correctly reasoned through edge cases like the differing
character being at index 0 and all characters being the same. One area to improve
is to think through edge cases more systematically upfront before coding — for
example, explicitly listing out cases like empty strings, single character
strings, or strings that are already equal before diving into implementation.
This would make your problem solving process even more rigorous in a real
interview setting.

### Language Proficiency — 4/5

You demonstrated a good command of Java throughout the interview. You correctly
used `LinkedList` with `addFirst` to optimize the solution, which shows
familiarity with the Java Collections framework. You also correctly used
`Collections.sort` when needed and properly handled generics with `List<Integer>`
and `LinkedList<Integer>`. One area for improvement is to be more familiar with
standard Java utility methods without needing to ask — for example, needing to ask
about `Collections.sort` is a minor gap for a staff-level engineer. You could
strengthen your Java proficiency by reviewing the Java Collections framework more
thoroughly, including methods available on `Deque`, `LinkedList`, and
`ArrayDeque`. Also, consider using `Deque<Integer> solution = new LinkedList<>()`
as the declared type to program to the interface rather than the implementation,
which is a Java best practice. Overall a solid Java performance with minor areas
to polish.

### Technical Communication — 4/5

Your technical communication was strong throughout the interview. You clearly
articulated your approach before coding, explained your reasoning for the
optimization, and correctly analyzed time and space complexity. You communicated
the trade-offs between your initial O(n log n) solution and the optimized O(n)
solution clearly. One area for improvement is to be more precise when discussing
complexity — for example, distinguishing between the number of traversals (2n) and
the overall Big O complexity (O(n)) is good, but you could go further by also
discussing the constant factors and why they don't affect the asymptotic
complexity. Additionally, when you identified the edge case about all characters
being the same causing a full traversal in `includeAdjacent`, you could have been
more explicit about how your optimization addresses or doesn't fully address that
case. Overall, very good communication with minor room for more precision.
