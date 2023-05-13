from sage.all import GF

if __name__ == "__main__":
  # prepare polynomials over Z_3
  Z5 = GF(5) # prime field
  Z5t = Z5["t"] # polynomial
  print("Z5t:", Z5)
  t = Z5t.gen() # get indeterminate

  # prepare an irreducible polynomial
  P = Z5t(t^2 + 2)
  assert(P.is_irreducible())

  # extension field
  F5_2 = GF(5^2, name="t", modulus=P)
  t = F5_2.gen()
  print("F3_2:", F5_2)

  assert(F5_2(t+2) * F5_2(2*t + 2) == F5_2(t))
    