
def modu(data, n):
    if (len(data) % n) % 2  != 1:
        print("bad")
    new = data[:]
    for i,d in enumerate(data):
        to = (i * n) % (len(data))
        new[to] = d
    print(new)
    for i in data:
        if i not in new:
            print("bad really")
            break
    return new
