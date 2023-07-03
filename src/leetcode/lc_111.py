# 111. Minimum Depth of Binary Tree
# Given a binary tree, find its minimum depth.
# The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
# Note: A leaf is a node with no children.
# Input: root = [3,9,20,null,null,15,7]
# Output: 2


from typing import Optional
import tree


def depth_min(node: Optional[tree.TreeNode]) -> int:
    if not node:
        return 0
    if not node.left and node.right:
        return 1 +  depth_min(node.right)
    elif node.left and not node.right:
        return 1 +  depth_min(node.left)
    else:
        return 1 + min(depth_min(node.right), depth_min(node.left))


def test():
    root = tree.create_test_tree_from_lc_string('[3,9,20,null,null,15,7]')
    print(root)
    assert depth_min(root) == 2

def test2():
    root = tree.create_test_tree_from_lc_string('[2,null,3,null,4,null,5,null,6]')
    assert depth_min(root) == 5




