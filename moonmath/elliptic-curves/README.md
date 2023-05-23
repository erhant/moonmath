# Selected Exercises

Also see this <https://curves.xargs.org/> for great animations, especially about Chord & Tangent rule. Furthermore, I have drawn the example TinyJubJub curves in WolframAlpha:

- [Short Weierstrass](https://www.wolframalpha.com/input?i2d=true&i=Power%5By%2C2%5D%3DPower%5Bx%2C3%5D%2B+8x+%2B8+for+x%5C%2844%29y+in+GF%5C%2840%2913%5C%2841%29)
- [Montgomery](https://www.wolframalpha.com/input?i2d=true&i=7Power%5By%2C2%5D%3DPower%5Bx%2C3%5D%2B6Power%5Bx%2C2%5D%2Bx+for+x%5C%2844%29+y+in+GF%5C%2840%2913%5C%2841%29)
- [Twisted Edwards](https://www.wolframalpha.com/input?i2d=true&i=3Power%5Bx%2C2+%5D%2B+Power%5By%2C2%5D+%3D+1+%2B+8+Power%5Bx%2C2%5D+Power%5By%2C2%5D+for+x%5C%2844%29+y+in+GF%5C%2840%2913%5C%2841%29)

## Exercise 60

> Look up the definition of curve BLS12-381, implement it in Sage, and compute the number of all curve points.

See the code [here](./short-weierstrass.sage).

## Exercise 63

> Consider the commutative group defined by the affine group law and TinyJubJub with base field $\mathbb{F}_{13}$.
>
> 1. Compute the inverse of $(10, 10), \mathcal{O}, (4, 0), (1, 2)$
> 2. Solve the equation $x \oplus (9, 4) = (5, 2)$ for some point $x$ on the curve.

See the code [here](./short-weierstrass.sage).

## Exercise 67 ✨

> Compute the projective representation of the TinyJubJub curve with base field $\mathbb{F}_{13}$. Then, print the logarithmic order of its large prime-order subgroup with respect to the generator $[ 7 : 11 : 1 ]$.

See the code [here](./short-weierstrass.sage).

## Exercise 72

> Show that `secp256k1` is not a Montgomery curve.

See the code [here](./montgomery.sage).

## Exercise 73 ✨

> Consider the commutative group defined by the Montgomery group law and TinyJubJub with base field $\mathbb{F}_{13} in Montgomery form$.
>
> - Compute the inverse of $(1, 9), \mathcal{O}, (7, 12), (4, 9)$.
> - Solve the equation $x \oplus (3, 8) = (10, 3)$ for some point in the Montgomery curve.
>
> Finally, choose some point in the curve and test to see if it is a generator. If not, keep trying until you find one. Print out that generator point's lograithmic order.

See the code [here](./montgomery.sage). **Generator part is still TO-DO**

## Exercise 74

> Show that `alt_bn128` is not a Montgomery curve.

See the code [here](./montgomery.sage).

## Exercise 75 ✨

> Consider the commutative group defined by the Twisted Edwards group law and TinyJubJub with base field $\mathbb{F}_{13} in Twisted Edwards form$.
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

## Exercise 80 ✨

> Consider `secp256k1` curve and it's full $r$-torsion group. Write down a single element frmo the curve's full torsion group that is not the point at infinity.

todo

## Exercise 81

> Consider `alt_bn128` curve and and it's full $r$-torsion group. Write a Sage program that computes a generator from the curve's full torsion group.

todo

## Exercise 83

> Consider `alt_bn128` curve and and it's curve extension. Write a Sage program that computes a generator for each of the torsion group $\mathbb{G}_1[p]$ and $\mathbb{G}_2[p]$.

todo

## Exercise 84

todo

## Exercise 85

todo

## Exercise 86

todo
