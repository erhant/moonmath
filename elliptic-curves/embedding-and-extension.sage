from sage.all import GF, EllipticCurve, factor

def embedding_degree(E, max_k = 50):
  # order of finite-field ord(F)
  p = E.base_field().order()
  # order of elliptic curve = ord(E(F))
  n = E.order()
  # largest prime factor of n
  r = max(factor(n))[0] 

  k = 1
  while k < r or k <= max_k:
    # Fermat's Little Theorem
    if (p ^ k - 1) % r == 0:
      break
    k += 1
  return k
  
def example_86():
  assert(embedding_degree(EllipticCurve(GF(5), [1, 1])) == 2)

def example_87():
  # TinyJubJub
  print(embedding_degree(EllipticCurve(GF(13), [8, 8])))

def example_90():
  F5 = GF(5)
  F5t.<t> = F5[] # type: ignore

  # degree = 2, polynomial = t^2 + 2 (which is irreducible)
  P_MOD_2 = F5t(t^2 + 2)
  print("t^2 + 2 is irreducible?: ", P_MOD_2.is_irreducible())

  # extension field
  F5_2 = GF(5^2, name='t', modulus=P_MOD_2)
  E1F5_2 = EllipticCurve(F5_2, [1, 1]) # same parameters with E(GF(5), [1, 1]), but different field
  print("Order:", E1F5_2.order())
  print("Points:")
  points = E1F5_2.points()
  infty = E1F5_2(0)
  for p in points:
    if p != infty:
      print(p.xy())
    else:
      print("point at infinity")

def exercise_76():
  F5 = GF(5)
  F5t.<t> = F5[] # type: ignore

  P_MOD_2 = F5t(t^2 + 2)
  F5_2 = GF(5^2, name='t', modulus=P_MOD_2)
  E1F5_2 = EllipticCurve(F5_2, [1, 1])

  # to check results
  print("(4t+3, 2t+1) + (3t + 3, 2) =", (E1F5_2(4*t + 3, 2*t + 1) + E1F5_2(3*t + 3, 2)).xy())
  print("x + (3t + 3, 3) = (3, 4)\tx =", (E1F5_2(3, 4) - E1F5_2(3*t + 3, 2)).xy())
  print("[5](2t + 1, 4t + 4) =", (5 * E1F5_2(2*t + 1, 4*t + 4)).xy())

def exercise_77():
  F13 = GF(13)
  F13t.<t> = F13[] # type: ignore
  P_MOD_4 = F13t(t^4 + 2) # degree = 4
  print(P_MOD_4.is_irreducible())
  F13_4 = GF(13^4, name='t', modulus=P_MOD_4)

  # tiny jub jub
  TJJ_F13_4 = EllipticCurve(F13_4, [8, 8])

  print("Order of E(F_13^4):", TJJ_F13_4.order())

def exercise_78():
  # curve parameters for alt_bn128
  p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
  a = 0
  b = 3

  # embedding
  k = 12 # known from a previous example in the book

  # prime field
  FP = GF(p)

  # polynomial ring over FP
  FPt.<t> = FP[] # type: ignore
  
  # find an irreducible polynomial with degree k (12)
  P_MOD_K = FPt.irreducible_element(k)

  # extension field
  FP_K = GF(p^k, name='t', modulus=P_MOD_K)

  E = EllipticCurve(FP_K, [a, b])

  print("Order of alt_bn128 extension:\n", E.order())

if __name__ == "__main__":
  exercise_78()
