from sage.all import GF

if __name__ == "__main__":
  # prepare polynomials over Z_3
  Z5 = GF(5) # prime field
  Z5t.<t> = Z5[] # polynomial # type: ignore
  print("Z5t:", Z5)

  # prepare an irreducible polynomial 
  P = Z5t(t^2 + 2) # type: ignore
  assert(P.is_irreducible())

  # extension field
  F5_2 = GF(5^2, name="t", modulus=P)
  t = F5_2.gen()
  print("F3_2:", F5_2)

  assert(F5_2(t + 2) * F5_2(2*t + 2) == F5_2(t))
    