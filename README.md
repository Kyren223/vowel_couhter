
This is my (over engineered) attempt at a vowel counter
Based on "Sloth Bytes" by the coding sloth
Newspaper https://slothbytes.beehiiv.com/subscribe

## Details and instructions
### How Many Vowels?

Create a function that takes a string and returns the number (count) of vowels in the string.

### Examples

count_vowels("Celebration") ➞ 5 # 5 vowels
count_vowels("Palm") ➞ 1 # 1 vowel
count_vowels("Prediction") ➞ 4 # 4 vowels

### Notes

- a, e, i, o, u are considered vowels (not y).
- All test cases are one word and only contain letters

# My Results / conclusion:
![image](https://github.com/Kyren223/vowel_couhter/assets/52241860/9b72839a-05b7-4c77-946f-45de36e0d4d4)

* Note, when saying "identical" I mean with a
  small variation that is not significant enough to be a factor.
  this is usually caused by memory layout differences, which can be ignored and considered "identical".

1. Standard for loop on a &str - pretty fast for most use cases
2. Functional approach on a &str - sometimes was ~20% faster than 1, sometimes it was identical
3. Meme, hardcoded it, 0.25ns let's go
4. unsafe get_unchecked on a &[char] - Around 2x faster than 1/2
5. unsafe get_unchecked on a &[u8] - Identical to 4
6. branch-less 5 by casting to a number - Around 1.5x faster than 1/2 or 50% slower than 4
   * My assumption is that having a branch is more efficient than just adding a conditional
     statement, my hypothesis is due to speculative execution, but I'm not sure.
7. 5 but with half the comparisons - Identical to 5
   * Assumes the first letter is always capitalized and the rest are lowercase
     this allows only comparing 5 characters instead of 10, seems to have no impact on performance

# So which should I use?
If you need a BLAZINGLY FAST vowel counter (for whatever reason):
Your best bet is option 5 for ascii-only or 4 for utf8.

Most likely you won't need a super fast vowel counter:
I'd recommend option 1 or 2 for the best readability.
