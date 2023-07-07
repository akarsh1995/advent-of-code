# 84. Largest Rectangle in Histogram
# Hard

# Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

# Input: heights = [2,1,5,6,2,3]
# Output: 10
# Explanation: The above is a histogram where width of each bar is 1.
# The largest rectangle is shown in the red area, which has an area = 10 units.

from typing import List

def largest(arr: List[int]) -> int:

    stack = []
    area = 0

    for i, h in enumerate(arr):
        start = i

        while stack and stack[-1][1] > h:
           index, last_bar_height = stack.pop()
           area = max(area, ((i - index) * last_bar_height))
           start = index
        stack.append((start, h))

    for i, h in stack:
        area = max(area, h * (len(arr) - i))

    return area




def test():
    heights = [2,1,5,6,2,3]
    assert largest(heights) == 10

def test2():
    heights = [2, 4]
    assert largest(heights) == 4

def test3():
    heights = [1, 1]
    assert largest(heights) == 2
