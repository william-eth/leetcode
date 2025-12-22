// https://leetcode.com/problems/find-all-anagrams-in-a-string/description/

// Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:
// Input: s = "cbaebabacd", p = "abc"
// Output: [0,6]
// Explanation:
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".

// Example 2:
// Input: s = "abab", p = "ab"
// Output: [0,1,2]
// Explanation:
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".



pub struct Solution;

fn main () {
  let s = "cbaebabacd".to_string();
  let p = "abc".to_string();
  let result = Solution::find_anagrams(s, p);
  println!("Result: {:?}", result);
}

impl Solution {
  pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut result = Vec::new();
    let mut s_chars = vec![0; 26];
    let mut p_chars = vec![0; 26];
    let s = s.as_bytes();
    let p = p.as_bytes();
    let s_len = s.len();
    let p_len = p.len();

    // println!("result: {:?}", result);
    // println!("s_chars: {:?}, p_chars: {:?}", s_chars, p_chars);
    // println!("s: {:?}, p: {:?}", s, p);
    // println!("s_len: {}, p_len: {}", s_len, p_len);
    // println!("-------------------");

    if s_len < p_len {
      return result;
    }

    for i in 0..p_len {
      // println!("p_len start: {}", i);
      s_chars[(s[i] - b'a') as usize] += 1;
      p_chars[(p[i] - b'a') as usize] += 1;
      // println!("s_chars: {:?}, p_chars: {:?}", s_chars, p_chars);
      // println!("-------------------");
    }

    if s_chars == p_chars {
      // println!("s_chars == p_chars");
      result.push(0);
      // println!("result: {:?}", result);
      // println!("-------------------");
    }

    for i in p_len..s_len {
      // println!("p_len .. s_len");

      // println!("s_chars before: {:?}", s_chars);
      s_chars[(s[i] - b'a') as usize] += 1;
      // println!("s_chars after +=1: {:?}", s_chars);
      // println!("i: {}, p_len: {}, i - p_len: {}", i, p_len, i - p_len);
      // println!("s[i - p_len]: {}", s[i - p_len]);
      s_chars[(s[i - p_len] - b'a') as usize] -= 1;
      // println!("s_chars -=1 : {:?}", s_chars);
      // println!("-------------------");

      if s_chars == p_chars {
        println!("s_chars == p_chars");
        result.push((i - p_len + 1) as i32);
      }
      println!("-------------------");
    }
    result
  }
}