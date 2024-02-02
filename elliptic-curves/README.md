# Chapter 5: Elliptic Curves

Also see this <https://curves.xargs.org/> for great animations, especially about Chord & Tangent rule. Furthermore, I have drawn the example TinyJubJub curves in WolframAlpha:

- [Short Weierstrass](https://www.wolframalpha.com/input?i2d=true&i=Power%5By%2C2%5D%3DPower%5Bx%2C3%5D%2B+8x+%2B8+for+x%5C%2844%29y+in+GF%5C%2840%2913%5C%2841%29)
- [Montgomery](https://www.wolframalpha.com/input?i2d=true&i=7Power%5By%2C2%5D%3DPower%5Bx%2C3%5D%2B6Power%5Bx%2C2%5D%2Bx+for+x%5C%2844%29+y+in+GF%5C%2840%2913%5C%2841%29)
- [Twisted Edwards](https://www.wolframalpha.com/input?i2d=true&i=3Power%5Bx%2C2+%5D%2B+Power%5By%2C2%5D+%3D+1+%2B+8+Power%5Bx%2C2%5D+Power%5By%2C2%5D+for+x%5C%2844%29+y+in+GF%5C%2840%2913%5C%2841%29)

## Exercise 58

> Compute the set of all points $(x, y) \in E_{1, 1}(\mathbb{F}_5)$ from example 70.

Using Sage, we can check for pairs of field elements that satisfy the curve equation.

```py
F5 = GF(5)
points = ['O'] # point at infinity
for x in F5:
  for y in F5:
    if y ** 2 == x ** 3 + x + 1:
      points.append((x, y))
print(points)
print(len(points), "points")
```

The set of all points is the one computed above, together with the point at infinity.

## Exercise 59

> Compute the set of all points $(x, y) \in TJJ_{13}$ example 71.

Using Sage, we can check for pairs of field elements that satisfy the curve equation.

```py
F13 = GF(13)
points = ['O'] # point at infinity
for x in F13:
  for y in F13:
    if y**2 == x**3 + 8*x + 8:
      points.append((x, y))
print(points)
print(len(points), "points")
```

The set of all points is the one computed above, together with the point at infinity.

## Exercise 60

> Look up the definition of curve BLS12-381, implement it in Sage, and compute the number of all curve points.

```py
p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
E = EllipticCurve(GF(p), [0, 4])

base_order, scalar_order = E.base_field().order(), E.order()
print("Base Field Order:\n{0}\n({1} bits)".format(base_order, base_order.nbits()))
print("Scalar Field Order:\n{0}\n({1} bits)".format(scalar_order, scalar_order.nbits()))
```

The number of all curve points is given by the scalar field order, which is 52435875175126190479447740508185965838148530120832936978733365853859369451521.

## Exercise 61

> Let $\mathbb{F}$ be a finite field, let $(a, b)$ and $(a', b')$ be two pairs of parameters, and let $c \in \mathbb{F}^*$ be an invertible field element such that $a' = a \cdot c^4$ and $b' = b \cdot c^6$ hold. Show that the function $I$ from equation (5.3) maps curve points onto curve points.

Let us write $x' := c^2 \cdot x$ and $y' := c^3 \cdot y$ as well as $a'$ and $b'$ in the curve equation:

$$
(c^3 \cdot y)^2 = (c^2 \cdot x)^3 + (c^4 \cdot a)(c^2 \cdot x) + (c^6 \cdot b)
$$

$$
c^6 \cdot y^2 = c^6 \cdot x^3 + c^6 \cdot a \cdot x + c^6 \cdot b
$$

Notice how we have $c^6$ on both sides, and we are told that $c$ has a multiplicative inverse, and therefore we can compute $c^{-6} = (c^{-1})^6$ and multiply both sides with it to obtain:

$$
y^2 = x^3 + a \cdot x + b
$$

which is our original curve equation, thus showing that the points are mapped onto curve points.

## Exercise 62 ✨

> Consider $TJJ_{13}$ example 71 and the curve $E_{7, 5}(\mathbb{F}_{13})$ defined as follows:
>
> $$
> E_{5, 7}(\mathbb{F}_{13})
> =    \{(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13}
> \mid y^2 = x^3 + 7x + 5\}
> $$
>
> Show that $TJJ_{13}$ and $E_{7, 5}(\mathbb{F}_{13})$ are isomorphic. Compute the set of all points from $E_{7, 5}(\mathbb{F}_{13})$, construct $I$ and map all points of $TJJ_{13}$ onto $E_{7, 5}(\mathbb{F}_{13})$

Let's remember the TinyJubJub curve over field of prime order 13:

$$
TJJ_{13} = \{(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13} \mid y^2 = x^3 + 8x + 8\}
$$

If we can find a $c \in \mathbb{F}_{13}^*$ such that $7 = 8 \times c^4$ (for $a$) and $5 = 8 \times c^6$ (for $b$); then, as shown in exercise 61, we would have an isomorphism and a mapping from TinyJubJub to the other curve.

The inverse of 8 in this field is 5, so lets multiply both sides with it in both equations to obtain $9 = c^4$ and $12 = c^6$. From this, we get $12 = 9 \times c^2$ and find $c^2 = 10$. Turns out that $6 \times 6 \equiv 10 \pmod{13}$ so $c = 6$ is a square root, as well as $c = 13 - 6 = 7$ (which is the negative root).

Equation (5.3) gives us the following isomorphism:

$$
I : TJJ_{13}(\mathbb{F}_{13}) \to E_{7, 5}(\mathbb{F}_{13})  : \begin{cases}
(x, y) \\
\mathcal{O}
\end{cases}

\mapsto

\begin{cases}
(10\cdot x, 8\cdot y) \\
\mathcal{O}
\end{cases}
$$

Let's use Sage to confirm this:

```py
F13 = GF(13)
TJJ = EllipticCurve(F13, [8, 8])
E75 = EllipticCurve(F13, [7, 5])

def I(xy):
  return (xy[0] * F13(10), xy[1] * F13(8))

TJJ_pts = [p.xy() for p in TJJ.points() if p != TJJ(0)]
E75_pts = [p.xy() for p in E75.points() if p != E75(0)]
I_TJJ_E75pts = [I(pt) for pt in TJJ_pts]
print(I_TJJ_E75pts == E75_pts)
```

Indeed when we run this, we see `True` as the result!

## Exercise 63

> Consider the commutative group defined by the affine group law and TinyJubJub with base field $\mathbb{F}_{13}$.
>
> 1. Compute the inverse of $(10, 10), \mathcal{O}, (4, 0), (1, 2)$
> 2. Solve the equation $x \oplus (9, 4) = (5, 2)$ for some point $x$ on the curve.

The inverse of $(x, y)$ is given by $(x, -y)$ unless the point is the point at infinity. So:

- $-(10, 10) = (10, 3)$
- $-\mathcal{O} = \mathcal{)}$
- $-(4, 0) = (4, 0)$
- $-(1, 2) = (1, 11)$

Let us solve the equation, which can be shown as:

$$
x = (5, 2) \oplus (9, 9)
$$

We will use the Chord rule to do this addition as the points are different. Letting $x = (x', y')$ for these equations:

$$
x' = \left(\frac{9-2}{9-5}\right)^2 - 5 - 9 = 5 \times 5 - 1 \equiv 11 \pmod{13}
$$

$$
y' = \left(\frac{9-2}{9-5}\right)(5 - 11) - 2 = 5 \times 7 - 2 = 33 \equiv 7 \pmod{13}
$$

Our result is $x = (11, 7)$. We can confirm with Sage:

```py
E = EllipticCurve(GF(13), [8, 8])
(E(5, 2) - E(9, 4)).xy()
# (11, 7)
```

## Exercise 64 ⚠️

> Consider example 79 and compute the set $\{[1](0, 1), [2](0, 1), \ldots, [8](0, 1), [9](0, 1)\}$ using the tangent rule only.

**Isn't this done in example 79 already?**

## Exercise 65 🔴

> Consider example 80 and compute the scalar multiplications $[10](5, 11)$ as well as $[10](9, 4)$ and $[4](9, 4)$ with pen and paper using the algorithm from exercise 38 (Efficient Scalar Multiplication).

TODO

## Exercise 66

> Consider example 81 and compute the set shown in equation (5.23) by inserting all points from the projective plane $\mathbb{F}_5\mathbb{P}^2$ into the defining projective Short Weierstrass equation.

The equation that we have to satisfy over projective points is the following:

$$
\forall (x, y, z) \in [X : Y : Z] : y^2z = x^3 + 1\cdot xz^2 + 1 \cdot z^3
$$

Using Sage, we can try the equation with $z=1$, since we know there are not solutions for $z=0$ other than point at infinity:

```py
F5 = GF(5)
E = EllipticCurve(F5, [1, 1])
def eqn(x, y, z):
  return y^2 * z == x^3 + x * z^2 + z^3
affine_points = [p.xy() for p in E.points() if p != E(0)]
proj_points = [(p[0], p[1], 1) for p in affine_points if eqn(p[0], p[1], F5(1))]
```

We find the points (point at infinity ignored):

```py
print(proj_points)
# [(0, 1, 1), (0, 4, 1), (2, 1, 1), (2, 4, 1), (3, 1, 1), (3, 4, 1), (4, 2, 1), (4, 3, 1)]
```

Alternatively, we can use `ProjectiveSpace` within Sage (thanks to [@skaunov](https://github.com/skaunov) for letting me know about `ProjectiveSpace` in issue [#1](https://github.com/erhant/moonmath/issues/1)):

```py
F5 = GF(5)
F5P2 = ProjectiveSpace(F5, 2)
points = [(x, y, z) for (x, y, z) in F5P2 if y^2 * z == x^3 + x * z^2 + z^3]
```

## Exercise 67 ✨

> Compute the projective representation of the TinyJubJub curve with base field $\mathbb{F}_{13}$. Then, print the logarithmic order of its large prime-order subgroup with respect to the generator $[ 7 : 11 : 1 ]$.

Let's begin by finding the co-factor:

```py
TJJ = EllipticCurve(GF(13), [8, 8])
order = TJJ.order()
factorization = factor(order)
lpf = max(factorization)[0] # largest-prime factor
cf = order // lpf # co-factor
```

Now let's do co-factor clearing:

```py
Esub = set([p * cf for p in TJJ.points()])
assert(len(Esub) == lpf) # order should be equal to lpf
```

Let's also make sure our generator for the exercise is in this subgroup:

```py
g = TJJ(7, 11) # [7 : 11 : 1]
assert(g in Esub) # make sure it is in the subgroup
```

We can then add the generator to itself many times to find the logarithmic order:

```py
log_order = [g]
for _ in range(1, lpf):
  log_order.append(log_order[-1] + g)
```

When we print the logarithmic order, we find:

$$
(7, 11) \to (8, 5) \to (8, 8) \to (7, 2) \to \mathcal{O}
$$

which has 5 elements, as expected from the large prime-order subgroup of $TJJ_{13}$ which has order 5.

## Exercise 68 🔴

> Consider example 81 again. Compute the following expressions for projective points $E_1(\mathbb{F}_5\mathbb{P}^2)$ using algorithm 7.
>
> - $[0 : 1 : 0] \oplus [4 : 3 : 1]$
> - $[0 : 3 : 0] \oplus [3 : 1 : 2]$
> - $-[0 : 4 : 1] \oplus [3 : 4 : 1]$
> - $[4 : 3 : 1] \oplus [4 : 2 : 1]$
>
> and then solve the equation $[X : Y : Z] \oplus [0 : 1 : 1] = [2 : 4 : 1]$ for some point from the projective Short Weierstrass curve $E_1(\mathbb{F}_5\mathbb{P}^2)$

TODO

## Exercise 69

> Compare the affine addition law for Short Weierstrass curves with the projective addition rule. Which branch in the projective branch corresponds to which case in the affine law?

The first two if-else's handle the addition with neutral element (point at infinity) case.

Then, we have a major branching after $V_1 = V_2$ check, which is true if $X_1 = X_2$. If $X$ coordinates match for two points, they are either the same point or on the opposite sides of the curve:

- $U_1 \ne U_2$ is true if these points are on the opposite sides, and we know this means $Y_1 = -Y_2$ and their addition results in $\mathcal{O}$
- $U_1 = U_2$ means this is the same point, and we apply the Tangent rule. However, there is one case where the tangent rule results in point at infinity, and that is when $Y = 0$ which is checked by $Y_1 = 0$ in the algorithm.

Naturally, the remaining branch (where $V_1 \ne V_2$) handles the Chord rule where we add two different points with different $X$ coordinates.

## Exercise 70 🔴

> Consider example 82 and compute the set in (5.30) by inserting every pair of field elements $(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13}$ into the defining Montgomery equation.

TODO

## Exercise 71 🔴

> Consider $E_1(\mathbb{F})$ from example 70 and show that this curve is not a Montgomery curve.

TODO

## Exercise 72

> Show that `secp256k1` is not a Montgomery curve.

One of the conditions to be a Montogomery curve is that order of the scalar field should be divisible by 4.

```py
# order of the base field
p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
E = EllipticCurve(GF(p), [0, 7])

E.order() % 4 # returns 1
```

As we can see, the order of the scalar field is not divisible by 4, therefore `secp256k1` is not a Montgomery curve.

## Exercise 73 ✨

> Consider the commutative group defined by the Montgomery group law and TinyJubJub with base field $\mathbb{F}_{13}$ in Montgomery form.
>
> - Compute the inverse of $(1, 9), \mathcal{O}, (7, 12), (4, 9)$.
> - Solve the equation $x \oplus (3, 8) = (10, 3)$ for some point in the Montgomery curve.
>
> Finally, choose some point in the curve and test to see if it is a generator. If not, keep trying until you find one. Print out that generator point's logarithmic order.

See the code [here](./montgomery.sage).

## Exercise 74

> Show that `alt_bn128` is not a Montgomery curve.

One of the conditions to be a Montogomery curve is that order of the scalar field should be divisible by 4.

```py
# order of the base field
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
E = EllipticCurve(GF(p), [0, 3])
E.order() % 4 # returns 1
```

As we can see, the order is not divisible by 4, therefore `alt_bn128` is not a Montgomery curve.

## Exercise 75 ✨

> Consider the commutative group defined by the Twisted Edwards group law and TinyJubJub with base field $\mathbb{F}_{13}$ in Twisted Edwards form.
>
> - Compute the inverse of $(1, 11), (0, 1), (3, 0), (5, 8)$.
> - Solve the equation $x \oplus (5, 8) = (1, 11)$ for some point in the Montgomery curve.
>
> Finally, choose some point in the curve and test to see if it is a generator. If not, keep trying until you find one. Print out that generator point's logarithmic order.

See the code [here](./twisted-edwards.sage).

## Exercise 76

> Consider the short Weierstrass curve $y^2 = x^3 + x + 1$ over extension field $\mathbb{F}_{5^2}$. Compute $(4t + 3, 2t + 1) \oplus (3t + 3, 2)$, and double-check the result in sage. Then, solve the equation $x \oplus (3t + 3, 3) = (3, 4)$ for some $x$ in the curve. Also, compute $[5](2t + 1, 4t + 4)$.

Here is the solution in Sage:

```py
F5 = GF(5)              # field
F5t.<t> = F5[]          # polynomial ring
P_MOD_2 = F5t(t^2 + 2)  # irreducible polynomial

# extension field
F5_2 = GF(5^2, name='t', modulus=P_MOD_2)

# curve over extension field
E1F5_2 = EllipticCurve(F5_2, [1, 1])

print(
  "(4t+3, 2t+1) + (3t + 3, 2) =",
  (E1F5_2(4*t + 3, 2*t + 1) + E1F5_2(3*t + 3, 2)).xy())
print(
  "x + (3t + 3, 3) = (3, 4)\tx =",
  (E1F5_2(3, 4) - E1F5_2(3*t + 3, 2)).xy())
print(
  "[5](2t + 1, 4t + 4) =",
  (5 * E1F5_2(2*t + 1, 4*t + 4)).xy())
```

## Exercise 77

> Consider TinyJubJub. Show that $t^4 + 2 \in \mathbb{F}_{13}[t]$ is irreducible.
>
> Then, write a sage program to implement the finite field extension $\mathbb{F}_{13^4}$. Implement the curve extension in the extension field, and compute the number of curve points (i.e. order).

Here is the solution in Sage:

```py
F13 = GF(13)              # field
F13t.<t> = F13[]          # polynomial ring
P_MOD_4 = F13t(t^4 + 2)   # irreducible polynomial
assert P_MOD_4.is_irreducible()

# extension field
F13_4 = GF(13^4, name='t', modulus=P_MOD_4)

# TinyJubJub over the extension field
TJJ_F13_4 = EllipticCurve(F13_4, [8, 8])
print("Order of E(F_13^4):", TJJ_F13_4.order())
```

## Exercise 78 ✨

> Consider `alt_bn128` curve. We know from example 89 that this curve has embedding degree 12.
>
> - Use Sage to find an irreducible polynomial in $\mathbb{F}_p[t]$
> - Then compute the field extension $\mathbb{F}_{p^{12}}$ to implement the curve extension of `alt_bn128`. Compute the number of curve points.

Here is the solution in Sage:

```py
# curve parameters for alt_bn128
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
a, b = 0, 3

FP = GF(p)      # field
FPt.<t> = FP[]  # polynomial ring

k = 12 # embedding degree
P_MOD_K = FPt.irreducible_element(k) # an irreducible polynomial of degree k
print("Irreducible polynomial:\n", P_MOD_K)

# extension field
FP_K = GF(p^k, name='t', modulus=P_MOD_K)

# curve over extension field
E = EllipticCurve(FP_K, [a, b])

print("Order of alt_bn128 extension:\n", E.order())
```

## Exercise 79 ✨

> Consider the full 5-torsion group $TJJ_{13}[5]$ from example 92.
>
> - Write down the set of all elements from this group, and identify the subset of all elements from $TJJ_{13}(\mathbb{F}_{13})[5]$ as well as $TJJ_{13}(\mathbb{F}_{13^2})[5]$.
> - Then compute the 5-torsion group $TJJ_{13}(\mathbb{F}_{13^8})[5]$.

First, let's compute the full 5-torsion group $TJJ_{13}[5]$ as shown in the example. For a full $r$-torsion group, we need the curve defined over the extension field over a polynomial with degree equal to $k(r)$. We know from a previous example that $k(5) = 4$ so we will use a degree 4 polynomial.

```py
F13 = GF(13)          # field
F13t.<t> = F13[]      # polynomial ring

# degree 4 irreducible polynomial
P_MOD_4 = F13t(t^4 + 2)
assert P_MOD_4.is_irreducible()

# extension field
F13_4 = GF(13^4, name='t', modulus=P_MOD_4)

# curve over the extension
TJJF13_4 = EllipticCurve(F13_4, [8, 8])

# full 5-torsion group, that is the
# set of points P such that [5]P == INF
TJJF13_4_5 = Set(TJJF13_4(0).division_points(5))
print("Number of elements:", TJJF13_4_5.cardinality()) # 25
```

From the definition of $r$-torsion groups, we know that the following holds:

$$
TJJ_{13}(\mathbb{F}_{13})[5] = TJJ_{13}(\mathbb{F}_{13^2})[5] \subset TJJ_{13}(\mathbb{F}_{13^4})[5] = TJJ_{13}(\mathbb{F}_{13^8})[5]
$$

Remember that this is because 4 is the embedding degree. So, with our code so far we have already computed the 5-torsion group $TJJ_{13}(\mathbb{F}_{13^8})[5]$. We can compute the other one over the curve itself, without the field extension.

```py
# curve over the field
TJJ = EllipticCurve(F13, [8, 8])

# 5-torsion group
TJJ_5 = Set(TJJ(0).division_points(5))
print("Number of elements:", TJJ_5.cardinality()) # 5
```

The subset of all elements from $TJJ_{13}(\mathbb{F}_{13})[5]$ and $TJJ_{13}(\mathbb{F}_{13^2})[5]$ are those that are simply composed of field elements, not polynomials. We can just print the points on $TJJ_{13}(\mathbb{F}_{13})[5]$ to see them:

```py
print(TJJ_5)
# (7 : 11 : 1)
# (8 : 8 : 1)
# (7 : 2 : 1)
# (8 : 5 : 1)}
# (0 : 1 : 0) which is INF
```

## Exercise 80

> Consider `secp256k1` curve and it's full $r$-torsion group. Write down a single element from the curve's full torsion group that is not the point at infinity.

In example 93, we learn the following:

> ..., without any optimizations, representing such an element would need $k \cdot 256$ bits, which is too much to be representable in the observable universe. It follows that it is not only infeasible to compute the full $r$-torsion group of $\text{secp256k1}$, but moreover to even write down single elements of that group in general.

So, the question boils down to some optimization. Futher in the exercise 96 at the end of next section, it mentions the following:

> ... according to example 93 we can not store average curve points from the extension curve $\text{secp256k1}(F_{p^k})$ on any computer, ...

We are looking for a point other than the point at infinity, and with the knowledge we have so far that seems to be impossible.

However, we could maybe find a value from the curve itself instead of an extension field, which would be in the full $r$-torsion group due to the subset rule. Also note that $r$ here is equal to the scalar order, because the order of `secp256k1` is a prime meaning that the order itself is the largest prime factor.

```py
p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
E = EllipticCurve(GF(p), [0, 7])
INF = E(0)
# embedding degree
k = 19298681539552699237261830834781317975472927379845817397100860523586360249056

# order of scalar field
q = E.order()
assert is_prime(q)

# try for some random points
for _ in range(200):
  assert E.random_point() * q == INF
```

Indeed any point within the original curve is a member of the torsion group $E(\mathbb{F}_p)[r]$ and we know that this is a subset of the full-torsion group!

## Exercise 81 🔴

> Consider `alt_bn128` curve and and it's full $r$-torsion group. Write a Sage program that computes a generator from the curve's full torsion group.

First of all, we should notice that $r$ is equal to the order of the scalar field of `alt_bn128` since that order is a prime and it is the largest prime factor on its own.

We also know from example 89 that this curve has an embedding degree of 12. So, we must compute the curve over the extension field with a degree 12 polynomial. Let's do that in Sage:

```py
# curve parameters for alt_bn128
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
a, b = 0, 3

FP = GF(p)      # field
FPt.<t> = FP[]  # polynomial ring
k = 12          # embedding degree

# an irreducible polynomial of degree k
P_MOD_K = FPt.irreducible_element(k)
assert P_MOD_K.is_irreducible()

# extension field
FP_K = GF(p^k, name='t', modulus=P_MOD_K)

# curve over extension field
E_K = EllipticCurve(FP_K, [a, b])

# curve over the base field
E = EllipticCurve(FP, [a, b])
```

It is stated that the full $r$-torsion group has order of $r^2$, we can use this to test whether our point is generator or not.

```py
# order of the scalar field
q = E.order()

# expected order of the full r-torsion group
qq = q^2
```

TODO: couldnt solve this yet

## Exercise 82

> Consider the small prime factor 2 of the TinyJubJub curve. Compute the full 2-torsion group of $TJJ_{13}$ and then compute the groups $\mathbb{G}_1[2]$ and $\mathbb{G}_2[2]$.

First, let's find the embedding degree $k(2)$ for this curve.

```py
# order of the base field for TJJ
p = 13
F13 = GF(p)
TJJ = EllipticCurve(F13, [8, 8])

# order of the curve's scalar field
n = TJJ.order()
# small prime factor
r = 2
assert n % r == 0

# find embedding degree
k = 1
while k < r:
  if (p^k - 1) % r == 0:
    break
  k += 1
print("Embedding degree:", k)
```

We find the embedding degree to be 1. In fact, you can immediately say that the embedding degree is 1, because notice that following operation in the congruence:

$$
13^k - 1 \bmod{2}
$$

We are looking for the smallest $k$ that results in 0 for the above operation, and it is obvious that $13-1$ is an even number and thus $k=1$. We also know this result from example 87 by the way.

Yet another argument is that an embedding degree $1 \leq k < r$ is guaranteed for the prime order due to FLT, and we can only have $k=1$ for $r=2$ anyways.

To compute the **full** 2-torsion group, we need to find the 2-torsion group of the curve over field extension with order $p^{k(2)}$. We have just shown that $k(2) = 1$, so it turns out that our original curve serves the purpose to find the full torsion group! We can simply choose points $P$ such that $[2]P = \mathcal{O}$ using Sage:

```py
# full r-torsion group, using the original curve
TJJ_1_tor = Set(TJJ(0).division_points(r))
print("{}-torsion group:".format(r))
print(TJJ_1_tor)
# {(4 : 0 : 1), (0 : 1 : 0)}
```

> [!IMPORTANT]
>
> We would expect $r^2$ elements (i.e. 4) in the full-torsion group, which is NOT the case here! After lengthy discussions with @bufferhe4d and his further discussions with more people, we have come to conclusion that the $r^2$ requirement is not strict when $k=1$. In some cases, we can have $r$ elements.

Let's compute the pairing groups now:

```py
def fro_pi(P):
  if P != INF:
    (x, y) = P.xy()
    return TJJ(x^p, y^p)
  else:
    return P

G1 = [P for P in TJJ_1_tor if fro_pi(P) == P]
print("G1:", G1)
# {(4 : 0 : 1), (0 : 1 : 0)}

G2 = [P for P in TJJ_1_tor if fro_pi(P) == p*P]
print("G2:", G1)
# {(4 : 0 : 1), (0 : 1 : 0)}
```

> [!NOTE]
>
> Regarding the note above about finding $r$ elements instead of $r^2$, there is also another thing to mention. If you find the 2-torsion group of TJJ over $\mathbb{F}_{p^4}$ you do actually get $r^2$ elements. 4 is the first time this happens, where the torsion group has 2 elements up until this point and has 4 elements beyond this point.
>
> This would be in-line with (5.44) in the book; however, 4 is not the embedding degree for $r=2$ in this case.
>
> Regardless of this fact, the pairing groups for the torsion group at $k=4$ is equal to the pairing groups computed for $k=1$ in this exercise!
>
> See this [code](./pairings.sage) here for a better presentation of this exercise.
>
> I have opened an issue about this exercise and the things discussed in particular: [see here](https://github.com/LeastAuthority/moonmath-manual/issues/84).

## Exercise 83 🔴

> Consider `alt_bn128` curve and and it's curve extension. Write a Sage program that computes a generator for each of the torsion group $\mathbb{G}_1[p]$ and $\mathbb{G}_2[p]$.

Note here that instead of $r$ we have used $p$ in the question. This refers to the characteristic $p$, which is the characteristic of this elliptic curve. Since the curve order is prime, we have $p=r$ so this exercise is actually very similar to previous exercise 81.

First let's do our setup:

```py
# curve parameters for alt_bn128
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
a, b = 0, 3

FP = GF(p)      # field
FPt.<t> = FP[]  # polynomial ring
k = 12          # embedding degree

# an irreducible polynomial of degree k
P_MOD_K = FPt.irreducible_element(k)

# extension field
FP_K = GF(p^k, name='t', modulus=P_MOD_K)

# curve over extension field
E_K = EllipticCurve(FP_K, [a, b])

# curve over the base field
E = EllipticCurve(FP, [a, b])
```

Generator of $\mathbb{G}_1$ is easy to find since a generator of the curve serves that purpose when $r$ is equal to the scalar field order, as also shown in exercise 81.

TODO: calculate gen for G2?

## Exercise 84 🔴

> Consider the `alt_bn128` curve from example 73, and the generators $g_1$ and $g_2$ of $\mathbb{G}_1[p]$ and $\mathbb{G}_2[p]$ from exercise 83. Write a Sage program that computes the Weil pairing $e(g_1, g_2)$

TODO

## Exercise 85 🔴

> Use our definition of the `try-hash` algorithm to implement a hash function $H_{TJJ[5]} : \{0, 1\}^\ast \to TJJ(\mathbb{F}_{13})[5]$ that maps binary strings of arbitrary length onto the 5-torsion group of $TJJ(\mathbb{F}_{13})[5]$

TODO

## Exercise 86 🔴

> Implement a cryptographic hash function $H_{\text{secp256k1}} : \{0, 1\}^* \to \text{secp256k1}$ that maps binary strings of arbitrary length onto the elliptic curve `secp256k1`.

```py
# parameters for secp256k1
p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
a, b = 0, 7

Fp = GF(p)
E = EllipticCurve(Fp, [a, b])
```

TODO

## Exercise 87

> Consider `alt_bn128` curve. Write a Sage program that computes the trace of Frobenius for `alt_bn128`. Does the curve contain more or less elements than its base field $\mathbb{F}_p$?

```py
# curve parameters
p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
a, b = 0, 3
Fp = GF(p)
E = EllipticCurve(Fp, [a, b])

# order of the scalar field
q = E.order()

# trace of Frobenius
t = p + 1 - q
print(t)

if q < p:
  print("curve contains less elements than Fp")
else:
  print("curve contains more elements than Fp")
```

We see that the curve `alt_bn128` contains less elements than its base field.

## Exercise 88 🔴

> Consider `alt_bn128` curve. Write a Sage program that computes the $j$-invariant for `alt_bn128`.

TODO

## Exercise 89 🔴

> Show that the Hilbert class polynomials for the CM-discriminants $D = -3$ and $D = -4$ are given by $H_{-3, q}(x) = x$ and $H_{-4, q}(x) = x - (1728 \bmod{q})$

TODO

## Exercise 90 🔴

> Use the complex multiplication method to construct an elliptic curve of order 7 over the prime field $\mathbb{F}_{13}$

TODO

## Exercise 91 🔴

> Use the complex multiplication method to compute all isomorphism classes of all elliptic curves of order 7 over the prime field $\mathbb{F}_{13}$

TODO

## Exercise 92 🔴

> Consider the prime modulus $p$ of curve `alt_bn128` from example 73, and its trace $t$ from exercise 92. Use the complex multiplication method to synthesize an elliptic curve over $F_p$ that is isomorphic to `alt_bn128` and compute an explicit isomorphism between these two curves.

TODO

## Exercise 93 🔴

> Consider the point $P = (9, 2)$. Show that $P$ is a point on the `BLS6_6` curve and compute the scalar product $[3]P$

TODO

## Exercise 94 🔴

> Compute the following expressions:
>
> - $-(26, 34)$
> - $(26, 9) \oplus (13, 28)$
> - $(35, 15) \oplus \mathcal{O}$
> - $(27, 9) \oplus (33, 9)$

TODO

## Exercise 95 🔴

> Consider the extended `BLS6_6` curve as defined in 5.67 and the two curve points $g1 = (13, 15)$ and $g2 = (7v^2, 16v^3)$. Compute the Weil pairing $e(g1, g2)$ using definition 5.49 and Miller's algorithm.

TODO
