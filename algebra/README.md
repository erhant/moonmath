# Chapter 4: Algebra

## Exercise 33

> Let $\mathbb{Z}_5^*$ be the set of all remainder classes from $\mathbb{Z}_5$ without the class 0, meaning that $\mathbb{Z}_5^* = \mathbb{Z}_5 \setminus \{0\}$. Show that $(\mathbb{Z}_5, \cdot)$ is a commutative group (aka. Abelian Group).

The map $\cdot$ (multiplication) of all elements in $\mathbb{Z}_5$ have the following properties:

- Closed due to multiplication
- Associative due to multiplication
- Commutative due to multiplication
- Identity is $1 \in \mathbb{Z}_5$
- Inverse exists for every element, as all elements are co-prime to $5$

With all these properties, the group is commutative. We can also see this from the table at example 16.

## Exercise 34

> Generalizing the previous exercise, consider $n$ and let $\mathbb{Z}_n^*$ be the set of all remainder classes from $\mathbb{Z}_n$ without the class 0, i.e. $\mathbb{Z}_n^* = \{1, 2, \ldots, n - 1\}$. Provide a counter-example to show that $(\mathbb{Z}_n^*, \cdot)$ is not a group in general.
>
> Find a condition such that $(\mathbb{Z}_n^*, \cdot)$ is a commutative group, compute the neutral element, give a closed form for the inverse of any element and prove the commutative group axioms.

TODO

## Exercise 35

> Let $n \in \mathbb{N}$ with $n \geq 2$ be some moodulus. What is the order of the remainder class group $(\mathbb{Z}_n, +)$?

Since there are $n$ different remainders possible for some integer division by $n$, the remainder class group has order $n$.

## Exercise 36

> Consider the group $(\mathbb{Z}_6, +)$ and show that 5 is a generator, while 2 is not a generator.

If 5 is a generator, then multiplications (i.e. multiple additions) of it should give the group members. Let us say that $(6 - 1)k$ results in group members for $k = 1, 2, ..., 6$. Indeed we have $-1, -2, ..., -6$ for the results, which are $5, 4, ..., 0$ in mod 6, giving out all elements in the group.

```py
sage: list(map(lambda k : (6-1)*k % 6, range(1, 7)))
[5, 4, 3, 2, 1, 0]
```

When we consider the same logic for 2, we think of $(6 - 4)k$, but notice that for $k = 3$ we get $18 - 12 \equiv 0 \pmod{6}$. This means that $k=4$ will result in same with $k=1$, so we can't generate all group members.

```py
sage: list(map(lambda k : (6-4)*k % 6, range(1, 7)))
[2, 4, 0, 2, 4, 0]
```

## Exercise 37 ✨

> Let $p \in \mathbb{P}$ be prime number and show that $(\mathbb{Z}_p^*, \cdot)$ is cyclic, i.e. it has a not necessarily unique single element that is able to generate the entire group.

I couldn't prove this myself, here is a [proof from YouTube](https://www.youtube.com/watch?v=EiM7U-WT40Q) that I saw in ZKHAC MoonMath study group. There is also this [question on StackExchange](https://math.stackexchange.com/questions/290427/is-mathbb-z-p-1-2-3-p-1-a-cyclic-group).

We must first recall **Euler's Phi (or Totient) Function**: $\phi(n)$ is the number of integers in $\{1,2,3, \ldots ,n\}$ which are relatively prime to $n$. You can use `euler_phi` in Sage to compute the Euler Totient Function.

Now, define a function $N_p(d)$ that counts the number of generators that have order $d$ in $\mathbb{Z}_p^*$. For example, consider elements in $\mathbb{Z}_5^* = \{1, 2, 3, 4\}$ which for this function result in:

- $N_5(1) = 1$ due to element $\{1\}$
- $N_5(2) = 1$ due to element $\{4\}$
- $N_5(3) = 0$
- $N_5(4) = 2$ due to elements $\{2, 3\}$, which are generators because 4 is the order of the group!

We will make use of the following three facts, which the YouTube lecture apparently covered before:

$$
\sum_{d \mid (p-1)}N_p(d) = p - 1
$$

We can see this in our example where $4 = 1 + 1 + 2$. Also notice that we are choosing $d$ that divides the order of our prime group $p-1$. Our next fact is:

$$
N_p(d) \leq \phi(d)
$$

This should make sense, because $\phi(d)$ can be at most $p-1$ for prime $p = d$, and $N_p(d)$ can be at most $p-1$ where each element in the group is a generator. Finally,

$$
\sum_{d \mid n}\phi(d) = n
$$

