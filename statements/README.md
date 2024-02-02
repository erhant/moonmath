# Chapter 6: Statements

## Exercise 96

> Define a decision function such that the associated language $L_{Exercise_1}$ consists of all solutions to the equation $5x + 4 = 28 + 2x$ over $\mathbb{F}_{13}$.
>
> Provide a constructive proof for the claim: "There exists a word in $L_{Exercise_1}$" and verify the proof.

The decision function can be written as follows:

$$
R_{Exercise_1} : (\mathbb{F}_{13})^* \to \{true, false\} ;
$$

$$
\langle x_1, \ldots, x_n \rangle
\mapsto
\begin{cases}
true & n = 1 \text{ and } 5x_1+4 = 28 + 2x_1 \\
false & else
\end{cases}
$$

We can find the solution $x$ as follows:

- Moving $x$ to left-side and others to the right: $3x = 24$
- Taking modulo 13 to get: $3x = 11$
- Inverse of $3$ is $9$ in this field, so we have $x = 11 * 9$ which is $x = 8$ in mod 13.

So, the string $\langle 8 \rangle$ is a constructive proof and the computation $R_{Exercise_1}(\langle 8 \rangle) = true$ verifies the proof.

## Exercise 97

> Consider modular 6 arithmetic $(\mathbb{Z}_6)$, the alphabet $\Sigma = \mathbb{Z}_6$ and the following decision function:
>
> $$
>  R_{example_{11}} : \Sigma^\ast \to \{true, false\} ;
> $$
>
> $$
> \langle x_1, \ldots,  x_n \rangle
>   \mapsto
>   \begin{cases}
>   true & n = 1 \text{ and } 3x_1 + 3 = 0 \\
>   false & else
>   \end{cases}
> $$
>
> Compute all words in the associated language $L_{example_{11}}$, provide a constructive proof for the statement "There exist a word in $L_{example_{11}}$" and verify the proof.

Looking at words with $n=1$, we have $\langle 0 \rangle, \langle 1 \rangle, \langle 2 \rangle, \langle 3 \rangle, \langle 4 \rangle, \langle 5 \rangle$. From these, $\langle 1 \rangle$ and $\langle 5 \rangle$ fit the whole grammar as $3(1) + 3 = 6 \equiv 0 \pmod{6}$ and $3(5)+3 = 18 \equiv 0 \pmod{6}$.

## Exercise 98

> Consider modular 6 arithmetic $(\mathbb{Z}_6)$, the alphabets $\Sigma_I$ and $\Sigma_W$ and the following decision function:
>
> $$
>  R_{linear} : \Sigma^\ast \times \Sigma^\ast \to \{true, false\} ;
> $$
>
> $$
> (i; w)
>   \mapsto
>   \begin{cases}
>   true & |i| = 3 \text{ and } |w| = 1 \text{ and } i_1 \cdot w_1 + i_2 = i_3 \\
>   false & else
>   \end{cases}
> $$
>
> Which of the following instances has a proof of knowledge in $R_{linear}$?
>
> - $(3, 3, 0)$
> - $(2, 1, 0)$
> - $(4, 4, 2)$

- Witness is $1$ for $(3, 3, 0)$,
- Witness is $1$ for $(4, 4, 2)$,
- There is no witness for $(2, 1, 0)$, as we cant find a $w$ that makes the equation hold.

## Exercise 99 ✨

> Consider the TinyJubJub curve together with its twisted Edwards addition law. Define an instance alphabet $\Sigma_I$, a witness alphabet $\Sigma_W$ and a decision function $R_{add}$ with associated language $L_{add}$ such that a string $(i;w) \in \Sigma_I^\ast \times \Sigma_W^\ast$ is a word in $L_{add}$ if and only if $i$ is a pair of curve points on the TinyJubJub curve in Edwards form, and $w$ is the sum of those curve points.
>
> With that, give example of an instance with a proof, and another example with an instance with no proof.

