# Fermat's Little Theorem

if __name__ == "__main__":
    p = random_prime(2 ^ 16)
    k = ZZ.random_element(p)
    
    result = ZZ(k) ^ ZZ(p) % ZZ(p)
    print(result == k)
