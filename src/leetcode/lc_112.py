# 112. Path Sum
# Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
# A leaf is a node with no children.
# Output: true
# Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22

from typing import Optional
from tree import TreeNode, create_test_tree_from_lc_string

def trav(node: Optional[TreeNode], sum_so_far: int, target: int):
    if not node:
        return False 
    sum_so_far += node.val
    if not node.left and not node.right:
        return sum_so_far == target
    elif node.left and not node.right: 
        return trav(node.left, sum_so_far, target)
    elif not node.left and  node.right: 
        return trav(node.right, sum_so_far, target)
    else:
        return trav(node.left, sum_so_far, target) or trav(node.right, sum_so_far, target)

def test():
    root = create_test_tree_from_lc_string('[5,4,8,11,null,13,4,7,2,null,null,null,1]')
    assert trav(root, 0, 22)

def test2():
    root = create_test_tree_from_lc_string('[1,2]')
    assert not trav(root, 0, 1)

def test3():
    root = create_test_tree_from_lc_string('[1,2,3]')
    assert not trav(root, 0, 5)

