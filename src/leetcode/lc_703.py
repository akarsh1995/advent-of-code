# 703. Kth Largest Element in a Stream
# Easy

# Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.

# Implement KthLargest class:

#     KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
#     int add(int val) Appends the integer val to the stream and returns the element representing the kth largest element in the stream.

#

# Example 1:

# Input
# ["KthLargest", "add", "add", "add", "add", "add"]
# [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
# Output
# [null, 4, 5, 5, 8, 8]

# Explanation
# KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
# kthLargest.add(3);   // return 4
# kthLargest.add(5);   // return 5
# kthLargest.add(10);  // return 5
# kthLargest.add(9);   // return 8
# kthLargest.add(4);   // return 8


from typing import List
from bisect import bisect_left

# binary search manual implementation
def bisect(vals: List[int], val: int) -> int:

    condition = lambda m: vals[m] > val

    l = 0
    r = len(vals)

    while l < r:
        m = l + (r - l) // 2

        if condition(m):
            r = m
        else:
            l = m + 1

    return l

class KthLargest:
    def __init__(self, k: int, nums: List[int]) -> None:
        self.k = k
        self.nums = nums
        self.nums.sort()

    def add(self, val: int) -> int:
        self.nums.insert(bisect_left(self.nums, val), val)
        return self.nums[-self.k]

    # binary search manual implementation
    def bsearch(self, val: int) -> int:
        self.nums.insert(bisect(self.nums, val), val)
        return self.nums[-self.k]


def test():
    kth_largest = KthLargest(3, [4,5,8,2])
    assert kth_largest.add(3) == 4
    assert kth_largest.add(5) == 5
    assert kth_largest.add(10) == 5
    assert kth_largest.add(9) == 8
    assert kth_largest.add(4) == 8

def test2():
    kth_largest = KthLargest(3, [4,5,8,2])
    assert kth_largest.bsearch(3) == 4
    assert kth_largest.bsearch(5) == 5
    assert kth_largest.bsearch(10) == 5
    assert kth_largest.bsearch(9) == 8
    assert kth_largest.bsearch(4) == 8
