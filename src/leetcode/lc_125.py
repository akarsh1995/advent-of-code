
def check_palin(s: str) -> bool:

    i = 0
    j = len(s) - 1

    while i <= j:
        if s[i].isalnum() and s[j].isalnum():
            if s[i].lower() == s[j].lower():
                i += 1
                j -= 1
            else:
                return False
        else:
            if not s[i].isalnum():
                i += 1

            if not s[j].isalnum():
                j -= 1

    return True


def test1():
    assert check_palin('A man, a plan, a canal: Panama')
    assert not check_palin('A man, a n, a canal: Panama')
    assert check_palin('aaaaaaaa')
    assert check_palin(',,,,,,')
    assert check_palin(',aba')
    assert check_palin(',,a')
    assert check_palin(',')
    assert check_palin(',,,')
    assert check_palin(' ')
    assert check_palin('8')
    assert not check_palin('89')

