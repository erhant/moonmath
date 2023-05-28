from sage.all import random_prime, ZZ

# deomnstrating Fermat's Little Theorem
if __name__ == "__main__":
  p = random_prime(2 ^ 16)
  k = ZZ.random_element(p)
  
  # k ^ p mod p == k
  result = ZZ(k) ^ ZZ(p) % ZZ(p)
  print(result == k)
