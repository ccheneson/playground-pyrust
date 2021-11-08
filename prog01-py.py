#!/usr/bin/python3

x = range(10000)
y = range(10000)

def process01():
    result = 0
    for i in x:
        for j in y:
            result += i * j           
    return result

result = process01()
print(result)