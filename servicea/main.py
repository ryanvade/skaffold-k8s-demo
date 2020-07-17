from typing import Optional
from fastapi import FastAPI
import requests

app = FastAPI()

def get_c():
    response = requests.get("http://servicec:3000")
    print(response)
    return response.json()

@app.get("/")
def root():
    c = get_c()
    return { "Service": "A", "C": c }
