# Shamir Secret Sharing

In this implementation, $k$ represents the minimum number of shares required to reconstruct the secret, $n$ represents the total number of shares to generate, and `secret` is the secret to be shared. The `coeffs` slice contains the coefficients of the polynomial used to generate the shares.

The `evaluatePolynomial` function evaluates the polynomial with the given coefficients at the given $x$ value using Horner's method.

The `interpolatePolynomial` function uses Lagrange interpolation to interpolate the polynomial from the given shares. The result is taken modulo 256 to ensure that it fits within a single byte.

To generate the shares, a loop is used to iterate over the values of $x$ from 1 to $n$, and the corresponding $y$ values are calculated using the `evaluatePolynomial` function.

To recover the secret, a subset of $k$ shares is selected and passed to the `interpolatePolynomial` function. The resulting value is the recovered secret.
