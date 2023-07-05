# 128. Longest Consecutive Sequence

# Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
# You must write an algorithm that runs in O(n) time.

# Example 1:

# Input: nums = [100,4,200,1,3,2]
# Output: 4
# Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

# Example 2:

# Input: nums = [0,3,7,2,5,8,4,6,0,1]
# Output: 9

from typing import List
from itertools import count


def n_consecutives(arr: List[int]):
    s = set(arr)

    max_consecutives = 0

    while s:
        num  = s.pop()
        consecutives = 1

        for i in count(1):
            if num - i in s:
                s.remove(num - i)
                consecutives += 1
            else:
                break

        for i in count(1):
            if num + i in s:
                s.remove(num + i)
                consecutives += 1
            else:
                break

        max_consecutives = max(max_consecutives, consecutives)
    return max_consecutives


def test():
    a = [1, 2,  3, 4, 5, 8]
    assert n_consecutives(a) == 5


def test2():
    a = [100,4,200,1,3,2]
    assert n_consecutives(a) == 4

def test3():
    a = [0,3,7,2,5,8,4,6,0,1]
    assert n_consecutives(a) == 9
