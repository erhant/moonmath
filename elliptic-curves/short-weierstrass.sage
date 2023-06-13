'''
Affine Short Weierstrass form:

  y^2 = x^3 + a(x) + b

Projective Short Weierstrass form:

  Y^2 * Z = X^3 + a * X * Z^2 + b * Z^3

'''

from sage.all import GF, EllipticCurve, factor, Set

def exercise_67():
  # TinyJubJub_13
  E = EllipticCurve(GF(13), [8, 8])

  # points (projective)
  print("Projective Points (TJJ_13):")
  Ps = E.points()
  for p in Ps:
    print(p)
  
  # Get it's largest prime factor
  print("\nCofactor Clearing")
  order = E.order()
  factorization = factor(order)
  lpf = max(factorization)[0]
  # Find co-factor
  cf = order // lpf
  print("\tOrder: {0}\n\tFactorization: {1}\n\tLargest Prime: {2}\n\tCofactor: {3}".format(order, factorization, lpf, cf))

  # Do co-factor clearing
  Ps_sub = [] # subgroup points
  for p in Ps:
    Ps_sub.append(p * cf)
  Ps_sub = Set(Ps_sub)
  # order (#elements) should be equal to largest prime factor
  assert(len(Ps_sub) == lpf)
  print("\nSubgroup Points:")
  for p in Ps_sub:
    print(p)

  # Generator given in exercise
  g = E(7, 11) # [7 : 11 : 1]
  assert(g in Ps_sub) # make sure it is in the subgroup

  # Find log order
  print("\nFinding logarithmic ordering:")
  log_order = [g]
  for _ in range(1, lpf):
    # adding g to itself many times
    log_order.append(log_order[-1] + g)
    
  print("     ", log_order[0])
  for p in log_order[1:]:
    print(" --> ", p)



def example_78():
  '''
  Adding random affine points in secp256k1.
  '''
  p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
  E = EllipticCurve(GF(p), [0, 7])
  
  P = E.random_point()
  Q = E.random_point()
  R = P + Q

  print("P: {0}\nQ: {1}\nR: {2}".format(P.xy(), Q.xy(), R.xy()))

def exercise_63():
  # TinyJubJub_13
  E = EllipticCurve(GF(13), [8, 8])

  # part 1
  print("Inverting points:")
  points = [E(10, 10), E(0), E(4, 0), E(1, 2)]
  inverses = list(map(lambda p : -p, points))
  for p, ip in zip(points, inverses):
    print("{0} --> {1}".format(p, ip))

  # part 2
  print("Solving x + (9, 4) = (5, 2)")
  A, B = E(9, 4), E(5, 2)
  X = B - A
  assert(X + A == B)
  print("X:", X.xy())

def print_points(E, compressed=False):
  '''
  Print all points of the elliptic curve, one per line.
  You can also use the compressed form, where `y` is a single bit
  indicating y is closer to 0 or `p` with 0 or 1 respectively.
  '''
  Ps = E.points()
  
  # if compressed, map y coordinates to 0 or 1
  # based on their closeness to the order of base field
  half = 0
  if compressed:
    half = E.base_field().order() // 2
 
  for p in Ps:
    try:
      affine_point = p.xy()
      if compressed: 
        affine_point = (affine_point[0], 0 if affine_point[1] <= half else 1)
      print("Point on curve:   ", affine_point)
    except ZeroDivisionError:
      print("Point at infinity:", p)
  print("{0} points including point at infinity.".format(len(Ps)))

def print_ordermod(E):
  '''
  Prints the prime modulus and order (number of elements)
  of the elliptic curve.
  '''
  # prime modulus
  prime = E.base_field().order()
  print("Prime:\n{0}\n({1} bits)".format(prime, prime.nbits()))
  # order
  order = E.order()
  print("Order:\n{0}\n({1} bits)".format(order, order.nbits()))

def discriminant(a, b, p):
  '''
  Compute the discriminant
  '''
  res = 4 * (a ** 3) + 27 * b ** 2
  return res % p

def example_snippet():
  # base field
  F5 = GF(5) 
  assert(F5.characteristic() > 2)

  # parameters
  a = F5(2)
  b = F5(4)

  # create the Short Weierstrass curve
  # y^2 = x^3 + ax + b
  E = EllipticCurve(F5, [a, b])
  assert(E.discriminant() != 0)
  # print(discriminant(a, b, F5.order())) # why is this one different?

  # point on curve
  # (2)^2 = (0)^3 + 2*(0) + 4
  P = E(0, 2)
  print("Affine coords of P:", P.xy())

  # point at infinity
  INF = E(0)
  try:
    INF.xy()
  except ZeroDivisionError:
    pass

  # create a plotted version
  P = E.plot()

def example_70():
  '''
  A short Weierstrass curve: y^2 = x^3 + x + 1

  This example prints out all it's points.
  '''
  E = EllipticCurve(GF(5), [1, 1])
  assert(E.discriminant() != 0)

  try:
    P = E(1, 1)
  except TypeError:
    # (1, 1) is not in this curve, which gives type error!
    pass

  print_points(E)

def example_tinyjubjub13():
  '''
  Tiny JubJub curve in FF_13. This is a SNARK-friendly curve. 
  It can also be represented in two alternative forms:
  - Montgomery
  - Twisted Edwards
  '''
  E = EllipticCurve(GF(13), [8, 8])
  assert(E.discriminant() != 0)
  print_points(E)

def example_secp256k1():
  '''
  Secp256k1 is the curve used by Bitcoin.
  It has a very large prime modulus, and number of elements. 
  Both these values are 256-bit numbers.
  '''
  p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
  E = EllipticCurve(GF(p), [0, 7])
  print_ordermod(E) 

def example_alt_bn128():
  '''
  alt_bn128 is the curve used by Ethereum for SNARK verification.
  It is also supported by Circom.
  '''
  p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
  E = EllipticCurve(GF(p), [0, 3])
  print_ordermod(E) 

def exercise_bls12_381():
  '''
  BLS12-381 is another curve supported by Circom (>= v2.0.6).
  This is for Exercise 60 of MoonMath.
  '''
  p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
  E = EllipticCurve(GF(p), [0, 4])
  print_ordermod(E) 

if __name__ == "__main__": 
  exercise_70()