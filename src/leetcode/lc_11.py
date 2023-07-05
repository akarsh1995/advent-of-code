# 11. Container With Most Water
# Medium

# You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

# Find two lines that together with the x-axis form a container, such that the container contains the most water.

# Return the maximum amount of water a container can store.

# Input: height = [1,8,6,2,5,4,8,3,7]
# Output: 49
# Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.


from typing import List


def max_water_area(arr: List[int]) -> int:
    i = 0
    j = len(arr) - 1
    max_water_area = 0

    while i < j:
        area = (j - i) * min(arr[i], arr[j])
        max_water_area = max(max_water_area, area)

        if arr[i] > arr[j]:
            j -= 1
        elif arr[i] < arr[j]:
            i += 1
        else:
            i += 1
            j -= 1

    return max_water_area



def test():
    a = [1,8,6,2,5,4,8,3,7]
    assert max_water_area(a) == 49

def test2():
    a = [0] * 10
    assert max_water_area(a) == 0

def test3():
    a = [1, 1]
    assert max_water_area(a) == 1
