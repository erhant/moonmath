from sage.all import GF


def cge(g: int, x: int, n: int) -> int:
    """
    Cyclic Group Exponentiation using "square-and-multiply"

    Parameters:
    - `g`: generator
    - `x`: exponent
    - `n`: order

    Returns:
    - `g^x (mod n)`
    """
    ans = 1
    base = g
    while x > 0:
        if x & 1 == 1:
            ans = (ans * base) % n  # multiply
        base = (base * base) % n  # square
        x >>= 1
    return ans


def esm(g: int, x: int, n: int) -> int:
    """
    Efficient Scalar Multiplication using "double-and-add"

    Parameters:
    - `g`: number
    - `x`: number
    - `n`: order

    Returns:
    - `g*x (mod n)`
    """
    ans = 0
    base = g
    while x > 0:
        if x & 1 == 1:
            ans = (ans + base) % n  # add
        base = (base + base) % n  # double
        x >>= 1
    return ans


if __name__ == "__main__":
    F13 = GF(13)
    assert F13(4) ^ 7 == cge(4, 7, 13)
    assert F13(4) * 7 == esm(4, 7, 13)
