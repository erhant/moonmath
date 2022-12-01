# Cryptopals

This repository has my [cryptopals](https://cryptopals.com/) solutions, which are a set of cryptography-related programming challenges.

Each challenge is run as a test, stored under `pkg/setN/chalM_test.go` for some set `N` and challenge `M`. I try to put all set related codes under that respective set folder, though sometimes a code is used by many challenges on different sets.

## Challenges

- [Set 1](./pkg/set1/): [Basics](https://cryptopals.com/sets/1)
  - [Challenge 1](./pkg/set1/chal1_test.go): [Convert `hex` to `base64`](https://cryptopals.com/sets/1/challenges/1)
  - [Challenge 2](./pkg/set1/chal2_test.go): [Fixed XOR](https://cryptopals.com/sets/1/challenges/2)
  - [Challenge 3](./pkg/set1/chal3_test.go): [Single-byte XOR cipher](https://cryptopals.com/sets/1/challenges/3)
  - [Challenge 4](./pkg/set1/chal4_test.go): [Detect single-character XOR cipher](https://cryptopals.com/sets/1/challenges/4)
  - [Challenge 5](./pkg/set1/chal5_test.go): [Implement repeating-key XOR](https://cryptopals.com/sets/1/challenges/5)
  - [Challenge 6](./pkg/set1/chal6_test.go): [Break repeating-key XOR](https://cryptopals.com/sets/1/challenges/6) there is a bug here!
  - [Challenge 7](./pkg/set1/chal7_test.go): [AES in ECB mode](https://cryptopals.com/sets/1/challenges/7)
  - [Challenge 8](./pkg/set1/chal8_test.go): [Detect AES in ECB mode](https://cryptopals.com/sets/1/challenges/8)
- [Set 2](./pkg/set2/): [Block crypto](https://cryptopals.com/sets/2)
  - [Challenge 9](./pkg/set2/chal9_test.go): [Implement PKCS#7 padding](https://cryptopals.com/sets/2/challenges/9)
  - [Challenge 10](./pkg/set2/chal10_test.go): [Implement CBC mode](https://cryptopals.com/sets/2/challenges/10)
  - [Challenge 11](./pkg/set2/chal11_test.go): [An ECB/CBC detection oracle](https://cryptopals.com/sets/2/challenges/11)
  - _more coming soon..._
