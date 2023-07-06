# 74. Search a 2D Matrix
# Medium

# You are given an m x n integer matrix matrix with the following two properties:

# Each row is sorted in non-decreasing order.
# The first integer of each row is greater than the last integer of the previous row.

# Given an integer target, return true if target is in matrix or false otherwise.

# You must write a solution in O(log(m * n)) time complexity.

# Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
# Output: true

from typing import List

def binary_search(l: int, h: int, arr: List[int], target: int) -> bool:
    m = (h + l) // 2
    if target == arr[m]:
        return True

    if l == h == m:
        return False

    if target < arr[m]:
        return binary_search(l, m , arr, target)
    elif target > arr[m]:
        return binary_search(m + 1, h, arr, target)
    return False

def binary_search_2d(l: int, h: int, arr: List[List[int]], target: int) -> bool:
    m = (h + l) // 2

    if target >= arr[m][0] and target <= arr[m][-1]:
        return binary_search(0, len(arr[m]) - 1,  arr[m], target)

    if l == h == m:
        return False

    if target < arr[m][0]:
        return binary_search_2d(l, m, arr, target)

    elif target > arr[m][-1]:
        return binary_search_2d(m + 1, h, arr, target)

    return False

def test_bin_search():
    assert binary_search(0, 3, [1, 2 , 3 , 4], 1)
    assert not binary_search(0, 3, [1, 2 , 3 , 4], 9)

    assert not binary_search(0, 3, [10, 11, 16, 20], 13)

def test():
    arr = [[1,3,5,7],[10,11,16,20],[23,30,34,60]]
    target = 3
    assert binary_search_2d(0, len(arr) - 1, arr, target)

def test1():
    arr = [[1,3,5,7],[10,11,16,20],[23,30,34,60]]
    target = 13
    assert not binary_search_2d(0, len(arr)- 1, arr, target)
