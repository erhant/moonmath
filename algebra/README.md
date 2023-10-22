# Chapter 4: Algebra

Topics in this chapter:

- Commutative Groups
- Commutative Rings
- Fields
- Projective Planes

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

> Generalizing the previous exercise, consider $n$ and let $\mathbb{Z}_n^*$ be the set of all remainder classes from $\mathbb{Z}_n$ without the class 0, i.e. $\mathbb{Z}_n^* = \{1, 2, \ldots, n - 1\}$. Provide a counter-example to show that $(\mathbb{Z}_n^*, \cdot)$ is not a commutative group in general.
>
> Find a condition such that $(\mathbb{Z}_n^*, \cdot)$ is a commutative group, compute the neutral element, give a closed form for the inverse of any element and prove the commutative group axioms.

In general, $(\mathbb{Z}_n^*, \cdot)$ is a commutative group when $n$ is a prime number. In these groups, neutral element is always 1 and the inverse of any element can be given by Fermat's Little Theorem, which states:

$$
a^{n} \equiv a \pmod{n}
$$

When $a$ and $n$ are coprime, which is true in this case because $n$ is a prime, this congruence implies:

$$
a^{n-2} \equiv a^{-1} \pmod{n}
$$

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

I couldn't prove this myself, here is a [proof from YouTube](https://www.youtube.com/watch?v=EiM7U-WT40Q) that I saw in ZKHACK MoonMath study group. There is also this [question on StackExchange](https://math.stackexchange.com/questions/290427/is-mathbb-z-p-1-2-3-p-1-a-cyclic-group).

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

Cyclic group exponentiation made use of "square-and-multiply".

```py
# g^x (mod n)
def cge(g: int, x: int, n: int) -> int:
  h = g
  x >>= 1
  while x > 0:
    h = (h * h) % n   # square
    if x & 1 == 1:
      h = (h * g) % n # multiply
    x >>= 1
  return h
```

In efficient scalar multiplication, we will essentially use it's equivalent operations in additive groups, that is, "double-and-add". We call this "efficient scalar multiplication".

```py
# g*x (mod n)
def esm(g: int, x: int, n: int) -> int:
  h = g
  x >>= 1
  while x > 0:
    h = (h + h) % n   # double
    if x & 1 == 1:
      h = (h + g) % n # add
    x >>= 1
  return h
```

## Exercise 39

> Consider example 40, and show that $\mathbb{Z}_5^*[2]$ is a commutative group.

The group consists of two elements: $\{1, 4\}$.

```c
| * | 1 | 4 |
| 1 | 1 | 4 |
| 4 | 4 | 1 |
```

Looking at the table, we see that all rules of a commutative group hold.

## Exercise 40

> Consider the finite cyclic group $(\mathbb{Z}_6, +)$. Describe all subgroups of this group. Then, identify its large prime order subgroup, define its cofactor clearing map and apply that map to all elements of $\mathbb{Z}_6$.

Let's use Sage to do this. First, let us construct $\mathbb{Z}_6$ via:

```py
Z6 = Integers(6)
order = Z6.order()
```

The fundamental theorem of finite cyclic groups say that the factorization of the order of our group should give the order of its subgroups. Let us do that:

```py
factorization = list(map(lambda x : x[0], factor(order)))
subgroupOrders = [1, order] # add the trivial group and itself too
subgroupOrders.extend(factorization)
subgroupOrders = sorted(subgroupOrders)
```

We can do cofactor clearing for each subgroup to find the elements in it:

```py
for subgroupOrder in subgroupOrders:
  cf = order // subgroupOrder # find cofactor
  Z6_f_elems = []
  for e in range(0, order):
    Z6_f_elems.append(Z6(e) * cf)
  Z6_f_elems = set(Z6_f_elems)
  print("Z6[{}]  :{}".format(subgroupOrder, Z6_f_elems))
```

Doing this gives us:

```sh
Z6[1]  :{0}
Z6[2]  :{0, 3}
Z6[3]  :{0, 2, 4}
Z6[6]  :{0, 1, 2, 3, 4, 5}
```

