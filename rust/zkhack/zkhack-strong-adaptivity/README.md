zkhack-strong-adaptivity
-------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/xdTZaHw1)

Puzzle description
==================

```
Shallan recently found a proof system (see below) that enables proving that two
Pedersen commitments commit to the same message (but with potentially different
randomness). She employes this in her private cryptocurrency to show that two
committed coins have the same value. However, soon after deployment, she
receives a message from a self-proclaimed hacker. The message contains two
Pedersen commitments and their openings, and a proof of message equality for
these commitments. The proof is valid, but there's a twist: the openings contain
different messages! How can this be? Reproduce the attack and help Shallan
diagnose the problem in her system.


The Proof of message equality is obtained by applying the Fiat--Shamir transform to the following sigma protocol:

        Prover                                           Verifier
=================================================================================================
Offline phase:
1. Prover computes 
    C_1 := PedersenCOMM(a; r1) = a * G + r1 * H
    C_2 := PedersenCOMM(a; r2) = a * G + r2 * H

    where G and H are generators of the group, and r1 and r2 are random field elements.
                            ------- C_1, C_2 ------->

Online phase:

1. Prover samples random elements r, ρ, τ.
2. Prover computes 
    C_ρ := PedersenCOMM(r; ρ)
    C_τ := PedersenCOMM(r; τ)
                            ------- C_ρ, C_τ ------->
                            <- random challenge e ---
3. Prover computes 
    s := r + e * a,
    u := ρ + e * r1
    t := τ + e * r2
                            -------- s, u, t ------->
                                                Check PedersenCOMM(s; u) = C_ρ + eC_1
                                                Check PedersenCOMM(s; t) = C_τ + eC_2
==================================================================================================
```
