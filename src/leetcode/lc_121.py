# 121. Best Time to Buy and Sell Stock
# You are given an array prices where prices[i] is the price of a given stock on the ith day.

# You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

# Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

from typing import List


def get_max_profit(arr: List[int]) -> int:
    if len(arr) == 0 or len(arr) == 1:
        return 0
    if len(arr) == 2:
        return max(0, arr[-1] - arr[0])

    min_so_far = arr[0] 
    profit_max = 0

    for num in arr[1:]:
        min_so_far = min(num, min_so_far)
        profit_max = max(profit_max, num - min_so_far)

    return profit_max




def test():
    arr = [7,1,5,3,6,4]
    assert get_max_profit(arr) == 5

def test2():
    arr = [7,6,4,3,1]
    assert get_max_profit(arr) == 0

def test3():
    arr = [2,1,2,0,1]
    assert get_max_profit(arr) == 1

def test4():
    arr = [2,1,2,1,0,1,2]
    assert get_max_profit(arr) == 2

def test5():
    arr = [1, 2, 11, 4, 7]
    assert get_max_profit(arr) == 10

