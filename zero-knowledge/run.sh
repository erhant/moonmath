#!/bin/bash

# Print versions
echo "Using:"
circom --version
snarkjs --help | head -n 1

# BabyJubJub Circuit Compilation
echo "Compiling BabyJubJub circuit..."
circom ./babyjubjub.circom --wasm --r1cs --sym
echo "Compilation done."

# Start of Trusted Setup Ceremony
echo "Starting trusted setup ceremony..."
echo "> Phase 1"
snarkjs powersoftau new bn128 3 ./ptau/babyjubjub_0.ptau -v
snarkjs powersoftau contribute ./ptau/babyjubjub_0.ptau ./ptau/babyjubjub_1.ptau -v
snarkjs powersoftau contribute ./ptau/babyjubjub_1.ptau ./ptau/babyjubjub_2.ptau -v
snarkjs powersoftau contribute ./ptau/babyjubjub_2.ptau ./ptau/babyjubjub_3.ptau -v
snarkjs powersoftau beacon ./ptau/babyjubjub_3.ptau ./ptau/babyjubjub_beacon.ptau 112233445566778899aabbccdd 10 -n="Final Beacon"

echo "> Phase 2"
snarkjs powersoftau prepare phase2 ./ptau/babyjubjub_beacon.ptau babyjubjub_final.ptau -v
snarkjs groth16 setup babyjubjub.r1cs babyjubjub_final.ptau ./zkey/babyjubjub_0.zkey
snarkjs zkey contribute ./zkey/babyjubjub_0.zkey ./zkey/babyjubjub_1.zkey -v
snarkjs zkey contribute ./zkey/babyjubjub_1.zkey ./zkey/babyjubjub_2.zkey -v
snarkjs zkey contribute ./zkey/babyjubjub_2.zkey ./zkey/babyjubjub_3.zkey -v
snarkjs zkey beacon ./zkey/babyjubjub_3.zkey babyjubjub_final.zkey 112233445566778899aabbccdd 10 -n="Final Beacon Phase2"
snarkjs zkey export verificationkey babyjubjub_final.zkey babyjubjub_vkey.json
echo "Trusted setup ceremony completed."

# Prepare input
x="13448550110720579527479695514602263661503077373315668541500566284897011814382"
y="9477578444658309752558281146916876883413894292121685200786367130619200566088"
echo "{\"x\":\"$x\", \"y\":\"$y\"}" > input.json

# Generate the witness
echo "Generating the witness..."
node ./babyjubjub_js/generate_witness.js ./babyjubjub_js/babyjubjub.wasm input.json witness.wtns

# Create a zk-SNARK proof
echo "Creating zk-SNARK proof..."
snarkjs groth16 prove babyjubjub_final.zkey witness.wtns proof.json public.json

# Verify the zk-SNARK
echo "Verifying zk-SNARK..."
snarkjs groth16 verify babyjubjub_vkey.json public.json proof.json
