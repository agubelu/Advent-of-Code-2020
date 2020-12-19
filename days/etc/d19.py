class Pattern:
    def __init__(self, text):
        self.text = text

    def to_regex(self, other_patterns):
        raise NotImplementedError

class TextPattern(Pattern):
    def to_regex(self, other_patterns):
        return self.text

class CompositePattern(Pattern):
    def __init__(self, text):
        super().__init__(text)
        
        self.patterns = []
        spl = text.strip().split("|")

        for s in spl:
            pat = [int(x) for x in s.strip().split(" ")]
            self.patterns.append(pat)

    def to_regex(self, other_patterns):
        pat_to_re = lambda pat: "".join(other_patterns[x].to_regex(other_patterns) for x in pat)

        if len(self.patterns) == 1:
            return pat_to_re(self.patterns[0])
        else:
            return "(" + "|".join(pat_to_re(pat) for pat in self.patterns) + ")"
