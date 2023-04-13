# El-Gamal

In this implementation, we first generate the public and private keys. We randomly choose a 256-bit prime $p$ and a generator $g$, and then choose a random secret exponent $x$ to generate the public key $y$.

To encrypt a message, we choose a random number $k$ and compute the values $a = g^k \bmod p$ and $b = m \times y^k \bmod p$, where $m$ is the message as a byte array. The encrypted message is the pair $(a, b)$.

To decrypt the message, we compute $s = a^x \bmod p$ and then multiply $b$ by $s^{-1} \bmod p$ to recover the original message as a byte array.
