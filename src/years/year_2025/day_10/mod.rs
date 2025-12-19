#[doc = include_str!(concat!(env!("OUT_DIR"), "/aoc_2025_10_1.html"))]
pub mod part_1;
/// This part was the hardest by far for me.
/// Fortunately, [this reddit post](http://reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)
/// was extremely helpful. They posted the following python program that was adapted to rust:
/// <details>
/// <summary>Original Python program</summary>
///
/// ```python
/// from functools import cache
/// from itertools import combinations, product
/// import aocd
///
/// def patterns(coeffs: list[tuple[int, ...]]) -> dict[tuple[int, ...], dict[tuple[int, ...], int]]:
///     num_buttons = len(coeffs)
///     num_variables = len(coeffs[0])
///     out = {parity_pattern: {} for parity_pattern in product(range(2), repeat=num_variables)}
///     for num_pressed_buttons in range(num_buttons+1):
///         for buttons in combinations(range(num_buttons), num_pressed_buttons):
///             pattern = tuple(map(sum, zip((0,) * num_variables, *(coeffs[i] for i in buttons))))
///             parity_pattern = tuple(i%2 for i in pattern)
///             if pattern not in out[parity_pattern]:
///                 out[parity_pattern][pattern] = num_pressed_buttons
///     return out
///
/// def solve_single(coeffs: list[tuple[int, ...]], goal: tuple[int, ...]) -> int:
///     pattern_costs = patterns(coeffs)
///     @cache
///     def solve_single_aux(goal: tuple[int, ...]) -> int:
///         if all(i == 0 for i in goal): return 0
///         answer = 1000000
///         for pattern, pattern_cost in pattern_costs[tuple(i%2 for i in goal)].items():
///             if all(i <= j for i, j in zip(pattern, goal)):
///                 new_goal = tuple((j - i)//2 for i, j in zip(pattern, goal))
///                 answer = min(answer, pattern_cost + 2 * solve_single_aux(new_goal))
///         return answer
///     return solve_single_aux(goal)
///
/// def solve(raw: str):
///     score = 0
///     lines = raw.splitlines()
///     for I, L in enumerate(lines, 1):
///         _, *coeffs, goal = L.split()
///         goal = tuple(int(i) for i in goal[1:-1].split(","))
///         coeffs = [[int(i) for i in r[1:-1].split(",")] for r in coeffs]
///         coeffs = [tuple(int(i in r) for i in range(len(goal))) for r in coeffs]
///
///         subscore = solve_single(coeffs, goal)
///         print(f'Line {I}/{len(lines)}: answer {subscore}')
///         score += subscore
///     print(score)
///
/// # solve(open('input/10.test').read())
/// solve(aocd.get_data(year=2025, day=10))
/// ```
///
/// </details>
///
/// Here is also the complete reddit post saved for posterity:
///
/// <details>
/// <summary>Complete Reddit post</summary>
///
/// # [2025 Day 10 (Part 2)] Bifurcate your way to victory!
///
/// Here's an approach for Part 2 that, to my surprise, I haven't seen *anyone* else use. (Sorry if someone's posted about it already; I did a quick scan of the subreddit and asked a few of my friends, and none of them had seen this approach.) It doesn't rely on sledgehammers like Z3 or scipy, it doesn't require you to know or implement linear algebra, and it doesn't use potentially-risky heuristics. The best part? If you're reading this, you've might've coded part of it already!
///
/// So, what's the idea? In fact, the idea is to use Part 1!
///
/// Here's a quick tl;dr of the algorithm. If the tl;dr makes no sense, don't worry; we'll explain it in detail. (If you're only interested in code, that's at the bottom of the post.)
///
/// >**tl;dr**: find all possible sets of buttons you can push so that the remaining voltages are even, and divide by 2 and recurse.
///
/// Okay, if none of that made any sense, this is for you. So how is Part 1 relevant? You've solved Part 1 already (if you haven't, why are you reading this...?), so you've seen the main difference:
///
/// * In part 2, the joltage counters can count 0, 1, 2, 3, 4, 5, ... to infinity.
/// * In part 1, the indicator lights can toggle off and on. While the problem wants us to think of it as toggling, we can also think of it as "counting:" the lights are "counting" off, on, off, on, off, on, ... to infinity.
///
/// While these two processes might seem very different, they're actually quite similar! The light is "counting" off and on based on the *parity* (evenness or oddness) of the joltage.
///
/// How can this help us? While Part 2 involves changing the joltages, we can imagine we're *simultaneously* changing the indicator lights too. Let's look at the first test of the sample data (with the now-useless indicator lights removed):
///
/// ```text
/// (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
/// ```
///
/// We need to set the joltages to 3, 5, 4, 7. If we're also toggling the lights, where will the lights end up? Use parity: 3, 5, 4, 7 are odd, odd, even, odd, so the lights must end up in the pattern `[##.#]`.
///
/// Starting to look familiar? Feels like Part 1 now! What patterns of buttons can we press to get the pattern `[##.#]`?
///
/// Here's where your experience with solving Part 1 might come in handy -- there, you might've made the following observations:
///
/// * The order we press the buttons in doesn't matter.
/// * Pressing a button twice does nothing, so in an optimal solution, every button is pressed 0 or 1 time.
///
/// Now, there are only 2^(6) = 64 choices of buttons to consider: how many of them give `[##.#]`? Let's code it! (Maybe you solved this exact type of problem while doing Part 1!) There are 4 possibilities:
///
/// 1. Pressing `{3}, {0, 1}`.
/// 2. Pressing `{1, 3}, {2}, {0, 2}`.
/// 3. Pressing `{2}, {2, 3}, {0, 1}`.
/// 4. Pressing `{3}, {1, 3}, {2, 3}, {0, 2}`.
///
/// Okay, cool, but now what? Remember: *any* button presses that gives joltages 3, 5, 4, 7 also gives lights `[##.#]`. But keep in mind that pressing the same button twice cancels out! So, if we know how to get joltages 3, 5, 4, 7, we know how to get `[##.#]` by pressing each button at most once, and in particular, that button-press pattern will match one of the four above patterns.
///
/// Well, we showed that if we can solve Part 2 then we can solve Part 1, which doesn't seem helpful... but we can flip the logic around! The *only* ways to get joltages of 3, 5, 4, 7 are to match one of the four patterns above, plus possibly some redundant button presses (where we press a button an even number of times).
///
/// Now we have a strategy: use the Part 1 logic to figure out which patterns to look at, and examine them one-by-one. Let's look at the first one, pressing `{3}, {0, 1}`: suppose our mythical 3, 5, 4, 7 joltage presses were modeled on that pattern. Then, we know that we need to press `{3}` once, `{0, 1}` once, and then every button some even number of times.
///
/// Let's deal with the `{3}` and `{0, 1}` presses now. Now, we have remaining joltages of 2, 4, 4, 6, and we need to reach this by pressing every button an even number of times...
///
/// ...huh, everything is an even number now. Let's simplify the problem! By cutting everything in half, now we just need to figure out how to reach joltages of 1, 2, 2, 3. Hey, wait a second...
///
/// ...this is the same problem (but smaller)! Recursion! We've shown that following this pattern, if the minimum number of presses to reach joltages of 1, 2, 2, 3 is `P`, then the minimum number of presses to reach our desired joltages of 3, 5, 4, 7 is `2 * P + 2`. (The extra plus-two is from pressing `{3}` and `{0, 1}` once, and the factor of 2 is from our simplifying by cutting everything in half.)
///
/// We can do the same logic for all four of the patterns we had. For convenience, let's define `f(w, x, y, z)` to be the fewest button presses we need to reach joltages of w, x, y, z. (We'll say that `f(w, x, y, z) = infinity` if we can't reach some joltage configuration at all.) Then, our `2 * P + 2` from earlier is `2 * f(1, 2, 2, 3) + 2`. We can repeat this for all four patterns we found:
///
/// 1. Pressing `{3}, {0, 1}`: this is `2 * f(1, 2, 2, 3) + 2`.
/// 2. Pressing `{1, 3}, {2}, {0, 2}`: this is `2 * f(1, 2, 1, 3) + 3`.
/// 3. Pressing `{2}, {2, 3}, {0, 1}`: this is `2 * f(1, 2, 1, 3) + 3`.
/// 4. Pressing `{3}, {1, 3}, {2, 3}, {0, 2}`: this is `2 * f(1, 2, 1, 2) + 4`.
///
/// Since every button press pattern reaching joltages 3, 5, 4, 7 has to match one of these, we get `f(3, 5, 4, 7)` is the minimum of the four numbers above, which can be calculated recursively! While descending into the depths of recursion, there are a few things to keep in mind.
///
/// * If we're calculating `f(0, 0, 0, 0)`, we're done: no more presses are needed. `f(0, 0, 0, 0) = 0`.
/// * If we're calculating some `f(w, x, y, z)` and there are no possible patterns to continue the recursion with, that means joltage level configuration w, x, y, z is impossible -- `f(w, x, y, z) = infinity`. (Or you can use a really large number. I used 1 000 000.)
/// * Remember to not allow negative-number arguments into your recursion.
/// * Remember to cache!
///
/// And there we have it! By using our Part 1 logic, we're able to set up recursion by dividing by 2 every time. (We used a four-argument `f` above because this line of input has four joltage levels, but the same logic works for any number of variables.)
///
/// This algorithm ends up running surprisingly quickly, considering its simplicity -- in fact, I'd been vaguely thinking about this ever since I saw Part 2, as well as after [I solved it in the most boring way possible (with Python's Z3 integration)](https://www.youtube.com/watch?v=zU-uJH3RF4c), but I didn't expect it to work so quickly. I expected the state space to balloon quickly like with other searching-based solutions, but that just... doesn't really happen here.
///
/// [Here's my Python code.](https://topaz.github.io/paste/#XQAAAQAQBgAAAAAAAAAzHIoib6poHLpewxtGE3pTrRdzrponKxDhfDpmpp1XOH9xnlIyXvIsci+yi/TEZZMKgfYR14I78p/pM5oOH7+L0m0X2/gBRIrqWEe011eodNWzpV5VknmseU1JuE4IcaE99AKYqGRdhO4xV27iJI5YsaOGSB3jAQLF+IoBwuA6I+KJTgil0Vy0U/Znn/1dcUQPGYrVKCEx9ahoJWuTWOB0qUnOXIJuoRWIV1H9lwwx3jU9Xvpgtq/mY3Mkx2pdWdsJ2ErQv7RvgEHZDvy8sSeAY6e1tcU4SHwXfWCH40hQLI1TQvDrAqSytaIZM9wRq9C9apZme2UNp+vpD+ytDFUoMuDLTL1fBFz7z9dlyPh63XLIPYUwNSkFTjyv6Zu6mg3UcKZTa0IbhvkPzPgLxQak0kiIF6xzmBz8YiUSD+27zJSEv6WklIVwb/zyvwR0AJk4/9C/nKhcViUezzVBRFi/5VdHm5EeUfPnwZnHwBrgjPix1AkFjTIvYXBCxmsGaAKxjlj3agCG/CAhEuPRI89ENMT9LwiRzCRqccwOmJFPqncDK6igCgggf2mhdXTpKOA91oX9bhdRhmDhAr7M3+bfsczTSBqmWVr0piKPwmB8Cf2cB8PBlyuFVq322PxqjBoMP79pPh8vN2U+sb2IIXs4Lv19crhlrFEzpkZbtfyIQde9W6AaSn1VK6FmE577MS1ML2cFc4D0j0HvqEAZ550Qhdr9acM7UnkvTv4sExEve5TgWWZALoCoNAPV7EAMvhw8VT9Ae6r88NaOjDo/Mfqf9X7d2GeaQ1jmNzyiwDbWSQJTMKVskX2V7QxJSiRcHq7PokPyQZhUGg438pnsiuqR9H8pKfjd7pzNm1mui1vSsAojoOQx3yICXM4IiuiY/+nKRCfX7kA6Zz5+0+0aU+/86b8F) (I use [advent-of-code-data](https://github.com/wimglenn/advent-of-code-data) to auto-download my input -- feel free to remove that import and read input some other way.) On my computer, I got times of \~7s on python and \~2.5s on pypy3 on my real input, which I think are perfectly acceptable speeds for such a difficult problem.
///
/// EDIT: u/DataMn (and likely others -- sorry if I missed you!) mention the optimization of grouping the possible patterns by parity, so that we don't need to search through the entire dict of `pattern_costs`. This cuts down the runtime a lot -- I now get runtimes of \~0.6s with python and \~1.5s with pypy3. [My code incorporating this optimization (as well as some minor stylistic tweaks) is here.](https://topaz.github.io/paste/#XQAAAQDrBgAAAAAAAAAzHIoib6poHLpewxtGE3pTrRdzrponKxDhfDpmpp1XOH9xnlIyXvIsci+yi/TTFy44FGq6ZrL5OGunysUd322wy+hc3ZIsGd8pNfizbHiJBJTwZuKTJfFD2uUHnzBwP+u/d/PLktBiYiqXhh1rLe8pUTd4hRgQ7Y7ZnPiYgWE25rG2G/K82KYb/v3eDZYBSqI6WDTw/KZ12Dc6FqQLlurOLmsFXKRqb7yL8I8sTp9GTt2rfbMrhrR7UlhjBxofh5Ckk4hXPfRc/R87qV/BXrRJFgFbvPjBlT03fVct8umxOsqUTZ0nT7hYZl0wGUxgeOty+QYL51kUz7Jh0+LwJz28zABJLSt4UoP/08Oei2An6Y1i7Z/d7tmq1TE/qp3ZUSUTcjpJHmwOb9bGYuS9ryexTqHm6rXlEzZyiR8LjrqEDglnSy+YbNGxN2bbfvYPPco3xwCryYbgxUQ+LP53awgvEkk+We0/iyJCHhS7k3s9KLf9SkeB7/aXFoRQoHzrlkzme18oufdVmq+7hJe2xK2Z0Vyj11XfERvYggQXUIuwEbMKJWgp6jVgaoc9yXLHeaz1O+E8ECvRY5GerpLRyK0ywx4j0ItPHMWvkcySQyfoJjD+oHHVSzsrkggtm2szpnyD4QPzu0Cj4IzdkzkjA+9RYKpumjThjZIRU1tpoNVq/llLE+51NJkIUr7SRiLP5okgfiUXiJs6bd3lPfR/pwaO3iqTyVcHL/4/tOLI2Cz5lU0anKFSl9Mm6AEUTRIMfFmtD+JMwOFK226DXSLfjaopjJfI1sg4TmSJ7AjgQQOjCDKSVIvILlpaYliEsTDEg/q9vvOfpLfkVFP2WI0dt2raqrR9h44Q3NaKwKnFNmLV1yIunwwvEecBn5wyaG8Zr5JlF38jcGtGWBHYWVvmf23ByFxPZTgUDC8a1Fa967eX9K3+hB7gR8LgjOyS+DkfbLSCAqqPckeyD9PFfVH9KNAvQtTjZ/kmits4YjOhdIX/6G3UXw==)
///
/// Sure, it might not be competing with the super-fast custom-written linear algebra solutions, but I'm still proud of solving the problem this way, and finding this solution genuinely redeemed the problem in my eyes: it went from "why does this problem exist?" to "*wow*." I hope it can do the same for you too.
///
/// [permalink](http://reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)
/// by *tenthmascot* (↑ 449/ ↓ 0)
///
/// </details>
///
/// ---
///
#[doc = include_str!(concat!(env!("OUT_DIR"), "/aoc_2025_10_2.html"))]
pub mod part_2;
