from typing import List


def cumulative_multiply(a: List[int]) -> List[int]:
    l = [1]
    for elem in a:
        l.append(elem * l[-1])
    return l[1:]

def get_prod_except_self(a: List[int]) -> List[int]:
    np_arry = a

    before = cumulative_multiply(a)
    after = cumulative_multiply(a[::-1])
    after = after[::-1]

    l = []
    for i in range(len(np_arry)):
        if i == 0:
            before_elem = 1
        else:
            before_elem = before[i - 1]

        if i == len(np_arry) - 1:
            after_elem = 1
        else:
            after_elem = after[i + 1]

        l.append(after_elem * before_elem)
    return l


def test_cum_prod():
    a = [9, 4,2,3]
    assert cumulative_multiply(a) == [9, 36, 72, 216]


def test():
    a = [9, 4,2,3]
    assert get_prod_except_self(a) == [24, 54, 108, 72]
