# 199. Binary Tree Right Side View
# Medium

# Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

#

# Example 1:

# Input: root = [1,2,3,null,5,null,4]
# Output: [1,3,4]

# Example 2:

# Input: root = [1,null,3]
# Output: [1,3]

# Example 3:

# Input: root = []
# Output: []

from collections import deque
from typing import Optional, List
from tree import TreeNode, create_test_tree_from_lc_string

def right_v(node: Optional[TreeNode]) -> List[int]:
    if not node:
        return []
    s = deque([node])

    view = []
    while s:
        view.append(s[-1].val)

        for _ in range(len(s)):
            node = s.popleft()

            if node.left:
                s.append(node.left)

            if node.right:
                s.append(node.right)

    return view

def test():
    node = create_test_tree_from_lc_string('[1,2,3,null,5,null,4]')
    assert right_v(node) == [1, 3, 4]

'''
common mistake:
    Can forget the left half of the tree
    could also contain the node deeper than the right half.
'''
