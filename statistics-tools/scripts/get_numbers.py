import json
import requests
import os

API_KEY = ""

def main():
    url = "https://api.random.org/json-rpc/2/invoke"

    # request for 10000 integers between 0 and 1
    payload = {
        "jsonrpc": "2.0",
        "method": "generateIntegers",
        "params": {
            "apiKey": API_KEY,
            "n": 10000,
            "min": 0,
            "max": 1,
            "replacement": True,
            "base": 10
        },
        "id": 32749
    }
    print(payload)
    response = requests.post(url, json=payload).json()

    data = response["result"]["random"]["data"]
    data = [str(data) for data in data]
    data = "".join(data)
    print(data)

    f = open("input.txt", "w")
    f.write(data)
    f.close()

if __name__ == "__main__":
    main()
