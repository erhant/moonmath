class Polynomial:
  coeffs: list[int]
  def __init__(self, coeffs: list[int]):
    self.coeffs = coeffs

  def degree(self) -> int:
    return len(self.coeffs) - 1

  def leading_coefficient(self) -> int:      
    return self.coeffs[-1]

  def __str__(self) -> str:
    s = []
    for i, c in enumerate(self.coeffs):
      if i == 0:
        s.append("{0}".format(c))
      elif i == 1:
        s.append("{0}x".format(c))
      else:
        s.append("{0}x^{1}".format(c, i))
    return ' + '.join(reversed(s))
  
  def __add__(self, Q: "Polynomial"):
    # add polynomials
    degP = self.degree()
    degQ = Q.degree()
    degMin = min(degP, degQ)

    coeffs = [self.coeffs[i] + Q.coeffs[i] for i in range(degMin+1)]
    if degQ == degP:
      # no more coefficients
      pass
    elif degQ == degMin:
      # rest of the coefficients are at P
      coeffs.extend(P.coeffs[degMin + 1:])
    elif degP == degMin:
      # rest of the coefficients are at Q
      coeffs.extend(Q.coeffs[degMin + 1:])
      
    return Polynomial(coeffs)
  
  def __mul__(self, Q: "Polynomial"):
   # multiply polynomials
    m, n = self.degree(), Q.degree()
    coeffs = [0] * (m+n+1)
    for i in range(m+1):
      for j in range(n+1):
        coeffs[i+j] += self.coeffs[i] * Q.coeffs[j]
    return Polynomial(coeffs)

  def __divmod__(self, Q: "Polynomial"):
    # poly-long-division helper method
    r = Polynomial(self.coeffs)
    d = Q.leading_coefficient()
    n = self.degree() - Q.degree()
    q = [0] * (n+1)
    for k in range(n, -1, -1):
      q[k] = r.leading_coefficient() // d
      s = Polynomial([0] * k + [q[k]])
      r = r - s * Q
    return Polynomial(q), r

  def __div__(self, Q: "Polynomial"):
    # poly-long-division
    return divmod(self, Q)[0]

  def __mod__(self, Q: "Polynomial"):
    # compute the remainder after polynomial division
    return divmod(self, Q)[1]

  def __sub__(self, Q: "Polynomial"):
    # subtract polynomials
    return self + (-Q)

  def __neg__(self):
    # negate polynomial
    return Polynomial([-c for c in self.coeffs])
  
if __name__ == "__main__":
  P = Polynomial([1, 2])
  Q = Polynomial([4])
  print("P:", P)
  print("Q:", Q)

  # addition
  print("P + Q:", P + Q)

  # subtraction
  print("P - Q:", P - Q)

  # multiplicaiton
  print("P * Q", P * Q)

  
    
  