# puzzle-bigger-is-better

**DO NOT FORK THE REPOSITORY, AS IT WILL MAKE YOUR SOLUTION PUBLIC. INSTEAD, CLONE IT AND ADD A NEW REMOTE TO A PRIVATE REPOSITORY, OR SUBMIT A GIST**

Trying it out
=============

Use `cargo run --release` to see it in action

Submitting a solution
=====================

[Submit a solution](https://xng1lsio92y.typeform.com/to/iNdMqB7I)

[Submit a write-up](https://xng1lsio92y.typeform.com/to/nxqx9Edt)

Puzzle description
==================


    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\


Bob was catching up on the latest in zkSNARK research, and came across the
Vampire paper [1]. In that paper, he found a reference to an inner-product
commitment scheme [2], which allows committing to a vector and later proving
that its inner-product with another (public) vector is equal to a claimed value.
Bob was intrigued by this scheme, and decided to implement it in Rust.

Bob was delighted with the performance of the resulting implementation, and so
decided to deploy it. The scheme requires a universal Powers-of-Tau-type trusted setup,
and so Bob generated a SRS using an MPC ceremony.

Things were going smoothly for a while, but then Bob received an anonymous email that
contained a full break of the scheme! Unfortunately for Bob, the email didn't contain
any details about the break. Can you help Bob figure out the issue, and fix his scheme?

[1]: https://ia.cr/2022/406
[2]: http://www.lsv.fr/Publis/PAPERS/PDF/ILV-imacc11-long.pdf