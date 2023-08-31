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
> E_{5, 7}(\mathbb{F}_{13}) = \{(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13} \mid y^2 = x^3 + 7x + 5\}
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
sage: E = EllipticCurve(GF(13), [8, 8])
sage: (E(5, 2) - E(9, 4)).xy()
(11, 7)
```

## Exercise 64

> Consider example 79 and compute the set $\{[1](0, 1), [2](0, 1), \ldots, [8](0, 1), [9](0, 1)\}$ using the tangent rule only.

**Isn't this done in example 79 already?**

## Exercise 65

> Consider example 80 and compute the scalar multiplications $[10](5, 11)$ as well as $[10](9, 4)$ and $[4](9, 4)$ with pen and paper using the algorithm from exercise 38 (Efficient Scalar Multiplication).

TODO

## Exercise 66

> Consider example 81 and compute the set shown in equation (5.23) by inserting all points from the projective plane $\mathbb{F}_5\mathbb{P}^2$ into the defining projective Short Weierstrass equation.

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
sage: print(proj_points)
[(0, 1, 1), (0, 4, 1), (2, 1, 1), (2, 4, 1), (3, 1, 1), (3, 4, 1), (4, 2, 1), (4, 3, 1)]
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

## Exercise 68

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

## Exercise 70

> Consider example 82 and compute the set in (5.30) by inserting every pair of field elements $(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13}$ into the defining Montgomery equation.

TODO

## Exercise 71

> Consider $E_1(\mathbb{F})$ from example 70 and show that this curve is not a Montgomery curve.

TODO

## Exercise 72

> Show that `secp256k1` is not a Montgomery curve.

One of the conditions to be a Montogomery curve is that order of the scalar field should be divisible by 4.

```py
sage: p = 115792089237316195423570985008687907853269984665640564039457584007908834671663
sage: E = EllipticCurve(GF(p), [0, 7])
sage: E.order() % 4
1
```

As we can see, the order is not divisible by 4, therefore `secp256k1` is not a Montgomery curve.

## Exercise 73 ✨

> Consider the commutative group defined by the Montgomery group law and TinyJubJub with base field $\mathbb{F}_{13}$ in Montgomery form.
>
> - Compute the inverse of $(1, 9), \mathcal{O}, (7, 12), (4, 9)$.
> - Solve the equation $x \oplus (3, 8) = (10, 3)$ for some point in the Montgomery curve.
>
> Finally, choose some point in the curve and test to see if it is a generator. If not, keep trying until you find one. Print out that generator point's lograithmic order.

See the code [here](./montgomery.sage). **Generator part is still TO-DO**

## Exercise 74

> Show that `alt_bn128` is not a Montgomery curve.

One of the conditions to be a Montogomery curve is that order of the scalar field should be divisible by 4.

```py
sage: p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
sage: E = EllipticCurve(GF(p), [0, 3])
sage: E.order() % 4
1
```

As we can see, the order is not divisible by 4, therefore `alt_bn128` is not a Montgomery curve.

## Exercise 75 ✨

> Consider the commutative group defined by the Twisted Edwards group law and TinyJubJub with base field $\mathbb{F}_{13}$ in Twisted Edwards form.
>
> - Compute the inverse of $(1, 11), (0, 1), (3, 0), (5, 8)$.
> - Solve the equation $x \oplus (5, 8) = (1, 11)$ for some point in the Montgomery curve.
>
> Finally, choose some point in the curve and test to see if it is a generator. If not, keep trying until you find one. Print out that generator point's lograithmic order.

See the code [here](./twisted-edwards.sage). **Generator part is still TO-DO**

## Exercise 76

> Consider the short Weierstrass curve $y^2 = x^3 + x + 1$ over extension field $\mathbb{F}_{5^2}. Compute $(4t + 3, 2t + 1) \oplus (3t + 3, 2)$, and double-check the result in sage. Then, solve the equation $x \oplus (3t + 3, 3) = (3, 4)$ for some $x$ in the curve. Also, compute $[5](2t + 1, 4t + 4)$.

See the code [here](./embedding-and-extension.sage)

## Exercise 77

> Consider TinyJubJub. Show that $t^4 + 2 \in \mathbb{F}_{13}[t]$ is irreducible. Then, write a sage program to implement the finite field extension $\mathbb{F}_{13^4}$. Implement the curve extension in the extension field, and compute the number of curve points (i.e. order).

See the code [here](./embedding-and-extension.sage)

## Exercise 78 ✨

> Consider `alt_bn128` curve. We know from example 89 that this curve has embedding degree 12. Use Sage to find an irreducible polynomial in $\mathbb{F}_p[t]$, and then compute the field extension $\mathbb{F}_{p^{12}}$ to implement the curve extension of `alt_bn128`. Compute the number of curve points.

See the code [here](./embedding-and-extension.sage)

## Exercise 79

> Consider the full 5-torsion group $TJJ_{13}[5]$ from example 92. Write down the set of all elements from this group, and identify the subset of all elements from $TJJ_{13}(\mathbb{F}_{13})[5]$ as well as $TJJ_{13}(\mathbb{F}_{13^2})[5]$. Then compute the 5-torsion group $TJJ_{13}(\mathbb{F}_{13^8})[5]$

TODO

## Exercise 80 ✨

> Consider `secp256k1` curve and it's full $r$-torsion group. Write down a single element from the curve's full torsion group that is not the point at infinity.

TODO

## Exercise 81

> Consider `alt_bn128` curve and and it's full $r$-torsion group. Write a Sage program that computes a generator from the curve's full torsion group.

TODO

## Exercise 83

> Consider `alt_bn128` curve and and it's curve extension. Write a Sage program that computes a generator for each of the torsion group $\mathbb{G}_1[p]$ and $\mathbb{G}_2[p]$.

TODO

## Exercise 84

> Consider the `alt_bn128` curve from example 73, and the generators $g_1$ and $g_2$ of $\mathbb{G}_1[p]$ and $\mathbb{G}_2[p]$ from exercise 83. Write a Sage program that computes the Weil pairing $e(g_1, g_2)$

TODO

## Exercise 85

> Use our definition of the _try_hash_ algorithm to implement a hash function $H_{TJJ_{13}[5]} : \{0, 1\}^* \to TJJ_{13}(\mathbb{F}_{13})[5]$ that maps binary strings of arbitrary length onto the 5-torsion group of $TJJ_{13}(\mathbb{F}_{13})[5]$

TODO

## Exercise 86

> Implement a cryptographic hash function $H_{secp256k1} : \{0, 1\}^* \to secp256k1$ that maps binary strings of arbitrary length onto the elliptic curve `secp256k1`.

TODO

## Exercise 87

> Consider `alt_bn128` curve. Write a Sage program that computes the trace of Frobenius for `alt_bn128`. Does the curve contain more or less elements than its base field $\mathbb{F}_p$?

TODO

## Exercise 88

> Consider `alt_bn128` curve. Write a Sage program that computes the $j$-invariant for `alt_bn128`.
