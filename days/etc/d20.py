class Tile:
    def __init__(self, tile_id, content):
        self.tile_id = tile_id
        self.content = content

    def from_string(raw):
        lines = raw.split("\n")
        tile_id = int(lines[0][5:-1])
        content = lines[1:]
        return Tile(tile_id, content)

    def transform(self, flip_x, flip_y, n_rotations):
        new_content = self.content.copy()
        new_tile = Tile(self.tile_id, new_content)

        if flip_x:
            new_tile._flip_x()
        if flip_y:
            new_tile._flip_y()
        
        new_tile._rotate(n_rotations)
        return new_tile
    
    def get_borders(self):
        border_top = self.content[0]
        border_right = "".join(line[-1] for line in self.content)
        border_bottom = self.content[-1]
        border_left = "".join(line[0] for line in self.content)
        return [border_top, border_right, border_bottom, border_left]

    def get_borders_and_invs(self):
        bs = self.get_borders()
        return bs + [x[::-1] for x in bs]

    def flatten(self):
        return "".join(self.content)

    def get_trimmed_content(self):
        return [line[1:-1] for line in self.content[1:-1]]

    def _flip_y(self):
        for i in range(len(self.content)):
            self.content[i] = self.content[i][::-1]

    def _flip_x(self):
        self.content.reverse()

    def _rotate(self, n_rotations):
        len_row = len(self.content[0])

        for _ in range(n_rotations):
            new_content = []

            for i in range(len_row):
                new_row = "".join(row[i] for row in reversed(self.content))
                new_content.append(new_row)
            
            self.content = new_content
        
    def __repr__(self):
        res = f"<Tile {self.tile_id}>\n"
        res += "\n".join(self.content)
        return res