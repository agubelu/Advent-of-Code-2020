def file_to_lines(filename, func=None, spl="\n"):
    with open(filename, "r", encoding="utf-8") as f:
        lines = [l.strip() for l in f.read().split(spl)]
    
    lines = list(filter(bool, lines)) # Remove empty lines
    if func:
        lines = list(map(func, lines))
    
    return lines
