# Bloom Filter

A Bloom Filter is a probabilistic data structure that allows for efficient membership tests of a set. It has two parameters:

- $m$ is the filter size, denoting the number of bits
- $k$ is the number of hash functions

To add a member to the set, we compute the hash of the data using $k$ hash functions. Then, we take the results mod $m$ and set the corresponding bit to 1 in the filter.

To see if a data exists, we simply check if the resulting hash values are set. It is possible that these values are set by some other data, thus making this a probabilistic data structure.
