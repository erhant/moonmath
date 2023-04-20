

if __name__ == "__main__":
  # find a large prime
  p = random_prime(2 ^ 256)

  # sample any integer less than `p`
  n = ZZ.random_element(p)

  # their GCD should be 1
  print(gcd(p, n))