from Lyra import Lyra
import requests
import json

lyra = Lyra()

def add(_, a, b):
    return a + b

def stackinfo(stack):
    print(stack[1:])

def getrequest(_, url):
    return requests.get(url).text

def choose(_, obj, name):
    return json.loads(obj)[name]

lyra.extend("add", add)
lyra.extend("stackinfo", stackinfo)
lyra.extend("get", getrequest)
lyra.extend("choose", choose)

lyra.compile("main.lyra", "main.lyrica")
Lyra.execute("main.lyrica")