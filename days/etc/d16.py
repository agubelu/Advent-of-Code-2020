import re

RE_TICKET = re.compile(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)")

class Field:
    def from_line(line):
        m = RE_TICKET.match(line)
        name = m.group(1)
        s1, e1, s2, e2 = (int(x) for x in m.groups()[1:])
        return Field(name, s1, e1, s2, e2)

    def __init__(self, name, start1, end1, start2, end2):
        self.name = name
        self.data = (start1, end1, start2, end2)
        self.range1 = range(start1, end1 + 1)
        self.range2 = range(start2, end2 + 1)

    def __repr__(self):
        return f"<Field: {self.name} [{self.data[0]}-{self.data[1]}] & [{self.data[2]}-{self.data[3]}]>"

    def accepts_val(self, val):
        return val in self.range1 or val in self.range2

class Ticket:
    def from_line(line):
        vals = [int(x) for x in line.split(",")]
        return Ticket(vals)

    def __init__(self, vals):
        self.vals = vals

    def get_invalid_values(self, tickets):
        res = []
        for val in self.vals:
            if all(not t.accepts_val(val) for t in tickets):
                res.append(val)
        return res

    def __iter__(self):
        for val in self.vals:
            yield val

    def __getitem__(self, key):
        return self.vals[key]

    def __repr__(self):
        return "<Ticket [" + ",".join(map(str, self.vals)) + "]>"

