'''
Affine Montgomery form:

  B(y^2) = x^3 + A(x^2) + x
  
'''

def map_to_short_weierstrass(A, B, x, y):
  '''
  Maps a given affine Montgomery curve point to a Short Weierstrass curve point.
  The considered curve is By^2 = x^3 + Ax^2 + x.
  '''
  return ((3 * x + A) / 3 * B, y / B)