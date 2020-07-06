#
# (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.
#

import json
import requests
import sys
import time
import requests
import os
from bs4 import BeautifulSoup

RANDOM_API_KEY = ""
HOTBITS_API_KEY = ""


def randomorg():
    url = "https://api.random.org/json-rpc/2/invoke"

    # request for 1000 integers between 0 and 1
    payload = {
        "jsonrpc": "2.0",
        "method": "generateIntegers",
        "params": {
            "apiKey": RANDOM_API_KEY,
            "n": 1000,
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

    f = open("input.txt", "w")
    f.write(data)
    f.close()

def hotbits():
    url = "https://www.fourmilab.ch/cgi-bin/Hotbits.api?nbytes=140&fmt=bin&apikey=" + HOTBITS_API_KEY
    r = requests.get(url)

    with open('Hotbits.api', 'wb') as f:
        f.write(r.content)

    with open('Hotbits.api', 'rb') as f:
        hexdata = f.read().hex()

    integer = int(hexdata, 16)
    i = format(integer, '0>42b')
    string = str(i)
    last_n_char = len(string) - 1000
    string = string[:-int(last_n_char)] + "\n"
    f = open("input.txt", "a+")
    f.write(string)
    f.close()

    os.remove('Hotbits.api')

def quantum():
    resp = requests.get('https://qrng.anu.edu.au/RawBin.php')
    soup = BeautifulSoup(resp.content, 'html.parser')

    data = soup.find("table", class_ = "rng")
    bins = data.get_text()

    string = bins[3:-24] + "\n"
    f = open("input.txt", "a+")
    f.write(string)
    f.close()


if sys.argv[1] == "randomorg":
    for i in range(0, 200):
        randomorg()
        time.sleep(5)

if sys.argv[1] == "hotbits":
    for i in range(0, 200):
        hotbits()
        time.sleep(5)

if sys.argv[1] == "quantum":
    for i in range(0, 200):
        quantum()
        time.sleep(5)
