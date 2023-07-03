# 101. Symmetric Tree
# Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

# Definition for a binary tree node.
from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def are_symmetric(v: List[Optional[TreeNode]]) -> bool:

    if not any(v):
        return True

    flag = True

    for elem_from_left, elem_from_right in zip(v, v[::-1]):
        if elem_from_left and elem_from_right:
            if elem_from_left.val != elem_from_right.val:
                flag &= False
        elif elem_from_left is None and elem_from_right is None:
            flag &= True
        else:
            return False

    all_child = []
    for n in v:
        if n:
            all_child.append(n.left)
            all_child.append(n.right)
        else:
            all_child.append(None)

    return flag and are_symmetric(all_child)

def test_compare(): 
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.left.left = TreeNode(3)
    root1.left.right = TreeNode(4)

    root1.right = TreeNode(2)
    root1.right.right = TreeNode(3)
    root1.right.left = TreeNode(4)

    assert are_symmetric([root1])

    root1 = TreeNode(1)
    root1.left = TreeNode(5)
    root1.left.left = TreeNode(3)
    root1.left.right = TreeNode(4)

    root1.right = TreeNode(2)
    root1.right.right = TreeNode(3)
    root1.right.left = TreeNode(4)

    assert are_symmetric([root1]) == False