The largest prime order here is that of `Z6[3]` with elements `{0, 2, 4}`. Also notice that for the cofactor clearing we did multiplication, not exponentiation; the latter is done when the group is multiplicative, but the former is done when the group is additive!

## Exercise 41 ✨

> Let $(\mathbb{Z}_p^*, \cdot)$ be a cyclic group. Show that for $p \geq 5$ not every element of $\mathbb{Z}_p^*$ is a generator of $\mathbb{Z}_p^*$.

A trivial answer is that $1 \in \mathbb{Z}_p^*$ but does not generate the group. Other than that, for larger groups for prime $p$, their order will be $p-1$. This is an even number, indicating that all these prime order groups have a subgroup of order 2.

So, there will always be an element other than 1 that generates the subgroup of order 2, which obviously can't generate the group itself. The only expection is when $p=3$, since that group has order 2, hence the reason the question asks for $p \geq 5$ for us.

## Exercise 42 ✨

> **Arithmetic Laws for Pairing Maps:** Let $\mathbb{G}_1, \mathbb{G}_2$ and $\mathbb{G}_3$ be finite cyclic groups of the same order $n$, and let $e(\cdot, \cdot) = \mathbb{G}_1 \times \mathbb{G}_2 \to \mathbb{G}_3$ be a pairing map.
>
> Show that for a given $g_1 \in \mathbb{G}_1, g_2 \in \mathbb{G}_2$ and all $a, b \in \mathbb{Z}_n$ the following identity holds:
>
> $$
> e(g_1^a, g_2^b) = e(g_1, g_2)^{a \cdot b}
> $$

The bilinearity property tells us that:

$$
e(g_1 \cdot g_1', g_2) = e(g_1, g_2) \cdot e(g_1', g_2)
$$

If $g_1' = g_1$ we get:

$$
e(g_1^2, g_2) = e(g_1, g_2) \cdot e(g_1, g_2)
$$

In fact, this can be generalized:

$$
e(g_1^a, g_2) = \underbrace{e(g_1, g_2) \cdot e(g_1, g_2) \cdot \ldots \cdot e(g_1, g_2)}_{a \text{ times}}
$$

We can do the same for $g_2$ as well due to the other identity:

$$
e(g_1^a, g_2^b) = \underbrace{e(g_1^a, g_2) \cdot e(g_1^a, g_2) \cdot \ldots \cdot e(g_1^a, g_2)}_{b \text{ times}}
$$

Since each $e(g_1^a, g_2)$ has $e(g_1, g_2)$ in it $a$ times, and we have $b$ of them, we get a total of $a \cdot b$ many terms thus proving that:

$$
e(g_1^a, g_2^b) = e(g_1, g_2)^{a \cdot b}
$$

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

In example 41, we already saw that the following is a non-degenerate pairing:

$$
e(\cdot, \cdot) : \mathbb{Z} \times \mathbb{Z} \to \mathbb{Z}
$$

It is non-degenerate because there is only one way to result in 0 here, that is one of the inputs must be 0. Things are different in mod $n$ though!

If $n$ is a composite number, the inputs can be the factors of it and it will result in the netural element in mod $n$, which makes the pairing not non-degenerate.

The bilinearity property holds due to distribute law of integers mod $n$, similar to the $\mathbb{Z}$ example.

## Exercise 44

> Consider $\mathbb{Z}_{13}^*$. Choose a set of 3 generators from this group, define it's associated Pedersen Hash Function and compute the Pedersen hash of $(3, 7, 11) \in \mathbb{Z}_{12}$.

Generators of $\mathbb{Z}_{13}^*$ are $\{2, 6, 7, 11\}$. Let's choose $2, 6, 7$ and define our Pedersen Hash Function as:

$$
H(x_1, x_2, x_3) = 2^{x_1}6^{x_2}11^{x_3} \pmod{13}
$$

Writing the generic code for it:

