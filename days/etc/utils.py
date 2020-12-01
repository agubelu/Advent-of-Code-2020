def file_to_lines(filename, func=None):
    with open(filename, "r", encoding="utf-8") as f:
        lines = [l.strip() for l in f.readlines()]
    
    lines = list(filter(bool, lines))
    if func:
        lines = list(map(func, lines))
    
    return lines
