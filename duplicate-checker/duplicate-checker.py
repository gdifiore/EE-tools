import sys

file = sys.argv[1]

print("Checking for duplicate binary strings in file: ", file)
with open(file) as f:
    seen = set()
    for line in f:
        line_lower = line.lower()
        if line_lower in seen:
            print(line)
        else:
            seen.add(line_lower)