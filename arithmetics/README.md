# Chapter 3: Arithmetics

Topics in this chapter:

- Integer Arithmetic
- Modular Arithmetic
- Polynomial Arithmetic

## Exercise 1

> What are the absolute values of integers -123, 27 and 0?

123, 27 and 0.

## Exercise 2

> Compute the factorization of 30030 and double check your results with Sage.

We find $2 \times 3 \times 5 \times 11 \times 13 \times 7$, which Sage confirms as:

```py
factor(30030)
# 2 * 3 * 5 * 7 * 11 * 13
```

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

## Exercise 5

> Find an $m \in \mathbb{Z}$ and an $r \in \mathbb{N}$ with $0 \leq r < |b|$ such that $a = mb + r$ holds for the following pairs:
>
> 1. $(a, b) = (27, 5)$
> 2. $(a, b) = (-27, 5)$
> 3. $(a, b) = (127, 0)$
> 4. $(a, b) = (-1687, 11)$
> 5. $(a, b) = (0, 7)$
>
> In which cases are your solutions unique?

1. $m = 5, r = 2$
2. $m = -5, r = 2$
3. No solutions possible because we get $0 \leq r < 0$ due to $b = 0$.
4. $m = -154, r = 7$, which we find by solving for $(1687, 11)$ first.
5. $m = 0, r = 0$.

All solutions are unique.

## Exercise 6 🔴

> Using the programming language of your choice, write an algorithm that computes integer long divison and handles all edge cases properly.

Here is an implementation in TypeScript. It uses simple statements so you can easily port it to other languages.

```ts
/**
 * Long Division
 *
 * Given `dividend` and `divisor`, returns `quot` and `rem` such that
 * `dividend = quot * divisor + rem` where `rem < divisor`.
 *
 * Throws an error if `divisor = 0`.
 */
function longDivision(
  dividend: number,
  divisor: number
): { quot: number; rem: number } {
  if (divisor === 0) {
    // edge case: divisor is 0
    throw new Error("division by zero");
  } else if (divisor === 1) {
    // edge case: divisor is 1
    return { quot: dividend, rem: 0 };
  }

  // digits of the dividend
  let digits = dividend
    .toString()
    .split("")
    .map((d) => parseInt(d));

  // the quotient
  let result = 0;

  // the value to be divided at each iteration
  let carry = 0;

  for (const digit of digits) {
    // append the new digit to the carry
    carry *= 10;
    carry += digit;

    // repeatedly subtract the divisor from carry until carry is less than divisor
    // keep track of the number of subtractions
    let count = 0;
    while (carry >= divisor) {
      carry -= divisor;
      count++;
    }

    // append the number of subtractions to the result (quotient)
    result *= 10;
    result += count;
  }

  return { quot: result, rem: dividend - result * divisor };
}
```

You can test out the implementation by comparing the results for random numbers, shown below:

```ts
for (let i = 0; i < 100_000; i++) {
  const dividend = Math.floor(Math.random() * 999_999_999);
  const divisor = Math.floor(Math.random() * 999_999);

  const result = longDivision(dividend, divisor);
  const expected = {
    quot: Math.floor(dividend / divisor),
    rem: dividend % divisor,
  };

  if (result.quot !== expected.quot || result.rem !== expected.rem) {
    throw new Error("results do not match");
  }
}
```

## Exercise 7

> Write an algorithm that computes the binary representation of any non-negative integer.

There are many many implementations out there, but here is one for Circom:

```cpp
template Num2Bits(n) {
  assert(n < 254); // due to BN128
  signal input in;
  signal output out[n];

  var lc = 0;
  var bit_value = 1;

  for (var i = 0; i < n; i++) {
    out[i] <-- (in >> i) & 1;
    out[i] * (out[i] - 1) === 0;

    lc += out[i] * bit_value;
    bit_value <<= 1;
  }

  lc === in;
}
```

## Exercise 8

> Find integers $s, t \in \mathbb{Z}$ such that $\gcd(a, b) = sa + tb$ holds for the following pairs:
>
> - $(a, b) = (45, 10)$
> - $(a, b) = (13, 11)$
> - $(a, b) = (13, 12)$

