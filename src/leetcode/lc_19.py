# 19. Remove Nth Node From End of List
# Medium

# Given the head of a linked list, remove the nth node from the end of the list and return its head.

#

# Example 1:

# Input: head = [1,2,3,4,5], n = 2
# Output: [1,2,3,5]

# Example 2:

# Input: head = [1], n = 1
# Output: []

# Example 3:

# Input: head = [1,2], n = 1
# Output: [1]

# Definition for singly-linked list.
from typing import Optional, List
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def get_vals(self) -> List[int]:
        x = self
        l = []
        while x:
            l.append(x.val)
            x = x.next
        return l

    def __str__(self) -> str:
        x = self
        d = []
        while x:
            d.append(f"{x.val}")
            x = x.next
        return ", ".join(d)





def reverse_ll(node: ListNode) -> ListNode:
    prev = None
    curr = node

    while curr:
        temp = curr.next
        curr.next = prev
        prev = curr
        curr = temp

    return prev


def remove_nth(root: Optional[ListNode], nth: int) -> Optional[ListNode]:
    if not root:
        return

    root = reverse_ll(root)
    if nth == 1:
        return root.next
    else:
        prev = root
        curr = root.next

        n = 2

        while True:
            if nth == n:
                curr = curr.next
                prev.next = curr
                break
            else:
                temp = curr
                curr = curr.next
                prev = temp
                n += 1

        return reverse_ll(root)


def test():
    head = [1,2,3]
    root = ListNode(1)
    root.next = ListNode(2)
    root.next.next = ListNode(3)

    n = 3
    assert remove_nth(root, n).get_vals() == [2, 3]