```py
# xs: inputs, gs: generators, N: order
def pedersen(xs, gs, N):
  assert(len(xs) == len(gs))

  Z = Integers(N)
  ans = 1
  for i in range(len(gs)):
    ans *= Z(gs[i]) ^ xs[i]

  return ans
```

We then find `pedersen([3, 7, 11], [2, 6, 11], 13)` to be `11`.

## Exercise 45

> Consider the Pedersen Hash and the SHA256 hash function. Implement the related hash-to-group function in Sage.

Here is the code for this:

```py
# s: input, gs: generators, N: modulus
def sha256_pedersen(s, gs, N):
  # sha256 hash-to-integer
  z = sha256(s)

  # get binary digits
  z_bin = z.digits(base = 2, padto = 256)

  # multiply with powers of the generators
  Z = Integers(N)
  ans = 1
  for i in range(len(gs)):
    ans *= Z(gs[i]) ^ z_bin[i]

  return ans
```

## Exercise 46

> Consider $\mathbb{Z}_{13}^*$ from example 34 and the parameter $k=3$. Choose a generator of $\mathbb{Z}_{13}^*$, a seed, and instantiate a member of the family given in equation (4.27) for that seed. Evaluate that member on the binary string $\langle 1, 0, 1 \rangle$.

TODO: (is $k$ really 3 here? in that case, the input must be 4 bits long.)

## Exercise 47

> Consider the ring $\mathbb{Z}_5$. Show that it is also a field and write its characteristic. Then, prove that $ax = b$ has only a single solution $x \in \mathbb{Z}_5$ for any given $a, b \in \mathbb{Z}_5^*$.

We have the ring $\mathbb{Z}_5$ which is $(\{0, 1, 2, 3, 4\}, +, \times) $. This is a field, because all field rules hold and every member expect 0 has a multiplicative inverse. The characteristic of this field is $5$, because $\sum_{i=1}^5 1 = 0$ in $\mathbb{Z}_5$.

As for $ax = b$, there is only one solution because each $a$ has a single multiplicative inverse $a^{-1}$ in the multiplicative field, and we get $x = ba^{-1}$ which is a single element in the field.

## Exercise 47

> Consider the ring $(\mathbb{Z}_5, +, \cdot)$. Show that it is a field, then:
>
> - Find it's characteristic.
> - Show that $ax = b$ has only a single solution $x \in \mathbb{Z}_5$ for any given $a, b \in \mathbb{Z}_5^*$

We know that $(\mathbb{Z}_5, +)$ is a commutative group with neutral element 0, and we also know $(\mathbb{Z}_5 \setminus \{0\}, \cdot)$ is a commutative group (i.e. the multiplicative group $\mathbb{Z}_5^*$) with neutral element 1. Distributivity holds for all elements in the field too.

- It's characteristic is 5, because adding 1 to itself 5 times result in 0 modulo 5.

- In the equation, $a$ has a unique inverse and therefore $ba^{-1}$ is a unique number for all $a, b \in \mathbb{Z}_5^*$. Since $x = ba^{-1}$ that means there is only a single (unique) solution.

## Exercise 48

> Consider the ring $(\mathbb{Z}_6, +, \cdot)$. Show that it is not a field.

Not every element in $\mathbb{Z}_6^*$ have an inverse, therefore this ring is not a field.

## Exercise 49

> Construct the addition and multiplication table of the prime field $\mathbb{F}_3$

```js
sage: F3 = GF(3)
sage: F3.addition_table('elements')
+  0 1 2
 +------
0| 0 1 2
1| 1 2 0
2| 2 0 1

sage: F3.multiplication_table('elements')
*  0 1 2
 +------
0| 0 0 0
1| 0 1 2
2| 0 2 1
```

## Exercise 50

> Construct the addition and multiplication table of the prime field $\mathbb{F}_{13}$

