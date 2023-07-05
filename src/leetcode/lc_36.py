# 36. Valid Sudoku
# Medium
# Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

#     Each row must contain the digits 1-9 without repetition.
#     Each column must contain the digits 1-9 without repetition.
#     Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

# Note:

#     A Sudoku board (partially filled) could be valid but is not necessarily solvable.
#     Only the filled cells need to be validated according to the mentioned rules.
# Input: board =
# [["5","3",".",".","7",".",".",".","."]
# ,["6",".",".","1","9","5",".",".","."]
# ,[".","9","8",".",".",".",".","6","."]
# ,["8",".",".",".","6",".",".",".","3"]
# ,["4",".",".","8",".","3",".",".","1"]
# ,["7",".",".",".","2",".",".",".","6"]
# ,[".","6",".",".",".",".","2","8","."]
# ,[".",".",".","4","1","9",".",".","5"]
# ,[".",".",".",".","8",".",".","7","9"]]
# Output: true

from collections import defaultdict
from typing import List
import numpy as np

def validate_box(arr: np.ndarray) -> bool:
    nums = [0] * 10
    arr = arr.flatten()
    for num in arr:
        if num != '.':
            nums[ord(num) - ord('0')] += 1
    return all([n == 0 or n == 1 for n in nums])

def validate(arr: List[List[str]]) -> bool:

    rows = defaultdict(set)
    columns = defaultdict(set)
    three_x_three = defaultdict(set)

    for i in range(9):
        for j in range(9):
            if arr[i][j] == '.':
                continue
            if arr[i][j] in rows[i] or arr[i][j] in columns[j]:
                return False
            if arr[i][j] in three_x_three[i // 3, j //3]:
                return False
            rows[i].add(arr[i][j])
            columns[j].add(arr[i][j])
            three_x_three[(i // 3, j // 3)].add(arr[i][j])
    return True



def test():
    a = [["5","3",".",".","7",".",".",".","."]
        ,["6",".",".","1","9","5",".",".","."]
        ,[".","9","8",".",".",".",".","6","."]
        ,["8",".",".",".","6",".",".",".","3"]
        ,["4",".",".","8",".","3",".",".","1"]
        ,["7",".",".",".","2",".",".",".","6"]
        ,[".","6",".",".",".",".","2","8","."]
        ,[".",".",".","4","1","9",".",".","5"]
        ,[".",".",".",".","8",".",".","7","9"]]
    assert validate(a)
    a[1][0] = '5'
    assert not validate(a)
