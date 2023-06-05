# Selected Exercises

## Exercise 33

> Let $\mathbb{Z}_5^*$ be the set of all remainder classes from $\mathbb{Z}_5$ without the class 0, meaning that $\mathbb{Z}_5^* = \mathbb{Z}_5 \backslash \{0\}$. Show that $(\mathbb{Z}_5, \cdot)$ is a commutative group (aka. Abelian Group).

The map $\cdot$ (multiplication) of all elements in $\mathbb{Z}_5$ have the following properties:

- Closed due to multiplication
- Associative due to multiplication
- Commutative due to multiplication
- Identity is $1 \in \mathbb{Z}_5$
- Inverse exists as all elements are co-prime to $5$

With all these properties, the group is commutative. We can also see this from the table at example 16.

## Exercise 35

> Let $n \in \mathbb{N}$ with $n \geq 2$ be some moodulus. What is the order of the remainder class group $(\mathbb{Z}_n, +)$?

Well, it is $n$.

## Exercise 37 ✨

> Let $p \in \mathbb{P}$ be prime number and show that $(\mathbb{Z}_p^*, \cdot)$ is cyclic, i.e. it has a not necessarily unique single element that is able to generate the entire group.

Let $x = g^a \in \mathbb{Z}_p^*$ and $y = g^{a+k} \in \mathbb{Z}_p^*$ where $x = y$. This would mean $g^a = g^{a+k}$ and thus $1 = g^k$. This means that $g^k$ is the inverse of $g$. However, this equation holds for all $1 \leq k < p$, which contradicts the fact that the natural element is unique. Therefore, $\forall 1 \leq k < p : x \ne y $ and this means there are $p-1$ different elements given by the powers of $g$, eventually creating the entire set.

## Exercise 38

> Let $(\mathbb{G}, +)$ be a finite cyclic group of order $n$. Consider "cyclic group exponentiation" and consider its analog for groups in additive notation.

Cyclic group exponentiation made use of "square-and-multiply". In efficient scalar multiplication, we will essentially use it's equivalent operations in additive groups, that is, "double-and-add". See the code [here](./cyclic-group-exponentiation.py)

## Exercise 41 ✨

> Let $(\mathbb{Z}_p^*, .)$ be a cyclic group. Show that for $p \geq 5$ not every element of $\mathbb{F}_p^*$ is a generator of $\mathbb{F}_p^*$

**TODO TODO TODO**

## Exercise 45

> Consider the Pedersen Hash and the SHA256 hash function. Implement the related hash-to-group function in Sage.

See the code [here](./hashing.sage).

## Exercise 47

> Consider the ring $\mathbb{Z}_5$. Show that it is also a field and write its characteristic. Then, prove that $ax = b$ has only a single solution $x \in \mathbb{Z}_5$ for any given $a, b \in \mathbb{Z}_5^*$.

We have the ring $\mathbb{Z}_5$ which is $(\{0, 1, 2, 3, 4\}, +, \times) $. This is a field, because all field rules hold and every member expect 0 has a multiplicative inverse. The characteristic of this field is $5$, because $\sum_{i=1}^5 1 = 0$ in $\mathbb{Z}_5$.

As for $ax = b$, there is only one solution because each $a$ has a single multiplicative inverse $a^{-1}$ in the multiplicative field, and we get $x = ba^{-1}$ which is a single element in the field.

## Exercise 56

> Consider the prime field $\mathbb{F}_5. Show that the polynomial $P = x^2 + 2$ from $\mathbb{F}_5[X]$ is irreducible. Implement the finite field $\mathbb{F}_2$ in Sage.

See the code [here](./extension-field.sage).