Twisted Edwards addition law for TinyJubJub is the following $(a = 3, d = 8)$:

$$
(x_1, y_1) \oplus (x_2, y_2) =
\left(
\frac
{x_1y_2 + y_1x_2}
{1 + 8x_1x_2y_1y_2}
,
\frac
{y_1y_2 - 3x_1x_2}
{1 - 8x_1x_2y_1y_2}
\right)
$$

for $x_1, y_1, x_2, y_2 \in \mathbb{F}_{13}$. Our instance is composed of two points, each a pair of field elements; our witness is composed of a single point, a pair of field elements.

Both alphabets are thus field elements:

- $\Sigma_I = \mathbb{F}_{13}$
- $\Sigma_W = \mathbb{F}_{13}$

Perhaps we could set our alphabet to equal a pair of field elements, that is $(\mathbb{F}_{13})^2$, but I prefer the above approach.

To make things easier, we will also re-use $L_{tiny-jj}$ from a previous example, which is a language such that if $a \in L_{tiny-jj}$ then $a$ is a word that is a pair of field elements corresponding to a point on the curve.

With that, our grammar $R_{add}$ is defined as:

$$
R_{add} : (\mathbb{F}_{13})^\ast \times (\mathbb{F}_{13})^\ast \to \{true, false\} ;
$$

$$
(i ; w) \mapsto
\begin{cases}
true  & |i| = 4 \text{ and } |w| = 2  \\
      & \text{ and } (\langle i_1, i_2 \rangle) \in L_{tiny-jj} \\
      & \text{ and } (\langle i_3, i_4 \rangle) \in L_{tiny-jj} \\
      & \text{ and } (w_1, w_2) =
\left(
\frac
{i_1i_4 + i_2i_3}
{1 + 8i_1i_3i_2i_4}
,
\frac
{i_2i_4 - 3i_1i_3}
{1 - 8i_1i_3i_2i_4}
\right) \\

false & else
\end{cases}
$$

The language $L_{add}$ is defined as:

$$
L_{add} := \{(i;w) \in \Sigma_I^\ast \times \Sigma_W^\ast | R_{add}(i; w) = true \}
$$

To provide an instance with knowledge proof, we can just pick two points in the curve and sum them as the witness. To provide an instance with no proof, we can let one of the points to be outside the curve and that would suffice.

## Exercise 100

> Consider the language $L_{add}$ from exercise 99. Define an R1CS such that words in $L_{add}$ are in 1:1 correspondance with solutions to this R1CS.

TODO

## Exercise 101

> Consider the circuit $C_{tiny-jj}(\mathbb{F}_{13})$ with it's associated language $L_{tiny-jj}$. Construct a proof $\pi$ for the instance $\langle 11, 6 \rangle$ and verify the proof.

The said circuit outputs 0 when the given point coordinates $x, y$ hold for the equation of the TinyJubJub curve in twisted Edwards form:

$$
3(x^2) + y^2 = 1 + 8(x^2)(y^2)
$$

To be more precise, the circuit calculates the following expression which is just a re-arrangment of the above equation such that one side is equal to 0:

$$
1 + 8(x^2)(y^2) + 10(x^2) + 12(y^2)
$$

Here is the circuit using MermaidJS:

```mermaid
flowchart TD
  %% inputs & constants
	x
  y
  10
  12
  1
  8

  %% gates
  m1((*))
  m2((*))
  m3((*))
  m4((*))
  m5((*))
  m6((*))
  a1((+))
  a2((+))
  a3((+))

  %% wirings
  10 --> m1
  m2 --S_3--> m1
  x --S_1--> m2
  x --S_1--> m2
  y --S_2--> m3
  y --S_2--> m3
  12 --> m4
  m3 --S_4--> m4
  m2 --S_3--> m5
  m3 --S_4--> m5
  m1 --> a1
  1 --> a1
  8 --> m6
  m5 --S_5--> m6
  a1 --> a2
  m6 --> a2
  a2 --> a3
  m4 --> a3
  a3 --S6--> f_tiny-jj
```

