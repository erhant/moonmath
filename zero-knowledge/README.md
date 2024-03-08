# Chapter 8: Zero-Knowledge Protocols

## Exercise 114 🔴

> Implement the Baby-JubJub twisted Edwards curve equation in Circom and compile it into an R1CS and associated witness generator.

## Exercise 115 🔴

> Consider exercise [114](#exercise-114) and execute a 3-party trusted setup phase for the BabyJubJub circuit.

## Exercise 116 🔴

> Consider exercise [114](#exercise-114) and the CRS from exercise [115](#exercise-115). Use Sage to generate a valid curve point on the Baby-JubJub curve in twisted Edwards form, that is not the neutral element and use this point as input to the circuit. Generate a witness for this input and use SnarkJS to compute an instance as well as a zk-SNARK for this witness.

## Exercise 117 🔴

> Consider exercise [114](#exercise-114), the CRS from exercise [115](#exercise-115) as well as the instance and zk-SNARK from exercise [116](#exercise-116) and verify the zk-SNARK against the instance.
