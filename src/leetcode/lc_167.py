# 167. Two Sum II - Input Array Is Sorted

# Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.

# Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

# The tests are generated such that there is exactly one solution. You may not use the same element twice.

# Your solution must use only constant extra space.


from typing import List, Tuple


def two_sum_indices(arr: List[int], tsum: int) -> Tuple[int, int]:
    i = 0
    j = len(arr) - 1

    while i < j:
        if arr[i] + arr[j] == tsum:
            return (i + 1, j + 1)
        elif arr[i] + arr[j] > tsum:
            j -= 1
        else:
            i += 1
    return (0, 0)


def test():
    a = [2,7,11,15]
    t = 9
    assert list(two_sum_indices(a, t)) == [1, 2]

def test2():
    a = [2, 3, 4]
    t = 6
    assert list(two_sum_indices(a, t)) == [1, 3]

def test3():
    a = [-1,0]
    t = -1
    assert list(two_sum_indices(a, t)) == [1, 2]
