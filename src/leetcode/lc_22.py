# 22. Generate Parentheses
# Medium

# Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

# Example 1:

# Input: n = 3
# Output: ["((()))","(()())","(())()","()(())","()()()"]

# Example 2:

# Input: n = 1
# Output: ["()"]


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
