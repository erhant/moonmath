from sage.all import GF

def exercise_53():
  # Extension of F3 over t^2 + 1
  F3_2 = GF(3^2, name="x", modulus=GF(3)['x']([1, 0, 1]))
  print(F3_2)

  # solve for y^2 = x^3 + 4
  solutions = []
  for x in F3_2:
    for y in F3_2:
      if y^2 == x^3 + 4:
        solutions.append((x, y))
  print(solutions)

def exercise_54():
  F3 = GF(3)
  F3x = F3['x']
  Q = F3x([2, 1, 2]) # x^2 + x + 2

  # is irreducible?
  is_irreducible = True
  for elem in F3:
    if Q(elem) == 0:
      is_irreducible = False
  assert(is_irreducible == Q.is_irreducible())

  F3_2 = GF(3^2, name="x", modulus=Q)

  print(F3_2.multiplication_table('elements'))


def exercise_56():
  F5 = GF(5)
  F5x.<x> = F5[] # type: ignore

  # is irreducible?
  P = F5x(x^2 + 2) # type: ignore
  is_irreducible = True
  for elem in F5:
    if P(elem) == 0:
      is_irreducible = False
  assert(is_irreducible == P.is_irreducible())

  F5_2 = GF(5^2, name="x", modulus=P)
  print(F5_2, ":")
  for elem in F5_2:
    print(" ", elem)

if __name__ == "__main__":
  exercise_56()