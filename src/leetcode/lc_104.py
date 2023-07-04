# 101. Symmetric Tree
# Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

# Definition for a binary tree node.
from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def max_depth(node: Optional[TreeNode]) -> int:
    if not node:
        return 0
    return 1 + max(max_depth(node.left), max_depth(node.right))


def test_compare():
    root1 = TreeNode(3)
    root1.left = TreeNode(9)
    root1.right = TreeNode(20)
    root1.right.right = TreeNode(7)
    root1.right.left = TreeNode(15)
    assert max_depth(root1) == 3
