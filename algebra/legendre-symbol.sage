from sage.all import GF


def legendre_symbol(y, p):
    """
    Legendre Symbol

    If the result is 1, then y is a quadratic residue in p.
    """
    assert p % 2 == 1
    l = y ^ ((p - 1) // 2) % p
    return -1 if l == p - 1 else l


def exercise_52():
    F13, legendres, sqrts = GF(13), {}, {}
    for e in F13:
        sqrts[e] = []
    for e in F13:
        legendres[e] = legendre_symbol(e, 13)
        sqrts[e * e].append(e)

    for e in F13:
        print("N = {}\tSymbol: {}\tSqrts: {}".format(e, legendres[e], sqrts[e]))


if __name__ == "__main__":
    exercise_52()
