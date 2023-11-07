def crt(a: list[int], n: list[int]):
    """
    Chinese Remainder Theorem demonstration.
    """
    assert len(a) == len(n)

    # find N as multiplication of all n
    N = 1
    for n_i in n:
        N *= n_i

    # find x'
    xp = 0
    for i in range(len(a)):
        Ndiv = N // n[i]
        _, s, t = xgcd(Ndiv, n[i])
        xp += a[i] * s * Ndiv

    # return x' mod N
    return xp % N


def xgcd(a: int, b: int, verbose: bool = False):
    """
    Extended Euclidean Algorithm

    Given `a` and `b`, finds `gcd(a,b)`, `s`
    and `t` such that `gcd(a, b) = s * a + t * b`.

    Parameters:
    - `a` number
    - `b` number
    - `verbose` print the execution trace

    Returns:
    - `[gcd(a,b), s, t]` as a triple
    """
    assert a >= b

    # initials
    r = [a, b]
    s = [1, 0]
    t = [0, 1]

    k = 2
    while r[k - 1] != 0:
        quot = r[k - 2] // r[k - 1]
        rem = r[k - 2] % r[k - 1]

        r.append(rem)
        s.append(s[k - 2] - quot * s[k - 1])
        t.append(t[k - 2] - quot * t[k - 1])
        k += 1

    if verbose:
        print("{0}|\t{1}\t{2}\t{3}".format("k", "r", "s", "t"))
        for i in range(k):
            print("{0}|\t{1}\t{2}\t{3}".format(i, r[i], s[i], t[i]))

    return r[k - 2], s[k - 2], t[k - 2]


def xgcd_arrayless(a: int, b: int):
    assert a >= b

    r_prev, r_cur = a, b
    s_prev, s_cur = 1, 0
    t_prev, t_cur = 0, 1

    while r_cur != 0:
        quot = r_prev // r_cur
        rem = r_prev % r_cur
        s_next = s_prev - quot * s_cur
        t_next = t_prev - quot * t_cur

        r_prev = r_cur
        r_cur = rem
        s_prev = s_cur
        s_cur = s_next
        t_prev = t_cur
        t_cur = t_next

    return r_prev, s_prev, t_prev
