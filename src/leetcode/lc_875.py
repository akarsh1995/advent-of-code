# Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

# Return the minimum integer k such that she can eat all the bananas within h hours.

# Example 1:

# Input: piles = [3,6,7,11], h = 8
# Output: 4

# Example 2:

# Input: piles = [30,11,23,4,20], h = 5
# Output: 30

# Example 3:

# Input: piles = [30,11,23,4,20], h = 6
# Output: 23

# Constraints:

#     1 <= piles.length <= 104
#     piles.length <= h <= 109
#     1 <= piles[i] <= 109

from typing import List


def calc_hours(piles: List[int], rate: int) -> int:
    return sum(((pile - 1) // rate) + 1 for pile in piles)


def min_eating_rate(piles: List[int], h: int) -> int:
    def condition(m):
        return calc_hours(piles, m) <= h

    res = max(piles)

    left, right = 1, res

    while left < right:
        mid = left + (right - left) // 2
        cond = condition(mid)
        if cond:
            # in search of lower rate
            res = min(res, mid)
            right = mid
        else:
            left = mid + 1
    return left


def test():
    a = [3,6,7,11]
    h = 8
    assert min_eating_rate(a, h) == 4

def test2():
    a = [30,11,23,4,20]
    h = 5
    assert min_eating_rate(a, h) == 30

def test3():
    a = [30,11,23,4,20]
    h = 6
    assert min_eating_rate(a, h) == 23
