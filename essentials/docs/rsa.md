# RSA

We generate a public and private key by generating two large prime numbers $p$ and $q$. Then,

- we find $n = p \times q$
- $\phi = (p - 1) \times (q - 1)$

Then we choose a publicly known $e$ and calculate the private exponent $d = e^{-1} \bmod{\phi}$.

We then generate a hash of the original message using SHA256, and encrypt the hash using the public key by computing $c = (h^e) \bmod n$.

We decrypt the hash using the private key by computing $h = (c ^ d) \bmod n$.

We compare the decrypted hash with the original hash to verify the authenticity of the message.
