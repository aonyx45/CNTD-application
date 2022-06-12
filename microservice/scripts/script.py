from flask import Flask
from os import getenv
import random
import string

# printing lowercase
APP_NAME = ''.join(random.choice(string.ascii_lowercase) for i in range(10))

app = Flask(__name__)

@app.route("/")
def hello_world():
    return "<h1>Hello from API : " + APP_NAME + "</h1>" + "<p>Return <a href=\"/\">home</a></p>"