```js
sage: F13 = GF(13)
sage: F13.addition_table('elements')
 +   0  1  2  3  4  5  6  7  8  9 10 11 12
  +---------------------------------------
 0|  0  1  2  3  4  5  6  7  8  9 10 11 12
 1|  1  2  3  4  5  6  7  8  9 10 11 12  0
 2|  2  3  4  5  6  7  8  9 10 11 12  0  1
 3|  3  4  5  6  7  8  9 10 11 12  0  1  2
 4|  4  5  6  7  8  9 10 11 12  0  1  2  3
 5|  5  6  7  8  9 10 11 12  0  1  2  3  4
 6|  6  7  8  9 10 11 12  0  1  2  3  4  5
 7|  7  8  9 10 11 12  0  1  2  3  4  5  6
 8|  8  9 10 11 12  0  1  2  3  4  5  6  7
 9|  9 10 11 12  0  1  2  3  4  5  6  7  8
10| 10 11 12  0  1  2  3  4  5  6  7  8  9
11| 11 12  0  1  2  3  4  5  6  7  8  9 10
12| 12  0  1  2  3  4  5  6  7  8  9 10 11

sage: F13.multiplication_table('elements')
 *   0  1  2  3  4  5  6  7  8  9 10 11 12
  +---------------------------------------
 0|  0  0  0  0  0  0  0  0  0  0  0  0  0
 1|  0  1  2  3  4  5  6  7  8  9 10 11 12
 2|  0  2  4  6  8 10 12  1  3  5  7  9 11
 3|  0  3  6  9 12  2  5  8 11  1  4  7 10
 4|  0  4  8 12  3  7 11  2  6 10  1  5  9
 5|  0  5 10  2  7 12  4  9  1  6 11  3  8
 6|  0  6 12  5 11  4 10  3  9  2  8  1  7
 7|  0  7  1  8  2  9  3 10  4 11  5 12  6
 8|  0  8  3 11  6  1  9  4 12  7  2 10  5
 9|  0  9  5  1 10  6  2 11  7  3 12  8  4
10|  0 10  7  4  1 11  8  5  2 12  9  6  3
11|  0 11  9  7  5  3  1 12 10  8  6  4  2
12|  0 12 11 10  9  8  7  6  5  4  3  2  1
```

## Exercise 51

> Consider the prime field $\mathbb{F}_{13}$. Find the set of all pairs $(x, y) \in \mathbb{F}_{13} \times \mathbb{F}_{13}$ that satisfy the following equation:
>
> $$
> x^2 + y^2 = 1 + 7 \cdot x^2 \cdot y^2
> $$

We can use Sage for this.

```py
F13 = GF(13)
pairs = []
elems = list(map(lambda n : F13(n), range(0, 13)))
for x in elems:
  for y in elems:
    lhs = x * x + y * y
    rhs = 1 + 7 * x * x * y * y
    if lhs == rhs:
      pairs.append((x, y))
print(pairs)
```

This gives us the pair of elements:

```
[(0, 1), (0, 12), (1, 0), (2, 4), (2, 9), (4, 2), (4, 11), (5, 6), (5, 7), (6, 5), (6, 8), (7, 5), (7, 8), (8, 6), (8, 7), (9, 2), (9, 11), (11, 4), (11, 9), (12, 0)]
```

Notice that if $(a, b)$ is a valid pair, so is $(b, a)$ due to commutativity in the equation.

## Exercise 52

> Consider the prime field $\mathbb{F}_{13}$. Compute the Legendre symbol $(\frac{x}{13})$ and the set of roots $\sqrt{x}$ for all elements $x \in \mathbb{F}_{13}$

Let's write the code for Legendre symbol:

```py
def legendre_symbol(y, p):
  assert(p % 2 == 1)
  l = y ^ ((p - 1) // 2) % p
  return -1 if l == p - 1 else l
```

Now, let's find the Legendre symbol & square root for each element:

```py
F13, legendres, sqrts = GF(13), {}, {}
for e in F13:
  sqrts[e] = []
for e in F13:
  legendres[e] = legendre_symbol(e, 13)
  sqrts[e * e].append(e)
```

When we print this nicely, we see the following:

```py
N = 0   Symbol: 0       Sqrts: [0]
N = 1   Symbol: 1       Sqrts: [1, 12]
N = 2   Symbol: -1      Sqrts: []
N = 3   Symbol: 1       Sqrts: [4, 9]
N = 4   Symbol: 1       Sqrts: [2, 11]
N = 5   Symbol: -1      Sqrts: []
N = 6   Symbol: -1      Sqrts: []
N = 7   Symbol: -1      Sqrts: []
N = 8   Symbol: -1      Sqrts: []
N = 9   Symbol: 1       Sqrts: [3, 10]
N = 10  Symbol: 1       Sqrts: [6, 7]
N = 11  Symbol: -1      Sqrts: []
N = 12  Symbol: 1       Sqrts: [5, 8]
```

As observed, we have two square roots (positive and negative) for each element that has Legendre symbol equal to 1.

## Exercise 53

> Consider the extension field $\mathbb{F}_{3^2}$ from the previous example and find all pairs of elements $(x, y) \in \mathbb{F}_{3^2}$ for which the following equation holds:
>
> $$
> y^2 = x^3 + 4
> $$

Using Sage:

```py
F3_2 = GF(3^2, name="x", modulus=GF(3)['x']([1, 0, 1]))
solutions = []
for x in F3_2:
  for y in F3_2:
    if y^2 == x^3 + 4:
      solutions.append((x, y))
print(solutions)
```

We get the results:

```sh
[(0, 2), (0, 1), (x + 2, 2*x + 2), (x + 2, x + 1), (2*x + 2, x + 2), (2*x + 2, 2*x + 1), (2, 0), (1, x), (1, 2*x)]
```

I'm not sure how to solve this one pen-and-paper.

## Exercise 54

> Show that the polynomial $Q = x^2 + x + 2$ from $\mathbb{F}_3[x]$ is irreducible. Construct the multiplication table of $\mathbb{F}_{3^2}$ with respect to $Q$ polynomial.

Using Sage:

```py
F3 = GF(3)
F3x = F3['x']
Q = F3x([2, 1, 1]) # x^2 + x + 2
```

To check if it is irreducible, we will evaluate it over all elements in the field and see if the result is zero or not. If there is a zero, it means there is a factor for that element, i.e. if $Q(a) = 0$ then there is a factor $(x - a)$, meaning that it is reducible.

```py
is_irreducible = True
for elem in F3:
  if Q(elem) == 0:
    is_irreducible = False

# compare with Sage's function for this
assert(is_irreducible == Q.is_irreducible())
```

Finally, we print the multiplication table:

```py
F3_2 = GF(3^2, name="x", modulus=Q)
print(F3_2.multiplication_table('elements'))
```

```sh
      *        0       1       2       x   x + 1   x + 2     2*x 2*x + 1 2*x + 2
       +------------------------------------------------------------------------
      0|       0       0       0       0       0       0       0       0       0
      1|       0       1       2       x   x + 1   x + 2     2*x 2*x + 1 2*x + 2
      2|       0       2       1     2*x 2*x + 2 2*x + 1       x   x + 2   x + 1
      x|       0       x     2*x 2*x + 1       1   x + 1   x + 2 2*x + 2       2
  x + 1|       0   x + 1 2*x + 2       1   x + 2     2*x       2       x 2*x + 1
  x + 2|       0   x + 2 2*x + 1   x + 1     2*x       2 2*x + 2       1       x
    2*x|       0     2*x       x   x + 2       2 2*x + 2 2*x + 1   x + 1       1
2*x + 1|       0 2*x + 1   x + 2 2*x + 2       x       1   x + 1       2     2*x
2*x + 2|       0 2*x + 2   x + 1       2 2*x + 1       x       1     2*x   x + 2
```

## Exercise 55

> Show that the polynomial $P = t^3 + t + 1$ from $\mathbb{F}_5[x]$ is irreducible. Then, consider the extension field $\mathbb{F}_{5^3}$ defined relative to $P$. Compute the multiplicative inverse of $(2t^2 + 4) \in \mathbb{F}_{5^3}$ using Extended Euclidean Algorithm. Then, find all $x \in \mathbb{F}_{5^3}$ that solve the following equation:
>
> $$
> (2t^2 + 4)(x - (t^2 + 4t + 2)) = (2t + 3)
> $$

