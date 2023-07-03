# 110. Balanced Binary Tree
# Given a binary tree, determine if it is height-balanced

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def max_height(root: Optional[TreeNode]):
    if not root:
        return 0
    return 1 + max(max_height(root.left), max_height(root.right))

def check_balanced(root: Optional[TreeNode]):
    if not root:
        return True
    return (abs(max_height(root.left) - max_height(root.right)) <= 1) and check_balanced(root.left) and check_balanced(root.right)


# To build the sample tree
def insertLevelOrder(arr, i, n):
    root: Optional[TreeNode] = None
    # Base case for recursion
    if i < n:
        if arr[i] is not None:
            root = TreeNode(arr[i])
     
            # insert left child
            root.left = insertLevelOrder(arr, 2 * i + 1, n)
     
            # insert right child
            root.right = insertLevelOrder(arr, 2 * i + 2, n)
    return root

def test2():
    root = [1,2,2,3,3,None,None,4,4]
    root = insertLevelOrder(root, 0, len(root))
    assert not check_balanced(root)

    root = [3,9,20,None,None,15,7]
    root = insertLevelOrder(root, 0, len(root))
    assert check_balanced(root)
    
