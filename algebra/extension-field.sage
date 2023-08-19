from sage.all import GF

def exercise_56():
  # prepare polynomials over Z_3
  Z5 = GF(5) # prime field
  Z5t.<x> = Z5[] # polynomial # type: ignore
  print("Z5x:", Z5)

  # prepare an irreducible polynomial 
  P = Z5t(x^2 + 2) # type: ignore
  assert(P.is_irreducible())

  # extension field
  F5_2 = GF(5^2, name="x", modulus=P)
  print("F3_2:", F5_2)

  assert(F5_2(x + 2) * F5_2(2*x + 2) == F5_2(x)) # type: ignore
  print(F5_2.addition_table('elements'))

if __name__ == "__main__":
  exercise_56()