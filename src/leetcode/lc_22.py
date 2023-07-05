


from typing import List

OPEN = '('
CLOSE = ')'

def gen_par(g: List[str], temp: str, o: int = 0, c: int = 0, n: int = 3):
    if o == n:
        if c < n:
            gen_par(g, temp + CLOSE, o, c + 1, n)
        else:
            g.append(temp)
    elif o == c:
        gen_par(g, temp + OPEN, o + 1, c, n)
    elif o > c:
        gen_par(g, temp + OPEN, o + 1, c, n)
        gen_par(g, temp + CLOSE, o, c + 1, n)

def test():
    n = 3
    g = []
    gen_par(g, "", n=n)
    assert g == ["((()))","(()())","(())()","()(())","()()()"]
