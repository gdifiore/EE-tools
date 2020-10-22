#
# (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.
#

import sys
from textwrap import wrap

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

# 500000 random numbers between 0 and 1
sample = random_sample(500000, [0, 2], int(sys.argv[1]))
sample = ''.join(str(e) for e in sample)
sample = wrap(sample, 1000)

with open('input.txt', 'a+') as f:
    for rng in sample:
        f.write('%s\n' % rng)

f.close()

# seed_used = 1598465313