To show irreducibility, I cant see any other way than showing it evaluates to non-zero on all points. Indeed, $P(0) = 1, P(1) = 3, P(2) = 1, P(3) = 1, P(4) = 4$ so $P$ is irreducible.

I've used Sage to find the inverse:

```py
F5 = GF(5)
F5x = F5['t']
P = F5x([1, 1, 0, 1])

F5_3 = GF(5^3, name="t", modulus=P)

print(xgcd(F5_3([4, 0, 2]), P))
# (1, 4*t^2 + 4*t + 1, 0)
```

Here we see that $4*t^2 + 4*t + 1$ is the inverse of $2t^2 + 4$. Let's solve the equation now. We can begin by multiplying both sides with $(2t^2 + 4)^{-1}$ which we have just found:

$$
x - (t^2 + 4t + 2) = (2t + 3)(4t^2 + 4t + 1)
$$

$$
x - (t^2 + 4t + 2) = 3t^3 + 3t^2 + 2t + 2t^2 + 2t + 3 = 3t^3 + 4t + 3
$$

$$
x = t^2 + 4t + 2 + 3t^3 + 4t + 3 = 3t^3 + t^2 + 3t
$$

Note that this is larger than our modulus with equal degree, so we have to find the remainder:

```py
F5x([0, 3, 1, 3]) % F5x([1, 1, 0, 1])
# t^2 + 2
```

We find $x = t^2 + 2$ as the solution.

## Exercise 56

> Consider the prime field $\mathbb{F}_5$. Show that the polynomial $P = x^2 + 2$ from $\mathbb{F}_5[X]$ is irreducible. Implement the finite field $\mathbb{F}_{5^2}$ in Sage.

Similar to exercise 54, we use Sage:

```py
F5 = GF(5)
F5x.<x> = F5[] # type: ignore

# is irreducible?
P = F5x(x^2 + 2) # type: ignore
is_irreducible = True
for elem in F5:
  if P(elem) == 0:
    is_irreducible = False
# also check Sage
assert(is_irreducible == P.is_irreducible())

# print elements
F5_2 = GF(5^2, name="x", modulus=P)
print(F5_2, ":")
for elem in F5_2:
  print(" ", elem)
```

Another way to show that $P$ is irreducible would be to check that all evaluations of this polynomial are non-zero. In other words, we must show that $\forall x \in \mathbb{F}_5 : x^2 + 2 \ne 0$.

This is equivalent to checking for $\forall x \in \mathbb{F}_5 : x^2 \ne 3$, which would be true if $3$ is a quadratic non-residue in this field. We can use Legendre Symbol to check for that:

$$
3^\frac{5-1}{2} = 3^2 = 9 \equiv 4 \equiv -1 \pmod{5}
$$

Finding -1 means that it is a quadratic non-residue, therefore $P$ is irreducible!

## Exercise 57

> Construct the so-called Fano Plane, i.e. the projective plane over the finite field $\mathbb{F}_2$

From the formula of number of elements $p^{2m} + p^m + 1$ for the projective plane over $\mathbb{F}_{p^m}$, we know that $F_{2^1}$ should have $4 + 2 + 1 = 7$ points. Let's write each:

```py
   [0:0:0] (excluded)
# affine points [X:Y:1]
1. [0:0:1]
2. [0:1:1]
3. [1:0:1]
4. [1:1:1]
# points at infinity [X:Y:0]
5. [0:1:0]
6. [1:0:0]
7. [1:1:0]
```

Notice that since $\mathbb{F}_2^* = \{1\}$, we cant obtain another point from a given point, because we can only multiply the coordinate by 1.

Double checking our results with Sage:

```py
for e in ProjectiveSpace(GF(2), 2):
  print(e)
```

```sh
(0 : 0 : 1)
(0 : 1 : 1)
(1 : 0 : 1)
(1 : 1 : 1)
(0 : 1 : 0)
(1 : 1 : 0)
(1 : 0 : 0)
```
