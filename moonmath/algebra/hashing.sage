import hashlib

# "try-and-increment" method on top of SHA256
# we expect `mod` to be close to 2^256 for this example
def sha256_try_and_increment(s, mod):
  # initial sha256 hash-to-integer
  z = sha256(s.encode())
  
  # if fails, try incrementing
  c = 0
  while z < mod:
    z = sha256(s.encode() + str(c).encode())

  return z

# Pedersen hash on top of SHA256, hash to cyclic-group
def sha256_pedersen(s, gs, N):

  # sha256 hash-to-integer as usual
  z = sha256(s)

  # get binary digits
  z_bin = z.digits(base = 2, padto = 256)

  # multiply with powers of the generators
  Z = Integers(N)
  ans = 1
  for i in range(len(gs)):
    ans *= Z(gs[i]) ^ z_bin[i]
  
  return ans

# Simple SHA256 to integer mapping
def sha256(s):
  # create hasher
  hasher = hashlib.sha256(s)

  # hash as hex string
  hexdigest = hasher.hexdigest()

  # map to an integer
  d = ZZ(hexdigest, 16)

  return d

  
if __name__ == "__main__":
  # print(sha256_pedersen("Math", [2, 3], 5))
  print(sha256_pedersen("", [3, 7, 11], 12))
