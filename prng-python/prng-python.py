#
# (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.
#

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
    # parameters as in GNU C Library
    glibc = LCG(1103515245, 12345, 2 ** 32, seed)

    for i in range(n):
        observation = (upper - lower) * (next(glibc) / (2 ** 32 - 1)) + lower
        sample.append(int(observation))

    return sample


# 1000 random numbers between 0 and 1
sample = random_sample(1000, [0, 2], int(sys.argv[1]))
sample = ''.join(str(i) for i in sample)
sample = sample + "\n"

f = open("input.txt", "a+")
f.write(sample)
f.close()
