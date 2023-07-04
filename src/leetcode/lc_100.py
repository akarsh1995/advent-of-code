# 100. Same Tree
# Given the roots of two binary trees p and q, write a function to check if they are the same or not.
# Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

# Definition for a binary tree node.
from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def inorder_compare(node: Optional[TreeNode], second: Optional[TreeNode]) -> bool:
    if node is None:
        return second is None
    if second is None:
        return node is None
    return inorder_compare(node.left, second.left) and node.val == second.val and inorder_compare(node.right, second.right)

def test_compare():
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.right = TreeNode(3)
    root2 = TreeNode(1)
    root2.left = TreeNode(2)
    root2.right = TreeNode(3)
    assert inorder_compare(root1, root2)
