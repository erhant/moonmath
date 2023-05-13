from sage.all import GF, EllipticCurve

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

  # exercise 63.1
  print("Inverting points:")
  points = [E(10, 10), E(0), E(4, 0), E(1, 2)]
  inverses = list(map(lambda p : -p, points))
  for p, ip in zip(points, inverses):
    print("{0} --> {1}".format(p, ip))

  print("Solving x + (9, 4) = (5, 2)")
  A, B = E(9, 4), E(5, 2)
  X = B - A
  assert(X + A == B)
  print("X:", X.xy())

    
exercise_63()