# 118. Pascal's Triangle
# Given an integer numRows, return the first numRows of Pascal's triangle.
# In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:


from typing import List


def generate(n: int) -> List[List[int]]:
    v = [[1]]
    for _ in range(n - 1):
        new_v = [0, *v[-1], 0]
        to_append = [x + y for x, y in zip(new_v[:-1], new_v[1:])]
        v.append(to_append)
    return v


def test():
    assert (generate(2)) == [[1], [1, 1]]
    assert (generate(3)) == [[1], [1, 1], [1, 2, 1]]
