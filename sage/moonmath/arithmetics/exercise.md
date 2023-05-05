# Selected Exercises

## Exercise 3

> For $4x + 21 = 5$, compute the set of all solutions for $x$ under the alternative assumptions:
>
> - equation is defined over set of natural numbers
> - equation is defined over set of integers

For natural numbers, we have no solutions. However, for integers, we have $x = -4 \in \mathbb{Z}$.

## Exercise 4

> For $2x^3 - x^2 - 2x = -1$, compute the set of all solutions for $x$ under the alternative assumptions:
>
> - equation is defined over set of natural numbers
> - equation is defined over set of integers
> - equation is defined over set of rational numbers

With a quick manipulation, we can see that $(2x)(x^2 - 1) = x^2 - 1$. With this form $x=1$ and $x=-1$ is a solution.

We can also find an $x$ such that $|x| \ne 1$ by dividing both sides by $x^2 - 1$, and obtain $2x=1$ to find $x=1/2$. The sets of solutions are thus:

- $\{1\} \in \mathbb{N}$
- $\{1, -1\} \in \mathbb{Z}$
- $\{1, -1, 1/2\} \in \mathbb{Q}$

## Exercise 9

> What is $gcd(n,p)$ for some natural number $n$ and a prime $p$ such that $n < p$?

Well, it's 1.

```py
p = random_prime(2 ^ 256)
n = ZZ.random_element(p)
print(gcd(p, n))`
```

## Exercise 11

> Show that $\gcd(n, m) = \gcd(n + m, m)$ for all $n, m \in \mathbb{N}$.

- Let $g$ denote $\gcd(n, m)$.
- Then, $g \mid n$ and $g \mid m$. So, $n = g n'$ and $m = g m'$.
- Now we ask $\gcd(g n' + m, g m') =\gcd(g n' + g m', g m') = \gcd(g(n' + m'), g m')$.
- This is equal to $g$, which is $\gcd(n, m)$.

## Exercise 19

> Show that for $a \equiv b \pmod{n}$ the equivalence $a^k \equiv b^k \pmod{n}$ holds.

It comes from "compatibility with multiplication" which is the rule that

$$
a_1 \equiv b_1 \pmod{n} \text{ and } a_2 \equiv b_2 \pmod{n} \implies a_1a_2 \equiv b_1b_2 \pmod{n}
$$

implying one by one the equivalences $a^i \equiv b^i \pmod{n}$ for $i = 2, 3, \ldots, k$.

## ⚠️ Exercise 20 ⚠️

> Let $a, n$ be integers, such that $a$ and $n$ are **not** co-prime. For which $b \in \mathbb{Z}$ does $ax \equiv b \pmod{n}$ have a solution $x$, and how does the solution set look like?

The equivalence implies $ax - ny = b$. Denote $d=\gcd(a, n)$ which also implies $a = da'$ and $n = dn'$. Now, supposing that the solution exists for $ax - ny = b$ for some $x', y'$:

$$b = ax' - ny' = da'x' - dn'y' = d(a'x' - n'y')$$

Since $a', x', n', y'$ are all integers, $b / d$ must be an integer too; implying that $d \mid b$.

- [Theory of Numbers - Theorem 57 (p. 68)](https://blngcc.files.wordpress.com/2008/11/hardy-wright-theory_of_numbers.pdf)

## Exercise 21

> Define $\mathbb{Z}_{13}$ as the arithmetic modulo 13. Consider the following equation:
>
> $$5x + 4 \equiv 28x +2x \pmod{13}$$
>
> Rewrite this in $\mathbb{Z}\_{13}

We have:

$$5x + 4 \equiv 2 + 2x \pmod{13}$$
$$3x + 2 \equiv 0 \pmod{13}$$
$$3x \equiv 11 \pmod{13}$$

Now, which integers $x \in \mathbb{Z}_{13}$ give $11 \pmod{13}$ when multiplied by 3? Instead of asking that, here I can ask what is the inverse of 3 in modulo 13, that is, which number when multiplied by 3 gives 1?

We know that $3 \times 9 = 27 \equiv 1 \pmod{13}$, so I can multiply both sides with 9.

$$x \equiv 99 \equiv 8 \pmod{13}$$

Meaning that all integers of the form $13k+8, k \in \mathbb{N}$ solve this equation.

## Exercise 28

> Show that the polynomial $B(x) = 2x^4 -3x +4 \in \mathbb{Z}_5[x]$ is a factor of the polynomial $A(x) = x^7 + 4x^6 + 4x^5 + x^3 + 2x^2 + 2x + 3 \in \mathbb{Z}_5[x]$.

The important thing here is to realize that you need to invert the leading coefficient. In the case of $B(x)$, that is 2. We need to find the inverse of 2, which we can via Fermat's Little Theorem as $2^{5-2} \bmod 5 = 3$. By doing the calculations by hand, I've found:

- $Q(x) = 3x^2 = 2x^2 + 2x + 2$

Let's verify from Sage:

```py
# Univariate Polynomial Ring in x
# over Ring of integers modulo 5
Z5x = Integers(5)[x]

