# 543. Diameter of Binary Tree
# Easy

# Given the root of a binary tree, return the length of the diameter of the tree.

# The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

# The length of a path between two nodes is represented by the number of edges between them.

# Example 1:

# Input: root = [1,2,3,4,5]
# Output: 3
# Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].

# Example 2:

# Output: 1
# Input: root = [1,2]

from tree import create_test_tree_from_lc_string, TreeNode
from typing import List, Optional


def height(node: Optional[TreeNode], max_dia_so_far: List[int]) -> int:
    if not node:
        return 0

    lh = height(node.left, max_dia_so_far)
    rh = height(node.right, max_dia_so_far)
    max_dia_so_far[0] = max(max_dia_so_far[0], lh + rh)
    return 1 + max(lh, rh)

def dia(root: Optional[TreeNode]):
    if not root:
        return 0
    max_dia_so_far: List[int] = [0]
    height(root, max_dia_so_far)
    return max_dia_so_far[0]


def test():
    tree = create_test_tree_from_lc_string('[4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]')
    assert dia(tree) == 8
