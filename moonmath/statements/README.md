# Selected Exercises

## Exercise 96

> Define a decision function such that the associated language $L_{Exercise_1}$ consists of all solutions to the equation $5x + 4 = 28 + 2x$ over $\mathbb{F}_{13}$. Provide a constructive proof for the claim: "There exists a word in $L_{Exercise_1}$" and verify the proof.

The decision function can be written as follows:

$$
R_{Exercise_1} : (\mathbb{F}_{13})^* \to \{true, false\} ; <x_1, \ldots, x_n>
$$

$$
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

So, the string $<8>$ is a constructive proof and the computation $R_{Exercise_1}(<8>) = true$ verifies the proof.

## Exercise 97

> Consider modular 6 arithmetic $(\mathbb{Z}_6)$, the alphabet $\Sigma = \mathbb{Z}_6$ and the following decision function:
>
> $$
>  R_{example_{11}} : \Sigma^* \to \{true, false\} ; <x_1, \ldots,  x_n>
> $$
>
> $$
>   \mapsto
>   \begin{cases}
>   true & n = 1 \text{ and } 3x_1 + 3 = 0 \\
>   false & else
>   \end{cases}
> $$
>
> Compute all words in the associated language $L_{example_{11}}$, provide a constructive proof for the statement "There exist a word in $L_{example_{11}}$" and verify the proof.

Looking at words with $n=1$, we have $<0>, <1>, <2>, <3>, <4>, <5>$. From these, $<1>$ and $<5>$ fit the whole grammar as $3(1) + 3 = 6 \equiv 0 \pmod{6}$ and $3(5)+3 = 18 \equiv 0 \pmod{6}$.
