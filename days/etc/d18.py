class Expr:
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def __mul__(self, other):
        return Prod(self, other)

    def __floordiv__(self, other):
        # // is a sum, with same priority as * (Part 1)
        return Sum(self, other)

    def __pow__(self, other):
        # ** is also a sum, but with a higher priority (Part 2)
        return Sum(self, other)

class Sum(Expr):
    def eval(self):
        return self.a.eval() + self.b.eval()

class Prod(Expr):
    def eval(self):
        return self.a.eval() * self.b.eval()

class Val(Expr):
    def __init__(self, v):
        self.v = v

    def eval(self):
        return self.v

# I made this expression parser for part 1, before part 2 fucked it up
# Not in use anymore but I'm proud of it so it'll stay here
def parse_expr(line, start_pos):
    if line[start_pos] == ")":
        right_expr, right_len = parse_expr(line, start_pos - 1)
        right_len += 2
    else:
        right_expr, right_len = parse_val(line, start_pos)

    next_pos = start_pos - right_len

    if next_pos < 0 or line[next_pos] == "(":
        return right_expr, right_len

    expr_class = Sum if line[next_pos] == "+" else Prod
    next_pos -= 1
        
    left_expr, left_len = parse_expr(line, next_pos)
    return expr_class(left_expr, right_expr), left_len + right_len + 1

def parse_val(line, start_pos):
    res = 0
    expr_len = 0
    while True:
        try:
            res += int(line[start_pos - expr_len]) * pow(10, expr_len)
            expr_len += 1

            if start_pos - expr_len < 0:
                break
        except ValueError:
            break
    return Val(res), expr_len

def parse_line(line):
    line = line.replace(" ", "")
    return parse_expr(line, len(line) - 1)[0]
