from sage.all import GF, EllipticCurve, factor

def exercise82():
    # curve parameters for TJJ_13
    p = 13
    a, b = 8, 8

    F13 = GF(p)      # field
    F13t.<t> = F13[]  # polynomial ring # type: ignore 
    r = 2            # prime factor


    def pairings(E, E_tor):
        print(E)

        # frobenius
        def fro_pi(P):
            if P != E(0):
                (x, y) = P.xy()
                return E(x^p, y^p)
            else:
                return P

        G1 = [P for P in E_tor if fro_pi(P) == P]
        print("G1:", G1)
        # {(4 : 0 : 1), (0 : 1 : 0)}

        G2 = [P for P in E_tor if fro_pi(P) == p*P]
        print("G2:", G2)

    # curve over the base field
    TJJ = EllipticCurve(F13, [a, b])
    assert TJJ.order() % r == 0

    # r-torsion group over base curve
    TJJ_1_tor = TJJ(0).division_points(r)
    print("2-torsion over p^1 (full)\n\t", TJJ_1_tor)
    pairings(TJJ, TJJ_1_tor)

    # curve over extension field of 4
    F13_4 = GF(p^4, name='t', modulus=F13t.irreducible_element(4)) # type: ignore 
    TJJ_4 = EllipticCurve(F13_4, [a, b])

    # r-torsion group over extended curve
    TJJ_4_tor = TJJ_4(0).division_points(r)
    print("2-torsion over p^4\n\t", TJJ_4_tor)
    pairings(TJJ_4, TJJ_4_tor)

if __name__ == "__main__":
    pass
