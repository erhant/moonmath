from extended_euclidean import xgcd

# Chinese Remainder Theorem
def crt(a: list[int], n: list[int]):
  assert(len(a) == len(n))

  # find N as multiplication of all n
  N = 1
  for n_i in n:
    N *= n_i

  # find x'  
  xp = 0
  for i in range(len(a)):
    Ndiv = N // n[i]
    _, s, t = xgcd(Ndiv, n[i])
    xp += a[i] * s * Ndiv
  
  # return x' mod N
  return xp % N

if __name__ == '__main__':
  print(crt([4, 1, 3, 0], [7, 3, 5, 11]))