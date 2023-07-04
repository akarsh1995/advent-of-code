# 16. 3Sum Closest
# Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
# Return the sum of the three integers.
# You may assume that each input would have exactly one solution.

# Example 1:

# Input: nums = [-1,2,1,-4], target = 1
# Output: 2
# Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

from typing import List


def get_closest_sum_to_target(arr: List[int], target: int) -> int:
    arr.sort()

    closest_three_sum = 10**5
    max_diff = 10**5

    for i, fixed in enumerate(arr):
        l = i + 1
        h = len(arr) - 1


        while l < h:
            low_val, high_val = arr[l], arr[h]

            three_sum = (fixed + low_val + high_val)
            diff = abs(target - three_sum)
            if diff == 0:
                return three_sum

            if diff < max_diff:
                max_diff = diff
                closest_three_sum = three_sum

            if three_sum > target:
                h -= 1
            else:
                l += 1

    return closest_three_sum


def test():
    arr = [-1,2,1,-4]
    target = 1
    assert get_closest_sum_to_target(arr, target) == 2

def test2():
    arr = [0,0,0]
    target = 1
    assert get_closest_sum_to_target(arr, target) == 0

def test3():
    arr = [-1,-2,-3, -4]
    target = 4
    assert get_closest_sum_to_target(arr, target) == -6

def test4():
    arr = [-2, 0, 3, 4]
    target = 0
    assert get_closest_sum_to_target(arr, target) == 1

def test5():
    arr = [4,0,5,-5,3,3,0,-4,-5]
    target = -2
    assert get_closest_sum_to_target(arr, target) == -2
