import time
t = time.time()

a = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
b = a - 1
ret = b + 1


print(f"Returned {hex(ret)} in {round((time.time() - t) * 10 **6)}µs")
