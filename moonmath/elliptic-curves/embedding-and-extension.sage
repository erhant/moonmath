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
  F5t = F5['t']
  t = F5t.gen() # get indeterminate

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

if __name__ == "__main__":
  example_90()
