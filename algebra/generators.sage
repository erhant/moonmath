from sage.all import Integers, factor, euler_phi

# we do cofactor clearing on each element, and see if they are mapped to their
# own subgroups; if not, it means they belong to the subgroup of order p, which
# is the group itself
#
# https://math.stackexchange.com/a/814898
def find_generators(p):
  Zp = Integers(p)

  # find largest prime factor
  order = p - 1
  factorization = factor(order)
  print("Factors of", order, "are", factorization)
  gens = []
  for e in range(2, p):
    e = Zp(e)
    is_gen = True
    for f in map(lambda f: f[0], factorization):
      if e ** (order // f) == 1:
        is_gen = False
        break
    if is_gen:
      gens.append(e)

  print(gens)
  assert(len(gens) == euler_phi(order))
  
def list_powers(p):
  Zp = Integers(p)

  for g in range(1, p):
    powers = []
    for e in range(0, p):
      powers.append(Zp(g) ** e)
    print("{}: {}".format(g, set(powers)))
    
if __name__ == "__main__":
  find_generators(13)
  # list_powers(23)