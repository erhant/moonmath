from sage.all import GF

def cge(g: int, x: int, n: int) -> int:
  '''
  Cyclic Group Exponentiation using "square-and-multiply"

  Parameters:
  - `g`: generator
  - `x`: exponent
  - `n`: order

  Returns:
  - `g^x (mod n)`
  '''
  h = g
  x >>= 1
  while x > 0:
    h = (h * h) % n   # square
    if x & 1 == 1:
      h = (h * g) % n # multiply
    x >>= 1
  return h

def esm(g: int, x: int, n: int) -> int:
  '''
  Efficient Scalar Multiplication using "double-and-add"

  Parameters:
  - `g`: number
  - `x`: number
  - `n`: order

  Returns:
  - `g*x (mod n)`
  '''
  h = g
  x >>= 1
  while x > 0:
    h = (h + h) % n   # double
    if x & 1 == 1:
      h = (h + g) % n # add
    x >>= 1
  return h
  
if __name__ == "__main__":
  F13 = GF(13)
  assert(F13(4) ^ 7 == cge(4, 7, 13))
  assert(F13(4) * 7 == esm(4, 7, 13))
  
    