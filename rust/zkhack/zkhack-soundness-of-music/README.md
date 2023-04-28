zkhack-soundness-of-music
-------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/fTcjb1l0)

Puzzle description
==================

```
    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\

It is a well-known fact that 1+1=2. Recent work by Alice von Trapp (et al) suggests that under
special conditions in the Swiss Alps, 1+1+1+1=1. Alice has been unable to prove this statement over the BLS12-381 scalar field.
The primary difficulty appears to be the fact that 1 is not equal to 4. Alice's proving system
can write a proof for every statement (x,y) where x and y are BLS12-381 scalars, and x+x+x+x=y.
The proving system easily outputs a proof for the statement (1,4) showing 1+1+1+1=4, but seems
unable to produce a proof for the statement (1,1) showing 1+1+1+1=1.

A proof for the statement (1,1) is badly needed for the elliptic curve finale song to be added to Appendix A.
Can you save the show and output a proof where verify(&[Fr::one(), Fr::one()], &setup, &proof) = true ?
```
