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

## Exercise 20

> Let $a, n$ be integers, such that $a$ and $n$ are **not** co-prime. For which $b \in \mathbb{Z}$ does $ax \equiv b \pmod{n}$ have a solution $x$, and how does the solution set look like?

... todo ...

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