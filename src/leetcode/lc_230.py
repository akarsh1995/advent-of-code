# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


from typing import Generator, Optional
from tree import TreeNode, create_test_tree_from_lc_string


def inorder(node: Optional[TreeNode]) -> Generator:
    if node:
        for val in inorder(node.left):
            yield val

        yield node.val

        for val in inorder(node.right):
            yield val


class Solution(object):
    def kthSmallest(self, root, k):
        """
        :type root: TreeNode
        :type k: int
        :rtype: int
        """
        for i, val in enumerate(inorder(root)):
            if i == k - 1:
                return val


def test():
    arr = '[5,3,6,2,4,null,null,1]'
    root = create_test_tree_from_lc_string(arr)
    k = 3
    for i, val in enumerate(inorder(root)):
        if i == k - 1:
            assert val == 3

    arr = '[3,1,4,null,2]'
    k = 1
    for i, val in enumerate(inorder(root)):
        if i == k - 1:
            assert val == 1
