from sage.all import GF

def lagrange_interpolation(points: list[(int, int)], p: int):
  '''
  Lagrange interpolation for polynomials over F_p[X]
  '''
  Fp = GF(p)
  Fpx = Fp["x"]

  # Create a list of polynomials, initially constant polynomials of 1
  n = len(points)
  L = [Fpx([1]) for _ in range(n)]

  # Compute the Lagrange basis polynomials
  for i in range(n):
    for j in range(n):
      if i != j:
        L[i] *= Fpx([-points[j][0], 1]) # x - points[j][0]
        L[i] /= Fpx([points[i][0] - points[j][0]])

  # Compute the Lagrange interpolating polynomial by summing the lagrange basis polynomials
  P = Fpx(0)
  for i in range(n):
    P += L[i] * points[i][1]

  return P

    
if __name__ == "__main__":
  # points to interpolate
  points = [(0, 4), (-2, 1), (2, 3)]

  # prime for the finite field
  p = 13

  # my local implementation
  lp = lagrange_interpolation(points, p)

  # compare to Sage
  Fp = GF(p)
  Fpx = Fp['x']
  assert(lp == Fpx.lagrange_polynomial(points))
  print(lp)
    