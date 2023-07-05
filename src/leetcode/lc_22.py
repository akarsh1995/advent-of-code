


from typing import List

OPEN = '('
CLOSE = ')'

def gen_par(g: List[str], p: str, o: int = 0, c: int = 0, n: int = 3):
    if o == c == n:
        g.append(p)
        return
    if o < n:
        gen_par(g, p + OPEN, o + 1, c, n)
    if c < o:
        gen_par(g, p + CLOSE, o, c+1, n)


def test():
    n = 3
    g = []
    gen_par(g, "", n=n)
    assert g == ["((()))","(()())","(())()","()(())","()()()"]
