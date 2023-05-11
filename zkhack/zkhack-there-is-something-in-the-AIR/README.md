zkhack-there-is-something-in-the-AIR
-------------------

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/aUSCQrcW)

Puzzle description
==================

```
    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\

Alice implemented a Semaphore protocol to collect anonymous votes from her friends on various
topics. She collected public keys from 7 of her friends, and together with her public key, built
an access set out of them.

During one of the votes, Alice collected 9 valid signals on the same topic. But that should not be
possible! The semaphore protocol guarantees that every user can vote only once on a given topic.
Someone must have figured out how to create multiple signals on the same topic.

Below is a transcript for generating a valid signal on a topic using your private key. Can you
figure out how to create a valid signal with a different nullifier on the same topic?
```
