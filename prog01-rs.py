#!./.env/bin/python3
import rustils

result = rustils.multi_and_sum(range(10000), range(10000))

print(result)