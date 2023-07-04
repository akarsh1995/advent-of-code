# Leetcode TreeNode Type

from collections import deque
from typing import List, Optional
import ast
from PrettyPrint import PrettyPrintTree


class TreeNode:
    pt = PrettyPrintTree(lambda x: prep_list(x), lambda x: x.val, return_instead_of_print=True)

    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __str__(self):
        return f"\n{self.pt(self)}"

def prep_list(x: Optional[TreeNode]):
    v = []
    if x:
        if x.left:
            v.append(x.left)
        if x.right:
            v.append(x.right)
    return v

def create_test_tree_from_array(arr) -> Optional[TreeNode]:
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
        if arr:
            if arr[0]:
                node.right = TreeNode(arr[0])
                d.append(node.right)
            arr = arr[1:]

    return root


def create_test_tree_from_lc_string(s: str) -> Optional[TreeNode]:
    arr: List[Optional[int]] = ast.literal_eval(s.replace('null', 'None'))
    tree = create_test_tree_from_array(arr)
    return tree
