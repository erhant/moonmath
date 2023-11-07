from sage.all import GF, EllipticCurve, factor

def exercise82(max_k: int = 7):
    # curve parameters for TJJ_13
    p = 13
    a, b = 8, 8
    q = 20

    F13 = GF(p)      # field
    F13t.<t> = F13[]  # polynomial ring # type: ignore 
    r = 2            # prime factor


    def pairings(E, E_tor):
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

        G2 = [P for P in E_tor if fro_pi(P) == q*P]
        print("G2:", G2)

    # try for some values of k
    for k in range(1, max_k):
        # curve
        if k == 1:
            # over base field
            TJJ = EllipticCurve(F13, [a, b])
        else:
            # over extension field
            F13_K = GF(p^k, name='t', modulus=F13t.irreducible_element(k)) # type: ignore 
            TJJ = EllipticCurve(F13_K, [a, b])
    

        # r-torsion group over base curve
        TJJ_tor = TJJ(0).division_points(r)
        print("{}-torsion over p^{} ({} elements)\n{}".format(r, k, len(TJJ_tor), TJJ_tor))
        pairings(TJJ, TJJ_tor) 
        print("")

if __name__ == "__main__":
    pass
