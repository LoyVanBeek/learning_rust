#! /usr/bin/env python

import threading

def count(n=5000000):
    c = 0
    for _ in range(n):
        c += 1

for _ in range(10):
    threading.Thread(target=count).start()
