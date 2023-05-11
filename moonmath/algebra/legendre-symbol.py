# Legendre Symbol
# if the result is 1, then y is a quadratic residue in p
def legendre_symbol(y, p):
  assert(p % 2 == 1)
  l = y ** ((p - 1) // 2) % p
  if l == p - 1:
    return -1
  else:
    return l
   

if __name__ == "__main__":
  print(legendre_symbol(3, 5))