
def modu(data, n):
    new = data[:]
    for i,d in enumerate(data):
        to = (i * n) % (len(data))
        print(i, d, to)
        new[to] = d
    print(new)
    return new
