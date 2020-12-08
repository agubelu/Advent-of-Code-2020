class HandheldConsole:
    def __init__(self, code):
        self.code = code

    def run(self, yolo=False):
        acc = 0
        pos = 0
        seen = {}
        n_instr = len(self.code)

        while yolo or pos not in seen:
            seen[pos] = 0
            instr, val = self.code[pos]
            incr_pos = 1

            if instr == "acc":
                acc += val
            elif instr == "jmp":
                incr_pos = val

            pos += incr_pos
            if pos >= n_instr: break

        return acc
