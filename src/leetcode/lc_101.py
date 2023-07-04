# 101. Symmetric Tree
# Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

# Definition for a binary tree node.
from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def are_symmetric(node1: Optional[TreeNode], node2: Optional[TreeNode]) -> bool:
    if node1 and not node2:
        return False
    elif not node1 and node2:
        return False
    elif not node1 and not node2:
        return True
    else:
        return node1.val == node2.val and are_symmetric(node1.right, node2.left) and are_symmetric(node1.left, node2.right)


def test_compare():
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.left.left = TreeNode(3)
    root1.left.right = TreeNode(4)

    root1.right = TreeNode(2)
    root1.right.right = TreeNode(3)
    root1.right.left = TreeNode(4)

    assert are_symmetric(root1.left, root1.right)

    root1 = TreeNode(1)
    root1.left = TreeNode(5)
    root1.left.left = TreeNode(3)
    root1.left.right = TreeNode(4)

    root1.right = TreeNode(2)
    root1.right.right = TreeNode(3)
    root1.right.left = TreeNode(4)

    assert not are_symmetric(root1.left, root1.right)

def test_falling():
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.left.left = TreeNode(2)

    root1.right = TreeNode(2)
    root1.right.left = TreeNode(2)

    assert not are_symmetric(root1.left, root1.right)

def test_123():
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.right= TreeNode(3)

    assert not are_symmetric(root1.right, root1.left)
