zkhack-can-you-turn-up-the-heat
-------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Creator
=======

[Bobbin Threadbare](https://github.com/bobbinth)

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/iVTqxqnZ)

Puzzle description
==================

```
    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\

Alice built a STARK prover and a verifier for verifying correct computation of the Fibonacci
sequence. Her friends would generate proofs for various terms send them to her. She'd verify
these proofs and every time the proof was accepted by the verifier, she'd feel joy.

One day, she received two proofs attesting to the correct computation of the 32nd term of the
Fibonacci sequence. These proofs claimed different results - but both proofs were accepted by the
verifier. This should not have been possible! Worried, Alice computed the 32nd term of the sequence
manually. It was 832040, but one of the proofs was claiming that the 32nd term was 123! Something
must have gone terribly wrong...

Can you figure out how to crate a fake proof at the same security level as the valid proof received
by Alice?
```
