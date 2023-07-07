# 143. Reorder List
# Medium
# You are given the head of a singly linked-list. The list can be represented as:

# L0 → L1 → … → Ln - 1 → Ln

# Reorder the list to be on the following form:

# L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …

# You may not modify the values in the list's nodes. Only nodes themselves may be changed.

#

# Example 1:

# Input: head = [1,2,3,4]
# Output: [1,4,2,3]


# Definition for singly-linked list.
from typing import List, Optional


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



# curr = none, root = 1 -> 2 -> 3
# root.next = none curr = 1
def reverse_ll(node: ListNode):
    prev = None
    curr = node

    while curr:
        temp = curr.next
        curr.next = prev
        prev = curr
        curr = temp
    return prev


def reorder_list(root: Optional[ListNode]):
    if not root:
        return
    if root.next is None:
        return
    else:
        slow = root
        fast = root.next

        if fast.next is None:
            return

        while fast.next:
            if slow:
                slow =slow.next
            fast = fast.next
            if fast.next is None:
                break
            else:
                fast = fast.next

        if slow:
            to_reverse = slow.next
            slow.next = None
            if to_reverse:
                f = reverse_ll(to_reverse)

                head = root

                while head:
                    temp = head.next
                    if f:
                        tempf = f.next
                        head.next = f
                        head.next.next = temp
                        f = tempf
                        head = temp
                    else:
                        break


def test():
    l = ListNode(1)
    l.next = ListNode(2)
    l.next.next = ListNode(3)
    l.next.next.next = ListNode(4)
    l.next.next.next.next = ListNode(5)
    reorder_list(l)
    assert l.get_vals() == [1, 5, 2, 4, 3]
