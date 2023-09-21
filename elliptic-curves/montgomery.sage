"""
Affine Montgomery form:

  B(y^2) = x^3 + A(x^2) + x

"""

from sage.all import GF, EllipticCurve, Set


class MontgomeryCurve:
    A = 0
    B = 0
    F: GF  # base field
    points: set  # points in curve

    def __init__(self, A, B, prime) -> None:
        F = GF(prime)
        self.A = F(A)
        self.B = F(B)
        self.F = F

        # create points (complexity O(p^2), be careful)
        affine_points = []
        for x in F:
            for y in F:
                if self.in_curve((x, y)):
                    affine_points.append((x, y))
        self.points = Set(affine_points)

    def __str__(self) -> str:
        return "{0} * y^2 = x^3 + {1} * x^2 + x".format(self.B, self.A)

    def add(self, P, Q):
        """
        Add points P and Q in the Montgomery curve.

        If P == Q, tangent law is used.
        If P != Q, chord law is used.
        If P == -Q, method fails.
        """
        assert P in self.points
        assert Q in self.points

        x1, x2, y1, y2 = P[0], Q[0], P[1], Q[1]
        if x1 != x2:
            # chord
            common = (y2 - y1) / (x2 - x1)
        else:
            # tangent
            common = (3 * x1 * x1 + 2 * self.A * x1 + 1) / (2 * self.B * y1)

        x3 = common * common * self.B - (x1 + x2) - self.A
        y3 = common * (x1 - x3) - y1
        assert self.in_curve((x3, y3))
        return (x3, y3)

    def in_curve(self, P) -> bool:
        """
        Returns true if the given point is in curve.
        """
        return self.B * (P[1] ** 2) == (P[0] ** 3) + self.A * (P[0] ** 2) + P[0]

    def inverse(self, P):
        """
        Inverts a point.
        """
        return (P[0], self.F.order() - P[1])

    def point(self, x, y):
        """
        Return the a point in curve.
        """
        return (self.F(x), self.F(y))

    def to_short_weierstrass(self, P):
        """
        Maps a given affine Montgomery curve point to a Short Weierstrass curve point.
        The considered curve is By^2 = x^3 + Ax^2 + x.
        """
        return ((self.F(3) * P[0] + self.A) / 3 * self.B, P[1] / self.B)


def check_montgomery_conversion(E):
    """
    Checks if a curve in Short Weierstrass form can be transformed
    to a Montgomery from.

      - Order of E(F) must be divisible by 4
      - Polynomial z^3 + az + b in F[z] must have at least one root (z_0 in F)
      - 3*(z_0)^2 + a is a quadratic residue in F*
    """

    # check order
    order = E.order()
    orderMod4 = order % 4
    print("{0} mod 4 => {1}".format(order, orderMod4))
    try:
        assert orderMod4 == 0
    except AssertionError:
        print("Number of points on secp256k1 is not divisible by 4!")


###############################################################


def exercise_72():
    # secp256k1
    p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
    E = EllipticCurve(GF(p), [0, 7])
    check_montgomery_conversion(E)


def exercise_73():
    # TinyJubJub parameters
    prime = 13
    B = 7
    A = 6
    MTJJ = MontgomeryCurve(A, B, prime)

    print("\nMontgomery points:")
    points = MTJJ.points
    print(points)

    # part 1
    print("\nInverting points:")
    inverses = list(map(lambda p: MTJJ.inverse(p), points))
    for p, ip in zip(points, inverses):
        assert ip in points
        print("{0} --> {1}".format(p, ip))
    # inverse of INF is INF
    print("INF --> INF")

    # part 2
    print("\nSolving x + (3, 8) = (10, 3)")
    A, B = MTJJ.point(3, 8), MTJJ.point(10, 3)
    assert MTJJ.in_curve(A)
    assert MTJJ.in_curve(B)

    # X = B + (-A)
    X = MTJJ.add(B, MTJJ.inverse(A))
    print("X:", X)
    assert MTJJ.in_curve(X)

    # part 3
    print("\nSearching for a generator")
    #   inf = MTJJ.add(X, MTJJ.inverse(X))
    #   print("DEBUG")
    for point in points:
        runner = point
        i = 1
        while i < (len(points) - 1):
            try:
                # print("DEBUG {}".format(MTJJ.point(point[0], point[1])))
                runner = MTJJ.add(
                    MTJJ.point(runner[0], runner[1]), MTJJ.point(point[0], point[1])
                )
                i += 1
            #       if runner == inf:
            except:
                print("{} is not a generator".format(point))
                i = 1
                break
        if i == len(points) - 1:
            #       assert(MTJJ.add(runner, point) == inf)
            print("{} is a generator".format(point))
            break


def exercise_74():
    # alt_bn128
    p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    E = EllipticCurve(GF(p), [0, 3])
    check_montgomery_conversion(E)


if __name__ == "__main__":
    exercise_73()
