# 572. Subtree of Another Tree
# Easy

# Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.

# A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.

#

# Example 1:

# Input: root = [3,4,5,1,2], subRoot = [4,1,2]
# Output: true

from typing import List, Optional
from tree import TreeNode, create_test_tree_from_array, create_test_tree_from_lc_string


def are_equal(t1: Optional[TreeNode], t2: Optional[TreeNode]) -> bool:
    if not t1 and not t2:
        return True
    elif (not t1 and t2) or (t1 and not t2):
        return False
    return t1.val == t2.val and are_equal(t1.left, t2.left) and are_equal(t1.right, t2.right)


def is_subtree(t1: Optional[TreeNode], t2: TreeNode, ored: List[bool]):
    if not t1: # leaf
        return
    if t1.val == t2.val:
        ored.append(are_equal(t1, t2))
    is_subtree(t1.left, t2, ored)
    is_subtree(t1.right, t2, ored)

def helper(t1: TreeNode, t2: TreeNode):
    ored = []
    is_subtree(t1, t2, ored)
    return any(ored)

def test():
    t1 = create_test_tree_from_array([1, 2, 3, 4, 5, 6, 7])
    t2 = create_test_tree_from_array([1, 2, 3, 4, 5, 6, 7])

    assert are_equal(t1, t2)

def test2():
    t1 = create_test_tree_from_array([1])
    t2 = create_test_tree_from_array([1, 2, 3, 4, 5, 6, 7])

    assert not are_equal(t1, t2)

def test3():
    t1 = create_test_tree_from_array([1])
    t2 = create_test_tree_from_array([1])
    if t1 and t2:
        assert are_equal(t1, t2)

def test4():
    root = create_test_tree_from_lc_string('[3,4,5,1,2]')
    subroot = create_test_tree_from_lc_string('[4,1,2]')
    if root and subroot:
        assert helper(root, subroot)
    root = create_test_tree_from_lc_string('[3,4,5,1,2]')
    subroot = create_test_tree_from_lc_string('[1,3,8]')
    if root and subroot:
        assert not helper(root, subroot)

def test5():
    root = create_test_tree_from_array([1, 1])
    subroot = create_test_tree_from_array([1])
    if root and subroot:
        assert helper(root, subroot)
