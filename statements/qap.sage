from sage.all import GF


def qap(r1cs, p: int):
    """
    Given a R1CS and a prime, returns the QAP (Quadratic Arithmetic Program).
    """
    # k := number of constraints
    k = len(r1cs[0])
    assert k < p

    # make sure lengths are alright
    cnt = len(r1cs[0][0])
    for term in range(3):
        # each term must have k constraints
        # and each constraint must have same amount of terms
        for cons in range(k):
            assert cnt == len(r1cs[term][cons])

    # polynomial over GF(p)
    Fp = GF(p)
    Fpx = Fp["x"]

    # pick k random elements, unique & invertible
    elems = []
    for _ in range(k):
        rand_elem = Fp.random_element()
        if rand_elem not in elems:
            elems.append(rand_elem)

    # compute the target polynomial
    target = Fpx(1)
    for e in elems:
        target *= Fpx([-e, 1])  # x - e

    # compute the lagrange polynomials
    polys = ([], [], [])
    for term in range(3):
        for c in range(cnt):
            points = [(elems[cons], r1cs[term][cons][c]) for cons in range(k)]
            polys[term].append(Fpx.lagrange_polynomial(points))

    return (target, polys)


def exercise_102():
    # prime for finite field of tinyjubjub
    p = 13

    # R1CS of tiny_jj language (from the book)
    r1cs_tiny_jj = (
        # [c, I1, I2, W1, W2, W3]
        [  # A
            [0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 8, 0, 0],
            [1, 0, 0, 10, 12, 1],
        ],
        [  # B
            [0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [1, 0, 0, 0, 0, 1],
        ],
        [  # C
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0],
        ],
    )

    QAP = qap(r1cs_tiny_jj, p)
    print("Target Polynomial")
    print(QAP[0])

    print("Polynomials (A)")
    print(QAP[1][0])

    print("Polynomials (B)")
    print(QAP[1][1])

    print("Polynomials (C)")
    print(QAP[1][2])


def example_131():
    p = 13

    # R1CS of 3.fac_zk language (from the book)
    r1cs_3faczk = (
        # [c, I1, W1, W2, W3, W4]
        [  # A
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
        ],
        [  # B
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 1, 0],
        ],
        [  # C
            [0, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0],
        ],
    )

    QAP = qap(r1cs_3faczk, p)
    print(QAP)


if __name__ == "__main__":
    exercise_102()
