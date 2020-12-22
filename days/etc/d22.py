class Deck:
    def from_lines(lines):
        return Deck([int(x) for x in lines])

    def __init__(self, ls):
        self.cards = ls

    def draw(self):
        return self.cards.pop(0)

    def add(self, card1, card2):
        self.cards += [card1, card2]

    def copy(self):
        return Deck(self.cards.copy())

    def __bool__(self):
        return bool(self.cards)

    def __len__(self):
        return len(self.cards)

    def __getitem__(self, k):
        return self.cards[k]

    def __repr__(self):
        return str(self.cards)

class Combat:
    def __init__(self, deck_1, deck_2, recursive=False):
        self.deck_1 = deck_1
        self.deck_2 = deck_2
        self.n_cards = len(deck_1) + len(deck_2)
        self.recursive = recursive
        self.previous_states = set()

    def play_game(self):
        while self.deck_1 and self.deck_2:

            if self.recursive:
                # Check that the state has not occured before in this game
                # If it has, player 1 wins
                cur_hash = hash(self)
                if cur_hash in self.previous_states:
                    return (1, None)
                else:
                    self.previous_states.add(cur_hash)

            self._play_round()

        winner = 1 if self.deck_1 else 2
        score = self._get_score(winner)
        return winner, score

    def _play_round(self):
        card1 = self.deck_1.draw()
        card2 = self.deck_2.draw()

        if self.recursive and len(self.deck_1) >= card1 and len(self.deck_2) >= card2:
            new_deck_1 = Deck(self.deck_1[:card1])
            new_deck_2 = Deck(self.deck_2[:card2])
            winner, _ = Combat(new_deck_1, new_deck_2, recursive=True).play_game()
        else:
            winner = 1 if card1 > card2 else 2
            
        if winner == 2:
            card1, card2 = card2, card1

        winner_deck = self.deck_1 if winner == 1 else self.deck_2
        winner_deck.add(card1, card2)

    def _get_score(self, winner):
        winner_deck = self.deck_1 if winner == 1 else self.deck_2
        return sum((self.n_cards - i) * winner_deck[i] for i in range(self.n_cards))

    def __hash__(self):
        return hash(str(self.deck_1) + "|" + str(self.deck_2))

    