from sage.all import Integers

def exercise_25():
  # polynomial in Z6
  Z6 = Integers(6)
  Z6x.<x> = Z6[] # type: ignore
  # polynomial in ZZ
  ZZx.<x> = ZZ[] # type: ignore

  print("Z6: ", Z6x((x - 2) * (x + 3) * (x - 5)))
  print("ZZ: ", ZZx((x - 2) * (x + 3) * (x - 5)))

def exercise_27():
  # polynomial in Z6
  Z6 = Integers(6)
  Z6x.<x> = Z6[] # type: ignore
  # polynomial in Z5
  Z5 = Integers(5)
  Z5x.<x> = Z5[] # type: ignore
  # polynomial in ZZ
  ZZx.<x> = ZZ[] # type: ignore

  for ring in [Z6x, Z5x, ZZx]:
    print(ring)
    A = ring(-3*x^4 + 4*x^3 + 2*x^2 + 4)
    B = ring(x^2 - 4*x + 2)
    print("A:    ", A)
    print("B:    ", B)
    D = A.quo_rem(B)
    print("A div B:", D[0])
    print("A mod B:", D[1])
    print("")
  
def exercise_30():
  # polynomial in Z6
  Z6 = Integers(6)
  Z6x = Z6['x'] 
  P = Z6x(x^7 + 3*x^6 + 3*x^5 + x^4 - x^3 - 3*x^2 - 3*x - 1) # type: ignore
  print(P)

  # find roots
  roots = P.roots(multiplicities=False)
  print("Roots:")
  print(roots)

  R1 = Z6x(x - 1)
  R5 = Z6x(x - 5)
  
  # ???

  

if __name__ == '__main__':
  exercise_30()