

def modu(data, n):
    new = data[:]

    for i,d in enumerate(data):
        to = (i * n) % (len(data))
        #print(f"writing val {i} to pos {to}")
        new[to] = d

    for i in data:
        if i not in new:
            print("bad really")
            break
    return new

