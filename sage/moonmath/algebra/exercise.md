# Selected Exercises

## Exercise 33

> Let $\mathbb{Z}_5^*$ be the set of all remainder classes from $\mathbb{Z}_5$ without the class 0, meaning that $\mathbb{Z}_5^* = \mathbb{Z}_5 \backslash \{0\}$. Show that $(\mathbb{Z}_5, \cdot)$ is a commutative group (aka. Abelian Group).

The map $\cdot$ (multiplication) of all elements in $\mathbb{Z}_5$ have the following properties:

- Closed due to multiplication
- Associative due to multiplication
- Commutative due to multiplication
- Identity is $1 \in \mathbb{Z}_5$
- Inverse as all elements are co-prime to $5$

With all these properties, the group is commutative.

## Exercise 35

> Let $n \in \mathbb{N}$ with $n \geq 2$ be some moodulus. What is the order of the remainder class group $(\mathbb{Z}_n, +)$?

Well, it is $n$.

## Exercise 37 ✨

> Let $p \in \mathbb{P}$ be prime number and show that $(\mathbb{Z}_p^*, \cdot)$ is cyclic, i.e. it has a not necessarily unique single element that is able to generate the entire group.

Let $x = g^a \in \mathbb{Z}_p^*$ and $y = g^{a+k} \in \mathbb{Z}_p^*$ where $x = y$. This would mean $g^a = g^{a+k}$ and thus $1 = g^k$. This means that $g^k$ is the inverse of $g$. However, this equation holds for all $1 \leq k < p$, which contradicts the fact that the natural element is unique. Therefore, $\forall 1 \leq k < p : x \ne y $ and this means there are $p-1$ different elements given by the powers of $g$, eventually creating the entire set.

## Exercise 38

> Let $(\mathbb{G}, +)$ be a finite cyclic group of order $n$. Consider "cyclic group exponentiation" and consider its analog for groups in additive notation.

Cyclic group exponentiation made use of "square-and-multiply". In efficient scalar multiplication, we will essentially use it's equivalent operations in additive groups, that is, "double-and-add". See the code [here](./cyclic-group-exponentiation.py)

## Exercise 41

> Let $(\mathbb{Z}_p^*, .)$ be a cyclic group. Show that for $p \geq 5$ not every element of $\mathbb{F}_p^*$ is a generator of $\mathbb{F}_p^*$

...
