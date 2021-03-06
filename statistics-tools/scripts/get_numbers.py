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
    response = requests.post(url, json=payload).json()

    data = response["result"]["random"]["data"]
    data = [str(data) for data in data]
    data = "".join(data)
    data = data + "\n"

    f = open("input.txt", "a+")
    f.write(data)
    f.close()

def hotbits():
    url = "https://www.fourmilab.ch/cgi-bin/Hotbits.api?nbytes=140&fmt=bin&apikey=" + HOTBITS_API_KEY
    r = requests.get(url)

    with open('Hotbits.api', 'wb') as f:
        f.write(r.content)

    with open('Hotbits.api', 'rb') as f:
        hexdata = f.read().hex()
        f.close()

    integer = int(hexdata, 16)
    i = format(integer, '0>42b')
    string = str(i)
    last_n_char = len(string) - 1000
    string = string[:-int(last_n_char)] + "\n"
    f = open("input.txt", "a+")
    f.write(string)
    f.close()

    os.remove('Hotbits.api')
    print(os.path.exists("Hotbits.api"))

def quantum():
    resp = requests.get('https://qrng.anu.edu.au/RawBin.php')
    soup = BeautifulSoup(resp.content, 'html.parser')

    data = soup.find("table", class_ = "rng")
    bins = data.get_text()

    string = bins[3:-26] + "\n"
    f = open("input.txt", "a+")
    f.write(string)
    f.close()

# need to run this twice over two days, the bits we want exceed the daily limit
if sys.argv[1] == "randomorg":
    for i in range(0, 250):
        randomorg()
        time.sleep(1)

if sys.argv[1] == "hotbits":
    for i in range(0, 88):
        print("test")
        hotbits()
        time.sleep(1)

if sys.argv[1] == "quantum":
    for i in range(0, 500):
        quantum()
        time.sleep(1)
