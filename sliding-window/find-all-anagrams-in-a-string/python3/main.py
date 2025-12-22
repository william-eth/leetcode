# https://leetcode.com/problems/find-all-anagrams-in-a-string/description/

# Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
# An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

# Example 1:
# Input: s = "cbaebabacd", p = "abc"
# Output: [0,6]
# Explanation:
# The substring with start index = 0 is "cba", which is an anagram of "abc".
# The substring with start index = 6 is "bac", which is an anagram of "abc".

# Example 2:
# Input: s = "abab", p = "ab"
# Output: [0,1,2]
# Explanation:
# The substring with start index = 0 is "ab", which is an anagram of "ab".
# The substring with start index = 1 is "ba", which is an anagram of "ab".
# The substring with start index = 2 is "ab", which is an anagram of "ab".

from typing import List
from collections import Counter

class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
      result = []
      p_len = len(p)
      s_len = len(s)

      if s_len < p_len:
        return result
      
      p_counter = Counter(p)
      print(f"p_counter: {p_counter}")
      window_counter = Counter()

      for i in range(s_len):
        window_counter[s[i]] += 1
        print(f"window_counter: {window_counter}")

        if i >= p_len:
          left_char = s[i - p_len]
          window_counter[left_char] -= 1
          print(f"window_counter after -=1: {window_counter}")

          if window_counter[left_char] == 0:
            del window_counter[left_char]

        if window_counter == p_counter:
          result.append(i - p_len + 1)

      return result

if __name__ == "__main__":
  s = "cbaebabacd"
  p = "abc"
  result = Solution().findAnagrams(s, p)
  print(result)