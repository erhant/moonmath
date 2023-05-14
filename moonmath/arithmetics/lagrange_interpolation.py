from polynomial import Polynomial

def lagrange_interpolation(points: list[(int, int)]):
  # Create a list of n+1 polynomials, each consisting of a single constant term
  # The i-th polynomial is Li(x)
  n = len(points) - 1
  L = [Polynomial([points[i][1]]) for i in range(n+1)]

  # Compute the Lagrange basis polynomials
  for i in range(n+1):
    for j in range(n+1):
      if i != j:
        L[i] = L[i] * Polynomial([-points[j][0], 1])
        L[i] = L[i] / Polynomial([points[i][0] - points[j][0]])

  # Compute the Lagrange interpolating polynomial
  P = Polynomial([0])
  for i in range(n+1):
    P = P + L[i]

  return P

    
if __name__ == "__main__":
  LP = lagrange_interpolation([(0, 4), (-2, 1), (2, 3)]) 
  print(LP)
    