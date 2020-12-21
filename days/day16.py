from etc.utils import file_to_lines
from etc.d16 import Ticket, Field

from itertools import chain
from math import prod

field_lines, my_lines, ticket_lines = file_to_lines("input/day16.txt", spl="\n\n")

fields = list(map(Field.from_line, field_lines.split("\n")))
my_ticket = Ticket.from_line(my_lines.split("\n")[1])
tickets = list(map(Ticket.from_line, ticket_lines.split("\n")[1:]))
n_fields = len(fields)

# Part 1
aux = [(t, t.get_invalid_values(fields)) for t in tickets]
valid_tickets = [x[0] for x in aux if not x[1]]
sol_part_1 = sum(chain(*(x[1] for x in aux)))
print("Part 1:", sol_part_1)

# Part 2
def ticket_pos_to_fields(ticket, fields):
    return [set(f for f in fields if f.accepts_val(val)) for val in ticket]

data_mat = [ticket_pos_to_fields(t, fields) for t in valid_tickets]

sets = data_mat[0]
for i in range(n_fields):
    sets[i].intersection_update(*(data[i] for data in data_mat[1:]))

disambiguated_pos = set()
fields_order = {}
while len(disambiguated_pos) != n_fields:
    for i in range(n_fields):
        if i not in disambiguated_pos and len(sets[i]) == 1:
            disambiguated_pos.add(i)
            for j in range(n_fields):
                if j != i and j not in disambiguated_pos:
                    sets[j] -= sets[i]
            fields_order[i] = sets[i].pop()

sol_part_2 = prod(my_ticket[i] for i in range(n_fields) if fields_order[i].name.startswith("departure"))
print("Part 2:", sol_part_2)
