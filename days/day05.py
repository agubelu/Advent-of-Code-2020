from etc.utils import file_to_lines

seats = file_to_lines("input/day05.txt")

# Aux functions
def bin_search(code, min_pos, max_pos):
    mid = (max_pos + min_pos) // 2
    if not code:
        return mid
    else:
        new_min, new_max = (min_pos, mid) if code[0] in ("F", "L") else (mid, max_pos)
        return bin_search(code[1:], new_min, new_max)

def get_seat_id(code):
    row = bin_search(code[:7], 0, 128)
    col = bin_search(code[7:], 0, 8)
    return row * 8 + col

# There's a much easier way to do this, but I wanna get to both answers in a single loop
highest_id = 0
existing_seats = {}
missing_seats = {}
missing_two_around = {}

def add_seats(sid):
    if sid in missing_seats:
        del missing_seats[sid]
    if sid in missing_two_around:
        del missing_two_around[sid]

    existing_seats[sid] = 1
    for other_sid in (sid + 1, sid - 1):
        if other_sid in existing_seats: continue
        n_refs = missing_seats.get(other_sid, 0) + 1
        missing_seats[other_sid] = n_refs
        if n_refs == 2:
            missing_two_around[other_sid] = 2

for seat in seats:
    sid = get_seat_id(seat)

    # Part 1
    if sid > highest_id:
        highest_id = sid

    # Part 2
    add_seats(sid)

my_seat = next(iter(missing_two_around))

print("Part 1:", highest_id)
print("Part 2:", my_seat)
