# puzzle-zero-sum-game
----------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/OTZePUDE)

[Submit a write-up](https://xng1lsio92y.typeform.com/to/Q1IC7Tok)

Puzzle description
==================

```
    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\

Bob has designed a new private payments protocol design, where every note comes with a secret 
polynomial f whose sum over a specific set is zero. This is enforced using a sumcheck protocol.
Once a note is spent, f is modified to a different polynomial whose sum isn't zero. One day, 
after an interesting conversation with her friends, Alice got an idea for an attack that can 
potentially allow her to double spend notes.

Alice successfully double spent a note. Can you figure out how she did it?

Be very careful, if the verifier somehow learns the sum of the modified f, 
they can deanonymize you.

In the rest of protocol that is not described here, the masking polynomial used by 
the prover is opened twice. Therefore, the masking polynomial cannot be a 
constant polynomial.

To see examples of sumcheck, you can review the protocol described in 
https://github.com/arkworks-rs/marlin/blob/master/diagram/diagram.pdf.
```
