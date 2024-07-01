// https://leetcode.com/problems/two-sum/description/
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]


pub struct Solution;

fn main () {
  let nums = [2,7,11,15];
  let target = 9;
  let result = Solution::two_sum(nums.to_vec(), target);
  println!("Result: {:?}", result);
}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for (i, num) in nums.iter().enumerate() {
      println!("num: {}, i: {}", num, i);
      let complement = target - num;
      println!("complement: {}", complement);
      if map.contains_key(&complement) {
        println!("map.get(&complement): {:?}", map.get(&complement).unwrap());
        return vec![*map.get(&complement).unwrap() as i32, i as i32];
      }
      map.insert(num, i);
      println!("map: {:?}", map);
    }
    vec![]
  }
}