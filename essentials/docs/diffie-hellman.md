# Diffie-Hellman

Alice and Bob would like to agree on a shared key, but they do not have any prior symmetric key to do this in a private way. Diffie-Hellman is an algorithm to achieves this!

- Alice and Bob both generate a private (secret) key, which is a random positive integer less than the order of the curve $N$.
  - Alice: $s_a \gets (0, N)$
  - Bob: $s_b \gets (0, N)$
- They also generate a public key, which is a point on the elliptic curve that is simply some base point $G$ multiplied by the private key.
  - Alice: $p_a = G \times s_a$
  - Bob: $p_b = G \times s_b$
- To share this key, both parties take one public key and multiply it with their own private key:
  - Alice: $k = p_b \times s_a = G \times s_a \times s_b$
  - Bob: $k = p_a \times s_b = G \times s_b \times s_a$

As shown above, both computed keys are equal to $G \times s_a \times s_b$, but an outsider can't find it because going back from an elliptic-curve multiplicatin is hard. The original implementation is based on the discrete-log assumption, which is rather similar.
