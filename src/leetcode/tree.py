# Leetcode TreeNode Type 

from collections import deque
from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def create_test_tree_from_array(arr, i, n) -> Optional[TreeNode]:
    if not arr: # array empty
        return None

    root: TreeNode = TreeNode(arr[0])
    arr = arr[1:]

    d = deque([root])
    while arr:
        node = d.popleft()
        if arr[0]:
            node.left = TreeNode(arr[0])
            d.append(node.left)
        arr = arr[1:]
        if arr[0]:
            node.right = TreeNode(arr[0])
            d.append(node.right)
        arr = arr[1:]

    return root


import ast

def create_test_tree_from_lc_string(s: str) -> Optional[TreeNode]:
    arr: List[Optional[int]] = ast.literal_eval(s.replace('null', 'None'))
    return create_test_tree_from_array(arr, 0, len(arr))
    



