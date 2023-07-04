# 15. 3Sum
# Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
# Notice that the solution set must not contain duplicate triplets.



from collections import Counter

from typing import List
from bisect import bisect_left



def get_triplets(arr: List[int]) -> List[List[int]]:

    arr.sort()
    v = set() 
    zero_pos = bisect_left(arr, x=0)

    if arr[zero_pos:]:
        if arr[zero_pos] == 0:
            if arr[zero_pos+1:]:
                if arr[zero_pos + 1] == 0:
                    zero_pos += 1

    counts = Counter(arr[zero_pos:])

    for i, elem in enumerate(arr[:zero_pos]):
        for _, next_elem in enumerate(arr[(i + 1):]):
            target = (0 - (elem + next_elem))
            if next_elem == target and counts[next_elem] == 1:
                continue 
            if target in counts:
                k = [elem, next_elem, target]
                k.sort()
                v.add(tuple(k))
    return list(map(list, v)) 


def test():
    arr = [-1,0,1,2,-1,-4]
    assert get_triplets(arr) == [[-1, 0, 1], [-1, -1, 2]]

def test2():
    arr = [0,1,1]
    assert get_triplets(arr) == []

def test3():
    arr = [0,0,0]
    assert get_triplets(arr) == [[0, 0, 0]]

def test4():
    arr = [0,-1,0, -1, 2]
    assert get_triplets(arr) == [[-1, -1, 2]]

def test5():
    arr = [1, 1, -2]
    assert get_triplets(arr) == [[-2, 1, 1]]

def test6():
    arr = [1, 2, -2, -1]
    assert get_triplets(arr) == []

def test7():
    arr = [-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0]
    assert get_triplets(arr) == [[-4, 1, 3], [-2, 1, 1], [0, 0, 0], [-2, -2, 4], [-4, 0, 4], [-5, 1, 4]]

def test8():
    arr = [-1,-1,-1,1]
    assert get_triplets(arr) == []