- $5 = (1) \times 45 + (-4) \times 10$
- $1 = (-5) \times 13 + (6) \times 11$
- $1 = (1) \times 13 + (-1) \times 12$

## Exercise 9

> What is $gcd(n,p)$ for some natural number $n$ and a prime $p$ such that $n < p$?

Well, it's 1. In particular, a prime number is defined to be only divisible by itself, or 1. Since the `gcd` must be one of these, it is definitely not $p$ since $n < p$, so it must be $1$.

```py
p = random_prime(2 ^ 256)
n = ZZ.random_element(p)
print(gcd(p, n))
```

## Exercise 10

> Find all numbers $k \in \mathbb{N}$ with $0 \leq k \leq 100$ such that $\gcd(100, k) = 5$.

We begin by reducing the GCD:

$$
\gcd(5 \times 20, 5 \times k') = 5 \implies \gcd(20, k') = 1
$$

From this, we simply list out all the primes less than 20:

$$
k' \in \{19, 17, 13, 11, 7, 3\}
$$

Our solutions for $k$ is therefore these numbers multiplied by 5:

$$
k \in \{95, 85, 65, 55, 35, 15\}
$$

## Exercise 11

> Show that $\gcd(n, m) = \gcd(n + m, m)$ for all $n, m \in \mathbb{N}$.

- Let $g$ denote $\gcd(n, m)$.
- Then, $g \mid n$ and $g \mid m$. So, $n = g n'$ and $m = g m'$.
- Now we ask $\gcd(g n' + m, g m') =\gcd(g n' + g m', g m') = \gcd(g(n' + m'), g m')$.
- This is equal to $g$, which is $\gcd(n, m)$.

## Exercise 12

> Consider exercise 8 again, which pairs $(a, b)$ from that exercise are coprime?

Pairs $(13, 11)$ and $(13, 12)$ are coprime.

## Exercise 13

> Write the octal numbers $0o1354$ and $0o777$ into their decimal representations.

- $0o1354 \to 4\times8^0 + 5\times8^1 + 3 \times 8^2 + 1\times8^3 = 748$
- $0o777$ is actually $0o1000 - 0o1$ so we can find $8^3 - 1 = 511$

## Exercise 14

> Which of the following pairs of numbers are congruent with respect to the modulus 13?
>
> - $(5, 19)$
> - $(13, 0)$
> - $(-4, 9)$
> - $(0, 0)$

- $5 \not\equiv 13 + 6 \pmod{13}$
- $13 + 0 \equiv 0 \pmod{13}$
- $13 + (-4) = 13 + (9 - 13) \equiv 9 \pmod{13}$
- $0 \equiv 0 \pmod{13}$

## Exercise 15

> Find all integers $x$ such that the congruence $x \equiv 4 \pmod{6}$ is satisfied.

All numbers of the form $6k + 4$ for integer $k$ satisfy this congruence.

## Exercise 16

> Find all solutions $x \in \mathbb{Z}$ to the following congruence:
>
> $$5x + 4 \equiv 28 + 2x \pmod{13}$$

By reducing we find $3x \equiv 24 \pmod{13}$. We need to get rid of the 3 in $3x$, so we must multiply $3x$ by some $k$ where $3k = 1$.

Fermat's Little Theorem tells us that $3^{13 - 2} \pmod{13}$ would give us that $k$, which we find to be 9. Multiplying both sides with 9, we get $x \equiv 216 \equiv 8 \pmod{13}$.

## Exercise 17

> Find all solutions $x \in \mathbb{Z}$ to the following congruence:
>
> $$69x \equiv 5 \pmod{23}$$

Due to mod 23, we get $0x \equiv 5 \pmod{23}$, so there are no solutions.

## Exercise 18

> Find all solutions $x \in \mathbb{Z}$ to the following congruence:
>
> $$69x \equiv 46 \pmod{23}$$

Due to mod 23, we get $0x \equiv 0 \pmod{23}$, so all $x$ are solutions.

## Exercise 19

> Show that for $a \equiv b \pmod{n}$ the equivalence $a^k \equiv b^k \pmod{n}$ holds.

It comes from "compatibility with multiplication" which is the rule that

$$
a_1 \equiv b_1 \pmod{n} \text{ and } a_2 \equiv b_2 \pmod{n} \implies a_1a_2 \equiv b_1b_2 \pmod{n}
$$

