from sage.all import Integers, factor

def example_40():
  Z5 = Integers(5)

  # get all elements
  # notice that we ignore 0, as this is a multiplicative group
  Z5_elems = []
  for e in range(1, Z5.order()):
    Z5_elems.append(Z5(e))
  Z5_elems = set(Z5_elems)
  print("Z5*   :", Z5_elems)

  # find largest prime factor
  order = Z5.order() - 1 # ignoring 0
  factorization = factor(order)
  lpf = max(factorization)[0] # largest prime factor
  cf = order // lpf # cofactor

  # clear cofactors
  Z5_2_elems = []
  for e in Z5_elems:
    Z5_2_elems.append(e ** cf)
  Z5_2_elems = set(Z5_2_elems)
  print("Z5*[2]:", Z5_2_elems)

def exercise_40():
  Z6 = Integers(6)
  order = Z6.order()

  # find subgroup orders, which are given by the factorization of the order
  factorization = list(map(lambda x : x[0], factor(order)))
  subgroupOrders = [1, order]
  subgroupOrders.extend(factorization)
  subgroupOrders = sorted(subgroupOrders)

  # do co-factor clearing to compute subgroups
  factorGroups = {}
  for subgroupOrder in subgroupOrders:
    cf = order // subgroupOrder # find cofactor
    Z6_f_elems = []
    for e in range(0, order):
      # notice that we use multiplication instead of exponent, 
      # because this is an additive group
      Z6_f_elems.append(Z6(e) * cf)
    Z6_f_elems = set(Z6_f_elems)
    print("Z6[{}]  :{}".format(subgroupOrder, Z6_f_elems))
    factorGroups[subgroupOrder] = Z6_f_elems
  
  lpf = max(factorization) # largest prime factor
  print("\nLargest Prime Order Subgroup:")
  print("Z6[{}]  :{}".format(lpf, factorGroups[lpf]))
  
def exercise_41():
  Zp = Integers(5)
  order = Zp.order()

  # clear co-factor for subgroup order 2
  so = 2
  cf = order // so

  sg_elems = []
  g_elems = []
  for e in range(1, order):
    g_elems.append(Zp(e))
    sg_elems.append(Zp(e) ** cf)
  sg_elems = set(sg_elems)
  g_elems = set(g_elems)

  print(g_elems)
  print(sg_elems)

  for g in sg_elems:
    powers = []
    for e in range(1, order):
      powers.append(g ** e)
    print("{}: {}".format(g, powers))


if __name__ == "__main__":
  exercise_41()
  
  