The proof will be to find the correct values for wire labels given the inputs $\langle 11, 6 \rangle$. That is:

```mermaid
flowchart TD
  %% inputs & constants
	x[11]
  y[6]
  10
  12
  1
  8

  %% gates
  m1((*))
  m2((*))
  m3((*))
  m4((*))
  m5((*))
  m6((*))
  a1((+))
  a2((+))
  a3((+))

  %% wirings
  10 --10--> m1
  m2 --"S_3 = 4"--> m1
  x --"S_1 = 11"--> m2
  x --"S_1 = 11"--> m2
  y --"S_2 = 6"--> m3
  y --"S_2 = 6"--> m3
  12 --12--> m4
  m3 --"S_4 = 10"--> m4
  m2 --"S_3 = 4"--> m5
  m3 --"S_4 = 10"--> m5
  m1 --1--> a1
  1 --1--> a1
  8 --8--> m6
  m5 --"S_5 = 1"--> m6
  a1 --2--> a2
  m6 --8--> a2
  a2 --10--> a3
  m4 --3--> a3
  a3 --"S6 = 0"--> f_tiny-jj
```

Indeed, we got the output to be 0 and we have a valid set of assignments to prove it! Our proof $\pi$, that is the assignments for $\langle S_1, S_2, S_3, S_4, S_5, S_6 \rangle$ are:

$$
\pi = \langle 11, 6, 4, 10, 1, 0 \rangle
$$

## Exercise 102 ✨

> Consider the R1CS for points on the TinyJubJub curve. Compute an associated QAP for this R1CS and check your results with Sage.

We can define the `qap` function as:

```py
def qap(r1cs, p: int):
  '''
  Given a R1CS and a prime, returns the QAP (Quadratic Arithmetic Program).
  '''
  # k := number of constraints
  k = len(r1cs[0])
  assert(k < p)

  # make sure lengths are alright
  cnt = len(r1cs[0][0])
  for term in range(3):
    # each term must have k constraints
    # and each constraint must have same amount of terms
    for cons in range(k):
      assert(cnt == len(r1cs[term][cons]))

  # polynomial over GF(p)
  Fp = GF(p)
  Fpx = Fp["x"]

  # pick k random elements, unique & invertible
  elems = []
  for _ in range(k):
    rand_elem = Fp.random_element()
    if rand_elem not in elems:
      elems.append(rand_elem)

  # compute the target polynomial
  target = Fpx(1)
  for e in elems:
    target *= Fpx([-e, 1]) # x - e

  # compute the lagrange polynomials
  polys = ([], [], [])
  for term in range(3):
    for c in range(cnt):
      points = [(elems[cons], r1cs[term][cons][c]) for cons in range(k)]
      polys[term].append(Fpx.lagrange_polynomial(points))

  return (target, polys)
```

When we run this over the R1CS from example 121, we get:

```sh
# Target Polynomial
x^4 + 11*x^3 + 12*x^2 + 3*x + 12

# Polynomials (A)
[12*x^3 + x^2 + 2*x + 12, 8*x^3 + 11*x^2 + 8*x + 12, 12*x^3 + 12*x^2 + 11*x + 4, 7*x^3 + 8*x + 8, x^3 + 12*x^2 + 11*x + 1, 12*x^3 + x^2 + 2*x + 12]

# Polynomials (B)
[12*x^3 + x^2 + 2*x + 12, 8*x^3 + 11*x^2 + 8*x + 12, 12*x^3 + 12*x^2 + 11*x + 4, 0, 7*x^3 + 2*x^2 + 5*x + 12, 12*x^3 + x^2 + 2*x + 12]

# Polynomials (C)
[0, 0, 0, 8*x^3 + 11*x^2 + 8*x + 12, 12*x^3 + 12*x^2 + 11*x + 4, 7*x^3 + 2*x^2 + 5*x + 12]
```
