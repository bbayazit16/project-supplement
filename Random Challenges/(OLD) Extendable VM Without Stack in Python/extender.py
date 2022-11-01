from requests import get as rqget
from hashlib import sha256 as hsha256
from json import loads
from Lyra import Lyra


def add(_, a, b):
    return a + b


def sub(_, a, b):
    return a - b


def mul(_, a, b):
    return a * b


def div(_, a, b):
    return a / b


def _print(_, a):
    print(a)


def iszero(_, a):
    return int(a == 0)


def get(_, a):
    return rqget(a).text


def jload(_, a):
    return loads(a)


def choose(_, a, b):
    return a[b]


def stackinfo(stack):
    print(stack[1:])


def push(stack, a):
    stack.append(a)
    return stack


def _int(_, a):
    return int(a)


def gt(_, a, b):
    return int(a > b)


def lt(_, a, b):
    return int(a < b)


def eq(_, a, b):
    return int(a == b)


def _not(_, a):
    return int(not a)


def sha256(_, a):
    return hsha256(a.encode()).hexdigest()


lyra = Lyra()
lyra.extend("add", add)
lyra.extend("sub", sub)
lyra.extend("mul", mul)
lyra.extend("div", div)
lyra.extend("print", _print)
lyra.extend("iszero", iszero)
lyra.extend("get", get)
lyra.extend("jload", jload)
lyra.extend("choose", choose)
lyra.extend("stackinfo", stackinfo)
lyra.extend("push", push)
lyra.extend("int", _int)
lyra.extend("gt", gt)
lyra.extend("lt", lt)
lyra.extend("eq", eq)
lyra.extend("not", _not)
lyra.extend("sha256", sha256)

# lric = pickle.loads(pickle.dumps(lyra, pickle.HIGHEST_PROTOCOL))

lyra.compile("main.lyra", "main.lyrica")
# lric.execute("main.lyrica")