# I forgot the seed I used for the PRNG lol
# this was used to brute force it because I knew the range of numbers that it was

str1 = "0101110111110111100001110011010101000100101001100100100011111101110111100010011111110101011011010101011110100001110100111100101111000000001110000011001001011001101111100100010010001001011001001010001010011110110011011001011110001111111110000111001001111001011101010000111100101100000011110011011000101110001001011000011100000111000010101000100011000011110100111100001001011101000101001000101011101110100010010000110010001001110011011111111110111011000111101011010110000111011001011111001101110010101000100110001111011010110001100110001110001100111000000111000110001110001110100001010110111001001000110110010110001101110000100100000100001110110000100110010101011110101011110111100100110001010110100110101001000110000110011011010000100101001011000100101111110101001110110011011101001101010000111010011010110110000001010000011111011010111011000011111110100010111011101111110001001001111100101110011000010000001011010111110110101110010101111101111001110111110000111100010011100101110110110000000010101101"

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

print("here")
for i in range(1598450880, 1603129598):
    sample = random_sample(1000, [0, 2], i)
    sample = ''.join(str(e) for e in sample)
    if sample != str1:
        continue
    if sample == str1:
        print("here1")
        print("collision found: ", i)
        exit()
