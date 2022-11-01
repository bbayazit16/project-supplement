from math import ceil, log

for i in range(128):
    rdx = 1 << i
    rad = 1 << (i + 1)
    print(f"{0 if rdx == 1 else rdx}..={rad-1} => {ceil(log(rdx+1, 2))},")