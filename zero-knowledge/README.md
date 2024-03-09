# Chapter 8: Zero-Knowledge Protocols

For this section, you need Circom and SnarkJS, see [chapter 2](../software-used/) for installations. I'm using SnarkJS v0.7.3 and Circom v2.1.8. Also use this directory as your active directory when working with the solution code below.

## Exercise 114

> Implement the Baby-JubJub Twisted Edwards curve equation in Circom and compile it into an R1CS and associated witness generator.

There is an implementation of [BabyJubJub](https://eips.ethereum.org/EIPS/eip-2494) circuits online at [iden3/circomlib](https://github.com/iden3/circomlib/blob/master/circuits/babyjub.circom). In fact, the linked file containts these circuits:

- `BabyCheck`: asserts that a given point is on curve.
- `BabyAdd`: adds two affine points via the Twisted Edwards curve.
- `BabyDbl`: adds a point to itself, calling `BabyAdd` with the same point as both inputs.

The curve-checking circuit is defined as follows:

```cs
template BabyJubJub() {
  signal input x;
  signal input y;

  var a = 168700;
  var d = 168696;

  signal x2 <== x*x;
  signal y2 <== y*y;

  a*x2 + y2 === 1 + d*x2*y2;
}
```

We write our circuit within the [babyjubjub.circom](./babyjubjub.circom) file. To compile, first we shall recall that BabyJubJub base field is that of prime order 21888242871839275222246405745257275088548364400416034343698204186575808495617, which is the default prime in Circom (or optionally specified as `-p bn128` argument). We compile with the following:

```sh
circom ./babyjubjub.circom --wasm --r1cs --sym
```

With the `--r1cs` option Circom outputs the R1CS, and with `--wasm` option it outputs a witness calculator in WASM and JS. These will be used in the upcoming exercises. We also added the `--sym` option to export symbol information, which can be used to analyze and debug our circuits and its constraints. Let's look at the output:

```sh
template instances: 1
non-linear constraints: 3
linear constraints: 0
public inputs: 0
private inputs: 2
public outputs: 0
wires: 5
labels: 5
Written successfully: ./babyjubjub.r1cs
Written successfully: ./babyjubjub.sym
Written successfully: ./babyjubjub_js/babyjubjub.wasm
Everything went okay
```

The important part here is our constraint count, which is really small at just 3 constraints. We can actually see the constraints with the following command:

```sh
snarkjs r1cs print ./babyjubjub.r1cs ./babyjubjub.sym
```

It prints three lines with huge numbers, those numbers are actually just negatives of small numbers. Below I write the more readable version:

```sh
[ (-1) * main.x ] * [ main.x ] - [ (-1) * main.x2 ] = 0
[ (-1) * main.y ] * [ main.y ] - [ (-1) * main.y2 ] = 0
[ (-168696) * main.x2 ] * [ main.y2 ] - [ 1 + (-168700) * main.x2 + (-1) * main.y2 ] = 0
```

Notice that unlike $Ax \odot Bx = Cx$ we have $Ax \odot Bx - Cx = 0$ here. Let's write these constraints mathematically as well:

$$
\begin{align*}
  (-1 \cdot x) \cdot (x) &= -1 \cdot x_2 \\
  (-1 \cdot y) \cdot (y) &= -1 \cdot y_2 \\
  (-168696 \cdot x_2) \cdot (y_2) &= 1  + (-168700) \cdot x_2 + (-1) \cdot y_2 \\
\end{align*}
$$

## Exercise 115

> Consider exercise [114](#exercise-114) and execute a 3-party trusted setup phase for the BabyJubJub circuit.

First, we start a ceremony for the BN128 field with at most $4 = 2^2$ constraints:

```sh
snarkjs powersoftau new bn128 2 ./ptau/babyjubjub_0.ptau -v
```

We contribute randomness three times:

```sh
snarkjs powersoftau contribute ./ptau/babyjubjub_0.ptau ./ptau/babyjubjub_1.ptau -v
snarkjs powersoftau contribute ./ptau/babyjubjub_1.ptau ./ptau/babyjubjub_2.ptau -v
snarkjs powersoftau contribute ./ptau/babyjubjub_2.ptau ./ptau/babyjubjub_3.ptau -v
snarkjs powersoftau verify ./ptau/babyjubjub_3.ptau
```

Let's make the final contribution with a random beacon:

```sh
snarkjs powersoftau beacon ./ptau/babyjubjub_3.ptau ./ptau/babyjubjub_beacon.ptau 112233445566778899aabbccdd 10 -n="Final Beacon"
```

Now, we can begin our Phase-2 setup:

```sh
snarkjs powersoftau prepare phase2 ./ptau/babyjubjub_beacon.ptau babyjubjub_final.ptau -v
snarkjs powersoftau verify babyjubjub_final.ptau
snarkjs groth16 setup babyjubjub.r1cs babyjubjub_final.ptau ./zkey/babyjubjub_0.zkey
```

Let's make some contributions to this as well:

```sh
snarkjs zkey contribute ./zkey/babyjubjub_0.zkey ./zkey/babyjubjub_1.zkey -v
snarkjs zkey contribute ./zkey/babyjubjub_1.zkey ./zkey/babyjubjub_2.zkey -v
snarkjs zkey contribute ./zkey/babyjubjub_2.zkey ./zkey/babyjubjub_3.zkey -v
snarkjs zkey verify babyjubjub.r1cs babyjubjub_final.ptau ./zkey/babyjubjub_3.zkey
```

Now let's finish this with one final randomness beacon:

```sh
snarkjs zkey beacon ./zkey/babyjubjub_3.zkey babyjubjub_final.zkey 112233445566778899aabbccdd 10 -n="Final Beacon Phase2"
snarkjs zkey verify babyjubjub.r1cs babyjubjub_final.ptau babyjubjub_final.zkey
```

This `babyjubjub_final.zkey` is our proving key! We can generate a verification key from it:

```sh
snarkjs zkey export verificationkey babyjubjub_final.zkey babyjubjub_vkey.json
```

This concludes our setup phase, and we are now ready to create proofs and verify them. To sum up:

- `babyjubjub_final.ptau` is our phase-1 setup result.
- `babyjubjub_final.zkey` is our phase-2 setup result, and is our proving key.
- `babyjubjub_vkey.json` is our verification key.

## Exercise 116

> Consider exercise [114](#exercise-114) and the CRS from exercise [115](#exercise-115). Use Sage to generate a valid curve point on the Baby-JubJub curve in twisted Edwards form, that is not the neutral element and use this point as input to the circuit. Generate a witness for this input and use SnarkJS to compute an instance as well as a zk-SNARK for this witness.

To generate a point on BabyJubJub curve, we can generate a random $x$ and see if there is a suitable $y$. First, recall the equation for BabyJubJub:

$$
168700x^2 + y^2 = 1 + 168696x^2y^2
$$

With some slight re-arrangement, we can see that:

$$
y^2 = \frac{168700x^2 - 1}{168696x^2 - 1}
$$

So we can try for random $x$ and see if $y^2$ is a quadratic residue. If it is, we can find its square root. Below is the Sage code for this:

```py
# Order of bn254
p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
F = GF(p)

# Curve parameters
a = F(168700)
d = F(168696)

while True:
    x = F.random_element()

    yy = (a * x * x - F(1)) / (d * x * x - F(1))
    if yy.is_square():
        y = yy.sqrt()
        print("(x, y) = ({}, {})".format(x, y))
        break
```

Note that the runtime of this method is not scary at all, approximately half of the elements in a prime field have a square! To get an intuition on why this is true, pick a random element $r \in \mathbb{F}_p$ for prime $p$ and then observe $r^2 = (p - r)^2$. Basically, two elements map into a single element with the square operation.

When we run the code above, we find:

```sh
(x, y) = (13448550110720579527479695514602263661503077373315668541500566284897011814382, 9477578444658309752558281146916876883413894292121685200786367130619200566088)
```

Our input is a JSON file, and JSON does not support bigints natively, but we can pass them as a string anyways. We prepare the input as:

```json
{
  "x": "13448550110720579527479695514602263661503077373315668541500566284897011814382",
  "y": "9477578444658309752558281146916876883413894292121685200786367130619200566088"
}
```

To compute a witness, we use the WASM and JS code generated above:

```sh
node ./babyjubjub_js/generate_witness.js ./babyjubjub_js/babyjubjub.wasm input.json witness.wtns
```

Our witness is generated at `witness.wtns`, and it holds the assigned values for each signal (as they appear on the symbol file `babyjubjub.sym`). We can now create a proof:

```sh
snarkjs groth16 prove babyjubjub_final.zkey witness.wtns proof.json public.json
```

If you look at `proof.json`, you will see some projective curve coordinates! Also, `public.json` is empty in this example because we don't have any public inputs or outputs.

## Exercise 117

> Consider exercise [114](#exercise-114), the CRS from exercise [115](#exercise-115) as well as the instance and zk-SNARK from exercise [116](#exercise-116) and verify the zk-SNARK against the instance.

For verification, we use our proof `proof.json` and public signals `public.json` and pass them to the verify function with the verification key:

```sh
snarkjs groth16 verify babyjubjub_vkey.json public.json proof.json
```

For some reason Groth16 proof verification failed when there were no public signals, so I made the inputs public. See relevant issue [here](https://github.com/iden3/snarkjs/issues/447).
