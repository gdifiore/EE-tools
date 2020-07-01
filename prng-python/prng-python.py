import sys

def LCG(a, c, m, seed):
    xi = seed
    while True:
        xf = (a * xi + c) % m
        xi = xf
        yield xf

def random_sample(n, interval, seed):
    lower, upper = interval[0], interval[1]
    sample = []
    glibc = LCG(1103515245, 12345, 2 ** 32, seed)       # parameters as in GNU C Library

    for i in range(n):
        observation = (upper - lower) * (next(glibc) / (2 ** 32 - 1)) + lower
        sample.append(int(observation))

    return sample

sample = random_sample(100, [0, 2], int(sys.argv[1]))   # 100 random numbers between 0 and 1
sample = ''.join(str(i) for i in sample)
print(sample)