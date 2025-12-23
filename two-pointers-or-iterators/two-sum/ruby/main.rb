# https://leetcode.com/problems/two-sum/description/

# Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

# You may assume that each input would have exactly one solution, and you may not use the same element twice.

# You can return the answer in any order.

 

# Example 1:

# Input: nums = [2,7,11,15], target = 9
# Output: [0,1]
# Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
# Example 2:

# Input: nums = [3,2,4], target = 6
# Output: [1,2]
# Example 3:

# Input: nums = [3,3], target = 6
# Output: [0,1]


# dirty code but works..

# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
    result = {}
    ans = []
    nums.each_with_index do | k, index |
        # puts "#{index} round, number #{k}, result: #{result}, k == target: #{k == target}"
        if k == target
            # next if !ans.nil? && index != 0
            ans << index
            # puts "ans: #{ans}"
            ans << result[0] if result.key?(0)
        elsif index == 0
            result = { k => index }
        else
            if k >= target
                temp = k - target
                temp = -temp

                if result.key?(temp) == true
                    # puts "result.key?(-temp) is true"
                    ans = []
                    ans << result[temp]
                    ans << index
                    # ans << result[0] if result.key?(0)
                    # puts "result[target]: #{target}, ans: #{ans}" 
                    # ans.delete!(target) if result.key?(target)
                    # puts "ans: #{ans}"
                    break
                else
                    # puts "put the #{k} into result"
                    result[k] = index
                end
            else
                temp = target - k

                if result.key?(temp) == true
                    # puts "result.key?(temp) is true"
                    ans = []
                    ans << result[temp]
                    ans << index
                    # ans << result[0] if result.key?(0)
                    # ans.delete!(target) if result.key?(target)
                    # puts "ans: #{ans}"
                    break
                else
                    # puts "put the #{k} into result"
                    result[k] = index
                end
            end
        end
    end
    ans.sort
end

def main
  nums = [5,0]
  target = 5
  result = two_sum(nums, target)
  p result
end