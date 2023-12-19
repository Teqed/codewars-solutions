import math
from functools import reduce

def narcissistic(value):
    length = int(math.log10(value)) + 1
    def fold_fn(acc, _):
        n, total = acc
        digit = n % 10
        return (n // 10, total + digit ** length)
    return reduce(fold_fn, range(length), (value, 0))[1] == value
