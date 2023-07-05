# 150. Evaluate Reverse Polish Notation
# Medium

# You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

# Evaluate the expression. Return an integer that represents the value of the expression.

# Note that:

#     The valid operators are '+', '-', '*', and '/'.
#     Each operand may be an integer or another expression.
#     The division between two integers always truncates toward zero.
#     There will not be any division by zero.
#     The input represents a valid arithmetic expression in a reverse polish notation.
#     The answer and all the intermediate calculations can be represented in a 32-bit integer.

#

# Example 1:

# Input: tokens = ["2","1","+","3","*"]
# Output: 9
# Explanation: ((2 + 1) * 3) = 9


from typing import List



calc = {
        '+': lambda x, y: y+x,
        '/': lambda x, y: int(y/x),
        '*': lambda x, y: y*x,
        '-': lambda x, y: y-x,
}

def calc_polish_notation(a: List[str]) -> int:
    s = []
    for char in a:
        if not char.strip('-').isnumeric():
            result = calc[char](*map(int, [s.pop(), s.pop()]))
            s.append(str(result))
        else:
            s.append(char)
    return int(s[-1])


def test():
    a = ["2","1","+","3","*"]
    assert calc_polish_notation(a) == 9

def test2():
    a = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
    assert calc_polish_notation(a) == 22
