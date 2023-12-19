import math

def narcissistic( value ):
    len = int(math.log10(value) + 1)
    return value == sum(int(x) ** len for x in str(value))