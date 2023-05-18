from sage.all import GF, EllipticCurve, factor, Set

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
  return k, r

def example_91():
  # curve parameters
  p = 5 # prime field
  a = 1
  b = 1

  # find curve over extension field
  FP = GF(p)
  E_FP = EllipticCurve(FP, [a, b])
  print("Order of E1(F5):", E_FP.order())

  # find embedding degree of the original curve
  k, r = embedding_degree(E_FP)
  print("r: {0}\tk: {1}".format(r, k))

  # generate extension
  FPt.<t> = FP[] # type: ignore
  P_MOD_M = FPt(t^k + 2)
  assert(P_MOD_M.is_irreducible())

  FP_M = GF(p^k, name='t', modulus=P_MOD_M)
  E_FP_M = EllipticCurve(FP_M, [a, b])
  print("Order of E1(F5^2):", E_FP_M.order())

  # compute r-torsion set
  INF = E_FP_M(0)
  torsion_set = []
  for p in E_FP_M:
    if r * p == INF:
      torsion_set.append(p)
  torsion_set = Set(torsion_set)
  print("\n{0}-torsion set:".format(r))
  print(torsion_set)
  print(len(torsion_set), "points")



if __name__ == "__main__": 
  example_91()