The last fact is not really obvious to me, but here is some [discussion and proofs](https://math.stackexchange.com/questions/737422/euler-totient-function-sum-of-divisors-theorem-2-2-apostol) about it.

Now, due to the first fact we have:

$$
p - 1 = \sum_{d \mid (p-1)}N_p(d)
$$

Due to the second fact (applied to every term in the sum) we have:

$$
p - 1 = \sum_{d \mid (p-1)}N_p(d) \leq \sum_{d \mid (p-1)}\phi(d)
$$

Finally, due to the third fact we have

$$
p - 1 = \sum_{d \mid (p-1)}N_p(d) \leq \sum_{d \mid (p-1)}\phi(d) = p - 1
$$

Notice how $p-1$ sandwiches this inequality from both sides, meaning that the two sums must be equal. This means that each element in the left sum must match the one on the right sum. In particular:

$$
N_p(p-1) = \phi(p-1)
$$

Euler's Totient Function is a positive function, meaning that it can never be equal to 0. So, $N_p(p-1)$ must also be positive, meaning that for any prime $p$ there is always at least 1 generator that can generate the group.

In fact, here we actually showed that there are $\phi(p-1)$ many generators that can generate the group for prime $p$. Indeed looking at our example of $\mathbb{Z}_5^*$ with order $4$, there were 2 generators $\{2, 3\}$ and $\phi(4) = 2$.

## Exercise 38

> Let $(\mathbb{G}, +)$ be a finite cyclic group of order $n$. Consider "cyclic group exponentiation" and define its analog for groups in additive notation.

Cyclic group exponentiation made use of "square-and-multiply". In efficient scalar multiplication, we will essentially use it's equivalent operations in additive groups, that is, "double-and-add". See the code [here](./cyclic-group-exponentiation.sage).

```py
def cge(g: int, x: int, n: int) -> int:
  h = g
  y = 1 # natural element g^0
  while x > 0:
    if x & 1 == 1:
      y = (y * h) % n
    h = (h * h) % n
    x >>= 1
  return y
```

## Exercise 39

> Consider example 40, and show that $\mathbb{Z}_5^*[2]$ is a commutative group.

TODO

## Exercise 40

> Consider the finite cyclic group $(\mathbb{Z}_6, +)$. Describe all subgroups of this group. Then, identify its large prime order subgroup, define its cofactor clearing map and apply that map to all elements of $\mathbb{Z}_6$.

TODO

## Exercise 41 ✨

> Let $(\mathbb{Z}_p^*, \cdot)$ be a cyclic group. Show that for $p \geq 5$ not every element of $\mathbb{F}_p^*$ is a generator of $\mathbb{F}_p^*$.

TODO

## Exercise 42 ✨

> **Arithmetic Laws for Pairing Maps:** Let $\mathbb{G}_1, \mathbb{G}_2$ and $\mathbb{G}_3$ be finite cyclic groups of the same order $n$, and let $e(\cdot, \cdot) = \mathbb{G}_1 \times \mathbb{G}_2 \to \mathbb{G}_3$ be a pairing map.
>
> Show that for a given $g_1 \in \mathbb{G}_1, g_2 \in \mathbb{G}_2$ and all $a, b \in \mathbb{Z}_n$ the following identity holds:
>
> $$
> e(g_1^a, g_2^b) = e(g_1, g_2)^{a \cdot b}
> $$

TODO

## Exercise 43

> Consider the remainder class groups $(\mathbb{Z}_n, +)$ for some modulus $n$. Show that the following map is a pairing map:
>
> $$
> e(\cdot, \cdot) : \mathbb{Z}_n \times \mathbb{Z}_n \to \mathbb{Z}_n
> $$
>
> $$
> (a, b) \mapsto a \cdot b
> $$

TODO

## Exercise 44

> Consider $\mathbb{Z}_13^*$. Choose a set of 3 generators from this group, define it's associated Pedersen Hash Function and compute the Pedersen hash of $(3, 7, 11) \in \mathbb{Z}_{12}$.

TODO

## Exercise 45

> Consider the Pedersen Hash and the SHA256 hash function. Implement the related hash-to-group function in Sage.

See the code [here](./hashing.sage).

## Exercise 46

> Consider $\mathbb{Z}_13^*$ from example 34 and the parameter $k=3$. Choose a generator of $\mathbb{Z}_13^*$, a seed, and instantiate a member of the family given in equation (4.27) for that seed. Evaluate that member on the binary string $\langle 1, 0, 1 \rangle$.

TODO

## Exercise 47

> Consider the ring $\mathbb{Z}_5$. Show that it is also a field and write its characteristic. Then, prove that $ax = b$ has only a single solution $x \in \mathbb{Z}_5$ for any given $a, b \in \mathbb{Z}_5^*$.

We have the ring $\mathbb{Z}_5$ which is $(\{0, 1, 2, 3, 4\}, +, \times) $. This is a field, because all field rules hold and every member expect 0 has a multiplicative inverse. The characteristic of this field is $5$, because $\sum_{i=1}^5 1 = 0$ in $\mathbb{Z}_5$.

As for $ax = b$, there is only one solution because each $a$ has a single multiplicative inverse $a^{-1}$ in the multiplicative field, and we get $x = ba^{-1}$ which is a single element in the field.

## Exercise 56

> Consider the prime field $\mathbb{F}_5$. Show that the polynomial $P = x^2 + 2$ from $\mathbb{F}_5[X]$ is irreducible. Implement the finite field $\mathbb{F}_2$ in Sage.

See the code [here](./extension-field.sage).
