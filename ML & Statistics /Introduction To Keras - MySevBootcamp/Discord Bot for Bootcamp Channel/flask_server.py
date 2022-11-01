from flask import Flask
from threading import Thread

app = Flask('')

@app.route('/')
def home():
    return "Bot açık kalsın yeter"

def run():
  app.run(host='0.0.0.0',port=8080)

def runner():
    threading = Thread(target=run)
    threading.start()