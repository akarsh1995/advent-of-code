# 1448. Count Good Nodes in Binary Tree
# Medium

# Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.

# Return the number of good nodes in the binary tree.

#

# Example 1:

# Input: root = [3,1,4,3,null,1,5]
# Output: 4
# Explanation: Nodes in blue are good.
# Root Node (3) is always a good node.
# Node 4 -> (3,4) is the maximum value in the path starting from the root.
# Node 5 -> (3,4,5) is the maximum value in the path
# Node 3 -> (3,1,3) is the maximum value in the path.

from typing import Optional
from tree import TreeNode, create_test_tree_from_lc_string

def count_good(node: Optional[TreeNode], parent_val: int) -> int:
    if not node:
        return 0

    if node.val >= parent_val:
        return 1 + count_good(node.left, node.val) + count_good(node.right, node.val)
    else:
        return 0 + count_good(node.left, parent_val) + count_good(node.right, parent_val)


def test():
    root = create_test_tree_from_lc_string('[3,1,4,3,null,1,5]')
    assert count_good(root, -10**8) == 4

'''
common mistake:
    count_good(node, parent_val)

    forget to pass on the parent_val in case condition is not met.
    Because if condition is met current node value gets pass down to parent_val argument.
'''
