# 739. Daily Temperatures
# Medium

# Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.


from typing import List

def get_waitlist(arr: List[int]) -> List[int]:
    waitlist = [0] * len(arr)
    temp_stack = []
    for i, elem in enumerate(arr):
        while True:
            if temp_stack:
                if temp_stack[-1][0] < elem:
                    _, index = temp_stack.pop()
                    diff_of_days = i - index
                    waitlist[index] = diff_of_days
                else:
                    break
            else:
                break
        temp_stack.append((elem, i))
    return waitlist


def test():
    temps = [73, 74, 75, 71, 69, 72, 76, 73]
    assert get_waitlist(temps) == [1, 1, 4, 2, 1, 1, 0, 0]


def test2():
    temps = [30, 40, 50, 60]
    assert get_waitlist(temps) == [1, 1, 1, 0]

def test3():
    temps = [30, 60, 90]
    assert get_waitlist(temps) == [1, 1, 0]

def test4():
    temps = [0, 3]
    assert get_waitlist(temps) == [1, 0]
