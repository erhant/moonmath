
class Polynomial:
  coeffs: list[int]
  # constructor
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
        s.append("({0})".format(c))
      elif i == 1:
        s.append("({0})*x".format(c))
      else:
        s.append("({0})*x^{1}".format(c, i))
    return ' + '.join(reversed(s))
  
  def __add__(self, Q: "Polynomial"):
    P: "Polynomial" = self

    # number of coefficients
    lenP = P.degree() + 1
    lenQ = Q.degree() + 1

    # add overlapping coefficients
    coeffs: list[int] = []
    for i in range(min(lenP, lenQ)):
      coeffs.append(Q.coeffs[i] + P.coeffs[i])

    # append the rest
    if (lenP < lenQ):
      coeffs.extend(Q.coeffs[len(coeffs):])
    elif (lenP > lenQ):
      coeffs.extend(P.coeffs[len(coeffs):])

    return Polynomial(coeffs)
  
  def __mul__(self, Q: "Polynomial"):
    raise NotImplemented()
  
if __name__ == "__main__":
  P = Polynomial([1, 2])
  Q = Polynomial([4])
  print((P + Q))
    
  