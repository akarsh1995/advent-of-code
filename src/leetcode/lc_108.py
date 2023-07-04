# 108. Convert Sorted Array to Binary Search Tree
# Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.

# Definition for a binary tree node.
from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        self.size = 0


    def traverse(self) -> List[Optional[int]]:
        v: List[Optional[int]] = [self.val] 
        _traverse(self.left, self.right, v)
        return v


    def is_leaf(self) -> bool:
        return not self.left and not self.right

    def __str__(self) -> str:
        return ", ".join(map(str, self.traverse()))



def _traverse(left: Optional[TreeNode], right: Optional[TreeNode], v: List[Optional[int]]):
    if left:
        v.append(left.val)
        if not right:
            v.append(None)

    if right:
        if not left:
            v.append(None)
        v.append(right.val)

    if left:
        _traverse(left.left, left.right, v)

    if right:
        _traverse(right.left, right.right, v)


def insert_it(node: Optional[TreeNode], arr: List[int]):
    m = len(arr) // 2

    if not node:
        node = TreeNode(arr[m])

    if len(arr) == 0:
        return
    else:
        if node.val > arr[m]:
            node.left = TreeNode(arr[m])
            insert_it(node.left, arr[:m])
            insert_it(node.left, arr[m+1:])
        else:
            node.right = TreeNode(arr[m])
            insert_it(node.right, arr[:m])
            insert_it(node.right, arr[m+1:])

def ins(arr: List[int]) -> TreeNode:
    m = len(arr) // 2
    root = TreeNode(arr[m])
    insert_it(root, arr[:m])
    insert_it(root, arr[m+1:])
    return root

def test_bst_insertion_1(): 
    v = [-10,-3,0,5,9]
    bst = [0, -3, 9, -10, None, 5, None]
    assert ins(v).traverse() == bst

def test_bst_insertion_2():
    v = ins([0, 1, 2, 3, 4, 5]).traverse()
    assert v == [3, 1, 5, 0, 2, 4, None]
