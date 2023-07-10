# 102. Binary Tree Level Order Traversal
# Medium

# Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

# Example 1:

# Input: root = [3,9,20,null,null,15,7]
# Output: [[3],[9,20],[15,7]]

# Example 2:

# Input: root = [1]
# Output: [[1]]

# Example 3:

# Input: root = []
# Output: []


from collections import deque
from typing import List, Optional

from tree import TreeNode, create_test_tree_from_lc_string

def traverser(node: Optional[TreeNode]):
    if not node:
        return []

    stack = deque([node])
    vals = [] # 3 9

    while stack:
        l = []
        for elem in stack:
            if elem:
                l.append(elem.val)

        vals.append(l)

        for _ in range(len(stack)):
            elem = stack.popleft()

            if not elem:
                continue

            if elem.left:
                stack.append(elem.left)
            if elem.right:
                stack.append(elem.right)

    return vals


def test():
    root = create_test_tree_from_lc_string('[3,9,20,null,null,15,7]')
    assert traverser(root) == [[3],[9,20],[15,7]]
