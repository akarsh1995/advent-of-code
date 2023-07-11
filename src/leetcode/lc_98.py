# 98. Validate Binary Search Tree
# Medium
# Given the root of a binary tree, determine if it is a valid binary search tree (BST).

# A valid BST is defined as follows:

#     The left
#     subtree
#     of a node contains only nodes with keys less than the node's key.
#     The right subtree of a node contains only nodes with keys greater than the node's key.
#     Both the left and right subtrees must also be binary search trees.

#

# Example 1:

# Input: root = [2,1,3]
# Output: true

# Example 2:

# Input: root = [5,1,4,null,null,3,6]
# Output: false
# Explanation: The root node's value is 5 but its right child's value is 4.



from typing import Optional
from tree import TreeNode, create_test_tree_from_lc_string


def validate(node: Optional[TreeNode], left: Optional[int] = None, right: Optional[int] = None) -> bool:
    if node is None:
        return True

    if left is None:
        left = -10**10

    if right is None:
        right = 10**10

    is_valid = left < node.val < right

    return all([ is_valid and  validate(node.left, left, node.val) and validate(node.right, node.val, right)])





def test():
    root = create_test_tree_from_lc_string('[2,1,3]')
    if root:
        assert validate(root) == True
    root = create_test_tree_from_lc_string('[5,1,4,null,null,3,6]')
    assert not validate(root)

def test2():
    root = create_test_tree_from_lc_string('[5,4,6,null,null,3,7]')
    assert not validate(root)

'''
common mistake:
    to stress more on the left side and right side passing

solution
    take only range into account like -inf < node.val < inf
'''
