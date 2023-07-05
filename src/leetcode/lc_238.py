from typing import List

def get_prod_except_self(a: List[int]) -> List[int]:

    prefix = [1] * len(a)
    for i, elem in enumerate(a[:-1]):
        prefix[i + 1]  = (elem * prefix[i]) # leave last element to abide product except self

    p = 1
    for i in range(len(a) - 1, -1, -1):
        prefix[i] *= p
        p *= a[i]
    return prefix


def test():
    a = [9, 4,2,3]
    assert get_prod_except_self(a) == [24, 54, 108, 72]