implying one by one the equivalences $a^i \equiv b^i \pmod{n}$ for $i = 2, 3, \ldots, k$.

You can also prove this by induction to show $a^1 \equiv b^1 \pmod{n}$ and assume $a^{k-1} \equiv b^{k-1} \pmod{n}$.

## Exercise 20 ✨

> Let $a, n$ be integers, such that $a$ and $n$ are **not** co-prime. For which $b \in \mathbb{Z}$ does $ax \equiv b \pmod{n}$ have a solution $x$, and how does the solution set look like?

The equivalence implies $ax - ny = b$. Denote $d=\gcd(a, n)$ which also implies $a = da'$ and $n = dn'$. Now, supposing that the solution exists for $ax - ny = b$ for some $x', y'$:

$$b = ax' - ny' = da'x' - dn'y' = d(a'x' - n'y')$$

Since $a', x', n', y'$ are all integers, $b / d$ must be an integer too; implying that $d \mid b$. In other words, solutions exists for $b \in \mathbb{Z}$ where $\gcd(a, n) \mid b$.

Now that we also know $b = db'$, let's rewrite the congruence:

$$da'x \equiv db' \pmod{dn'}$$

We can remove $d$ from all sides:

$$a'x \equiv b' \pmod{n'}$$

Since $\gcd(a', n') = 1$, this has only one solution. Denote this solution as $t$ where $x \equiv t \pmod{n'}$. The other solutions are the numbers from $t$ to $t + n$, with increments of $n'$. Notice that:

- $t \implies da'(t) \equiv db' \pmod{dn'}$
- $t + n' \implies da'(t + n') = da't + a'dn' = da' \equiv db' \pmod{dn'}$
- $t + 2n' \implies da'(t + 2n') = da't + 2a'dn' = da' \equiv db' \pmod{dn'}$
- $\ldots$
- $t + dn' \implies da'(t + dn') = da't + da'dn' = da' \equiv db' \pmod{dn'}$

Since $t + (d+1)n' = t + n' + dn'$ would be equivalent to $t + n'$ in modulo $n'$, we start to wrap around the solution set of $x$. That is our complete set of solutions with $d$ elements.

See also: [Theory of Numbers - Theorem 57 (p. 68)](https://blngcc.files.wordpress.com/2008/11/hardy-wright-theory_of_numbers.pdf)

## Exercise 21

> Define $\mathbb{Z}_{13}$ as the arithmetic modulo 13. Consider the following equation:
>
> $$5x + 4 \equiv 28x +2x \pmod{13}$$
>
> Rewrite this in $\mathbb{Z}_{13}$.

We have:

$$5x + 4 \equiv 2 + 2x \pmod{13}$$
$$3x + 2 \equiv 0 \pmod{13}$$
$$3x \equiv 11 \pmod{13}$$

Now, which integers $x \in \mathbb{Z}_{13}$ give $11 \pmod{13}$ when multiplied by 3? Instead of asking that, here I can ask what is the inverse of 3 in modulo 13, that is, which number when multiplied by 3 gives 1?

We know that $3 \times 9 = 27 \equiv 1 \pmod{13}$, so I can multiply both sides with 9.

$$x \equiv 99 \equiv 8 \pmod{13}$$

Meaning that all integers of the form $13k+8, k \in \mathbb{N}$ solve this equation.

## Exercise 22

> Which of the integers 7, 1, 0, 805, -4255 have multiplicative inverses in modular 24 arithmetic?

First let's get rid of the trivial cases:

- 1 is always it's own inverse.
- 0 never has a multiplicative inverse.

We can check GCD for the remaining numbers:

```py
gcd(7, 24), gcd(805, 24), gcd(-4255, 24)
# (1, 1, 1)
```

Apparently, all of these are coprime to 24. We can perhaps use Extended Euclidean Algorithm, which is implemented in Sage already. Using `xgcd(a, b)` where `a >= b` we can find:

$$
\gcd(a, b) = s \times a + t \times b
$$

First, let's treat all numbers in mod 24, so $805 \equiv 13 \pmod{24}$ and $-4255 \equiv 17 \pmod{24}$. Then, let's use `xgcd` as follows:

```py
print(xgcd(24, 7))
# (1, -2, 7)

print(xgcd(24, 13))
# (1, 6, -11)

print(xgcd(24, 17))
# (1, 5, -7)
```

To interpret these results:

- $7^{-1} \equiv 7 \pmod{24}$
- $13^{-1} \equiv -11 \equiv 13 \pmod{24}$
- $17^{-1} \equiv -7 \equiv 17 \pmod{24}$

These numbers all are their own inverses, nice!

## Exercise 23

> Find the set of all solutions to the congruence $17(2x + 5) - 4 \equiv 2x + 4 \pmod{5}$. Then, project the congruence into $\mathbb{Z}_5$ and solve the resulting equation in $\mathbb{Z}_5$.

Due to mod 5, we find:

$$
2(2x) + 1 \equiv 2x + 4 \pmod{5}
$$

$$
2x \equiv 3 \pmod{5}
$$

We have $2^{-1} \equiv 2^{5-2} \pmod{5}$ which is 3. Multipyling both sides with 3 we find:

$$
x \equiv 4 \pmod{5}
$$

## Exercise 24

> Find the set of all solutions to the congruence $17(2x + 5) - 4 \equiv 2x + 4 \pmod{6}$. Then, project the congruence into $\mathbb{Z}_6$ and _try to solve_ the resulting equation in $\mathbb{Z}_6$.

Due to mod 6 we find:

$$
5(2x + 5) + 2 \equiv 2x + 4 \pmod{6}
$$

$$
8x + 23 \equiv 0 \pmod{6}
$$

$$
2x \equiv 1 \pmod{6}
$$

This has no solutions. No way to think about it is that for something to be 1 in mod 6, it has to be an odd number. The left-hand side of this congruence will always be an even number.

## Exercise 25

> Compare both expansions of $P_7$ from $\mathbb{Z}[x]$ in example 18 and from $\mathbb{Z}_6[x]$ in example 19. Can you see how definition of $P_7$ over $\mathbb{Z}$ projects to the definition over $\mathbb{Z}_6$, if you consider the residue classes of $\mathbb{Z}_6$?

Our polynomial is $P_7(x) = (x-2)(x+3)(x-5)$, which is projected to be:

$$
P_7 \in \mathbb{Z}[x] = x^3 - 4x^2 - 11x + 30
$$

$$
P_7 \in \mathbb{Z}_6[x] = x^3 + 2x^2 + x
$$

We can compute the result in $\mathbb{Z}$ and then project the coefficients, that is equal to doing everything in $\mathbb{Z}_6$ in the first place!

## Exercise 26

> Compare the sum $P + Q$ and the product $P \cdot Q$ from the previous two examples 22 and 23. How can we derive the computations in $\mathbb{Z}_6[x]$ from the computations in $\mathbb{Z}[x]$?

We can find the answers in $\mathbb{Z}[x]$, and then project the coefficients from $\mathbb{Z}$ to $\mathbb{Z}_6$, giving us the answer in $\mathbb{Z}_6[x]$.

## Exercise 27

> Consider polynomials $A(x) := -3x^4 + 4x^3 + 2x^2 + 4$ and $B(x) = x^2 - 4x + 2$. Compute the Euclidean Division of A by B in the following types:
>
> - $A, B \in \mathbb{Z}[x]$
> - $A, B \in \mathbb{Z}_6[x]$
> - $A, B \in \mathbb{Z}_5[x]$

We can calculate the result with coefficients in $\mathbb{Z}$ and then project it to the others. I used Sage instead of doing by hand:

```py
ZZx = ZZ['x']
A = ZZx(-3*x^4 + 4*x^3 + 2*x^2 + 4)
B = ZZx(x^2 - 4*x + 2)

# quotient
A // B
# -3*x^2 - 8*x - 24

# remainder
A % B
# -80*x + 52
```

The result of division is the quotient $Q$ and remainder $R$ polynomials:

- $Q(x) = -3x^2 -8x - 24$
- $R(x) = -80x + 52$

Projecting these to $\mathbb{Z}_6[x]$ we get:

- $Q(x) = 3x^2 + 4x$
- $R(x) = 4x + 4$

Projecting these to $\mathbb{Z}_5[x]$ we get:

- $Q(x) = 2x^2 + 2x + 1$
- $R(x) = 2$

## Exercise 28

> Show that the polynomial $B(x) = 2x^4 -3x +4 \in \mathbb{Z}_5[x]$ is a factor of the polynomial $A(x) = x^7 + 4x^6 + 4x^5 + x^3 + 2x^2 + 2x + 3 \in \mathbb{Z}_5[x]$.

The important thing here is to realize that you need to invert the leading coefficient. In the case of $B(x)$, that is 2. We need to find the inverse of 2, which we can via Fermat's Little Theorem as $2^{5-2} \bmod 5 = 3$. By doing the calculations by hand, I've found:

- $Q(x) = 3x^3 + 2x^2 + 2x + 2$

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

## Exercise 29 ✨

> Show that if a polynomial $P \in \mathbb{R}[X]$ of degree $\deg(P) = m$ has less than $m$ roots, it must have a prime factor $F$ such that $\deg(F) > 1$.

Suppose that $P$ has $k$ roots $\{z_1, z_2, \ldots, z_k\}$ such that $k < m$. Then, we know that:

$$
P(x) = (x - z_1)(x - z_2)\ldots(x - z_k)Q(x)
$$

From this, we can see that $P(x) = (x^k + \ldots)Q(x)$, meaning that $Q(x)$ must have degree $m - k$. $Q(x)$ itself may be reducible; however, none of it's factors may be linear because that would imply the existence of another root. So, $Q(x)$ must have at least one prime factor that is not linear (i.e. degree greater than 1), which implies $P(x)$ such a factor.

## Exercise 30

> Consider $P = x^7 + 3x^6 + 3x^5 + x^4 - x^3 - 3x^2 - 3x - 1$ in $\mathbb{Z}_6[X]$. Find the set of all roots $R_0(P)$ and then compute the prime factorization of $P$.

Let's use Sage to find the roots.

```py
Z6 = Integers(6)
Z6x = Z6['x']
P = Z6x(x^7 + 3*x^6 + 3*x^5 + x^4 - x^3 - 3*x^2 - 3*x - 1)

# find roots
roots = P.roots(multiplicities=False)
print("Roots:", roots)
# [1, 5]
```

To find factors, we can divide $x - r$ for each root $r$ until the quotient does not have $r$ as a root anymore. Once we are done with all roots, if the remaining result is not $1$ we can also include that as a factor.

```py
factors = []
Q = P
for r in roots:
  R = Z6x([-r, 1])
  count = 1
  while True:
    Q = Q // R
    if r not in Q.roots(multiplicities=False):
      break
    count += 1
  factors.append((R, count))

if Q != Z6x(1):
  factors.append((Q, 1))

print("Factors:", factors)
# [(x + 5, 1), (x + 1, 4), (x^2 + 1, 1)]
```

Note that the same factor can appear a few times, so the `factors` here is a list of tuples where the first item of a tuple is the factor, and the second item is the number of time it appears.

After finding the factors, we can test to see if we get back the original polynomial when we multiply all these factors.

```py
PP = Z6x(1)
for f in factors:
  assert(f[1] > 0)
  for _ in range(f[1]):
    PP = PP * f[0]

assert(P == PP)
```

## Exercise 31

> Consider modular 5 arithmetic, and set $S = \{(0, 0), (1, 1), (2, 2), (3, 2)\}$. Find a polynomial $P \in \mathbb{Z}_5[x]$ such that $P(x_i) = y_i$ for all $(x_i, y_i) \in S$.

We need to do lagrange interpolation for this. We could do by hand, but let's just use Sage for this one!

```py
Integers(5)[x].lagrange_polynomial([(0, 0), (1, 1), (2, 2), (3, 2)])
# 4*x^3 + 3*x^2 + 4*x
```

Apparently, $4x^3 + 3x^2 + 4x$ does the job.

## Exercise 32

> Consider the same set $S = \{(0, 0), (1, 1), (2, 2), (3, 2)\}$. Why is it not possible to do Lagrange interpolation for these points in $\mathbb{Z}_6[x]$?

We can't, because we need the elements to have multiplicative inverses of the evaluated elements. In $\mathbb{Z}_6$, only 1 and 5 have multiplicative inverse; the other elements do not have a multiplicative inverse.
