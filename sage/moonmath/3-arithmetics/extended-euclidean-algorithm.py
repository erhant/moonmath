
## Extended Euclidean Algorithm
## xgcd(a,b) = s * a + t * b
## return `gcd(a,b)` along with `s` and `t`
def xgcd(a: int, b: int):
  assert(a >= b)

  # initials
  r = [a, b]
  s = [1, 0]
  t = [0, 1]

  k = 2
  while r[k-1] != 0:
    quot = r[k-2] // r[k-1]
    rem = r[k-2] % r[k-1]

    r.append(rem)
    s.append(s[k-2] - quot * s[k-1])
    t.append(t[k-2] - quot * t[k-1])
    k += 1

  print("{0}|\t{1}\t{2}\t{3}".format("k", "r", "s", "t"))
  for i in range(k):
     print("{0}|\t{1}\t{2}\t{3}".format(i, r[i], s[i], t[i]))
     
  return r[k-2], s[k-2], t[k-2]

if __name__ == '__main__':
    print(xgcd(12, 5))