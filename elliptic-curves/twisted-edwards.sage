"""
Affine Twisted Edwards form:

  a(x^2) + y^2 = 1 + d(x^2)(y^2)

"""

from sage.all import GF, Set


class TwistedEdwardsCurve:
    a = 0
    d = 0
    F: GF  # base field
    points: set  # points in curve

    def __init__(self, a, d, prime) -> None:
        F = GF(prime)
        self.a = F(a)
        self.d = F(d)
        self.F = F

        # create points (complexity O(p^2), be careful)
        affine_points = []
        for x in F:
            for y in F:
                if self.in_curve((x, y)):
                    affine_points.append((x, y))
        self.points = Set(affine_points)

    def __str__(self) -> str:
        return "{0} * x^2 + y^2 = 1 + {1} * x^2 * y^2".format(self.a, self.d)

    def add(self, P, Q):
        """
        Add points P and Q in the Twisted Edwards curve.
        """
        x1, x2, y1, y2 = P[0], Q[0], P[1], Q[1]

        x3 = (x1 * y2 + y1 * x2) / (1 + self.d * x1 * x2 * y1 * y2)
        y3 = (y1 * y2 - self.a * x1 * x2) / (1 - self.d * x1 * x2 * y1 * y2)
        return (x3, y3)

    def in_curve(self, P) -> bool:
        """
        Returns true if the given point is in curve.
        """
        return self.a * (P[0] ** 2) + (P[1] ** 2) == self.F(1) + self.d * (
            P[0] ** 2
        ) * (P[1] ** 2)

    def inverse(self, P):
        """
        Inverts a point.
        """
        return (self.F.order() - P[0], P[1])

    def point(self, x, y):
        """
        Return the a point in curve.
        """
        return (self.F(x), self.F(y))


def exercise_75():
    # TinyJubJub parameters
    prime = 13
    a = 3
    d = 8
    TETJJ = TwistedEdwardsCurve(a, d, prime)

    print("\nTwisted Edwards points:")
    points = TETJJ.points
    print(points)

    # part 1
    print("\nInverting points:")
    inverses = list(map(lambda p: TETJJ.inverse(p), points))
    for p, ip in zip(points, inverses):
        assert ip in points
        print("{0} --> {1}".format(p, ip))
    # INF is (0, 1)

    # part 2
    print("\nSolving x + (5, 8) = (1, 11)")
    A, B = TETJJ.point(5, 8), TETJJ.point(1, 11)
    assert TETJJ.in_curve(A)
    assert TETJJ.in_curve(B)

    # X = B + (-A), we have to use addition law
    X = TETJJ.add(B, TETJJ.inverse(A))
    print("X:", X)
    assert TETJJ.in_curve(X)

    # part 3
    print("\nSearching for a generator")
    for point in points:
        runner = point
        i = 1
        while i < (len(points) - 1):
            if runner == (0, 1):
                print("{} is not a generator".format(point))
                i = 1
                break
            runner = TETJJ.add(
                TETJJ.point(runner[0], runner[1]), TETJJ.point(point[0], point[1])
            )
            i += 1
        if i == len(points) - 1:
            assert TETJJ.add(runner, point) == (0, 1)
            print("{} is a generator".format(point))
            break


if __name__ == "__main__":
    exercise_75()
