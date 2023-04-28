zkhack-hidden-in-plain-sight
-------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/pBnDwVNe)

Puzzle description
==================

```
One-thousand accounts participate in a shielded pool which hides the
recipients and other data in each transaction between parties in the pool. The
*recipient* is a 256-bit account address which is hidden by blinding a KZG-like
polynomial commitment to the address. The sender of the transaction chooses two
secret *blinding factors* known only to them by which the polynomial commitment
is blinded. Included with the commitment are two openings used to verify the
commitment. You intercept a transaction by observing the shielded pool. Armed
with the blinded commitment and two openings from the intercepted transaction as
well as the public data (the trusted setup for the KZG commitment scheme, a list
of all one-thousand account addresses participating in the pool, and the two
random challenges used to compute the openings) can you deanonymize the
recipient address?

Blinding Scheme
---------------
The 256-bit recipient address is split into a vector of 32
bytes, and each byte (as a BLS12-381 scalar) becomes a coefficient of a
degree-31 polynomial P. There is an evaluation domain H = {1, ω, ω^2, ...,
ω^(n-1)} with $ω^n = 1$ and a vanishing polynomial Z_H(x) = x^n -1 which
evaluates to 0 on each element of the evaluation domain. The sender of the
transaction chooses two secret blinding factors b_0 and b_1 and computes the
blinded polynomial Q(x) = P(x) + (b_0 + b_1x) • Z_H(x).

Polynomial Commitment
---------------------

The KZG-like polynomial commitment scheme uses a public
trusted setup S={g, g•s, ..., g•s^(n+1)}$ where g is the generator point of the
BLS12-381 elliptic curve and s is a secret scalar. For the polynomial Q(x) = c_0
+ c_1x+c_2x^2 + ... + c_(n+1)x^(n+1) the commitment com(Q) = c_0•g + c_1•g•s +
c_2•g•s^2 + ... + c_(n+1)•g•s^(n+1).

Openings
--------
Openings of the polynomial are required in order to verify the
polynomial commitment. These are simple evaluations of the polynomial at random
challenges which are public. For instance, for challenges z_1 and z_2, the
openings are Q(z_1) and Q(z_2).
```