A = Z5x(x^7 + 4*x^6 + 4*x^5 + x^3 + 2*x^2 + 2*x + 3)
B = Z5x(2*x^4 - 3*x + 4)

# expected result
Q = Z5x(3*x^3 + 2*x^2 + 2*x + 2)
Q == A / B # true
```

## ⚠️ Exercise 29 ⚠️

> Show that if a polynomial $P \in \mathbb{R}[X]$ of degree $\deg(P) = m$ has less than $m$ roots, it must have a prime factor $F$ such that $\deg(F) > 1$.

Let $z_1, z_2, \ldots, z_k$ be the distinct roots of $P$, where $1 \leq k < m$. Then, $P(x) = c(x - z_1)(x - z_2) \cdots (x - z_k) Q(x)$, where $c$ is the leading coefficient of $P$ and $Q(x)$ is a polynomial of degree $m-k$.

Since $P$ is not constant, we have $k < m$. Thus, $Q(x)$ is a non-constant polynomial of degree at least $1$

Suppose for the sake of contradiction that all the prime factors of $P$ have degree at most $1$. Then, we can write some prime factor $F(x) = ax + b$ for some $a,b \in \mathbb{R}$.

By construction, $Q(x)$ is not identically zero and has degree $m-k \geq 1$, so $a \neq 0$. Moreover, $Q(x)$ has no roots in common with the polynomial $(x - z_1)(x - z_2) \cdots (x - z_k)$, since $Q(x)$ has no roots and $z_1, z_2, \ldots, z_k$ are all distinct roots of $P$. Therefore, the product $(x - z_1)(x - z_2) \cdots (x - z_k) Q(x)$ has $m$ distinct roots, which contradicts the fact that $P$ has less than $m$ roots.

Thus, our assumption that all prime factors of $P$ have degree at most $1$ leads to a contradiction. Therefore, $P$ must have a prime factor $F$ such that $\deg(F) > 1$, as required.

## Exercise 31

> Consider modular 5 arithmetic, and set $S = \{(0, 0), (1, 1), (2, 2), (3, 2)\}$. Find a polynomial $P \in \mathbb{Z}_5[x]$ such that $P(x_i) = y_i$ for all $(x_i, y_i) \in S$.

We need to do lagrange interpolation for this. We could do by hand, but let's just use Sage for this one!

```py
Integers(5)[x].lagrange_polynomial([(0, 0), (1, 1), (2, 2), (3, 2)])
# 4*x^3 + 3*x^2 + 4*x
```

Apparently, $4x^3 + 3x^2 + 4x$ does the job!

## Exercise 32

> Consider the same set $S = \{(0, 0), (1, 1), (2, 2), (3, 2)\}$. Why is it not possible to do Lagrange interpolation for these points in $\mathbb{Z}_6[x]$?

We can't, because we need the elements to have multiplicative inverses of the evaluated elements. In $\mathbb{Z}_6$, only 1 and 5 have multiplicative inverse; the other elements do not have a multiplicative inverse.
