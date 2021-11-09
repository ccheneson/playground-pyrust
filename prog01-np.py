#!/usr/bin/python3

import numpy as np

x = range(10000)
y = range(10000)

def process01():
    mat_x = np.array(x).reshape(len(x), 1)
    mat_y = np.array(y).reshape(1, len(x))

    return np.sum(mat_x * mat_y)

result = process01()
print(result)