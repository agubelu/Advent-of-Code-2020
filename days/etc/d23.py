from tqdm import trange

class CrabCupGame:
    def __init__(self, ls):
        self.n_cups = len(ls)
        self.adj = {ls[i]: ls[i + 1] for i in range(self.n_cups - 1)}
        self.adj[ls[-1]] = ls[0]
        self.current = ls[0]

    def play(self, n_rounds, progress_bar=False):
        range_func = trange if progress_bar else range

        for _ in range_func(n_rounds):
            current = self.current
            taken = self._take3()
            target = self._find_target(taken)

            taken_next = self.adj[taken[2]]
            target_next = self.adj[target]

            self.adj[current] = taken_next
            self.adj[target] = taken[0]
            self.adj[taken[2]] = target_next

            self.current = self.adj[current]

        return self.adj

    def _take3(self):
        e1 = self.adj[self.current]
        e2 = self.adj[e1]
        e3 = self.adj[e2]
        return (e1, e2, e3)

    def _find_target(self, taken):
        target = self.current - 1 if self.current > 1 else self.n_cups
        while target in taken:
            target = target - 1 if target > 1 else self.n_cups
        return target
    
    def __repr__(self):
        ls = [1]
        while len(ls) != self.n_cups:
            ls.append(self.adj[ls[-1]])
        return " ".join(str(x) for x in ls)
