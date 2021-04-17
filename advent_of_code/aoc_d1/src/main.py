f = open("d1input.txt", "r").read()
nos = f.split('\n')
for i in range(0, len(nos)):
    no = int(i)
    sub = 2020-no
    try:
        if nos.index(str(sub))!=-1:
            print(no)
    except:
        continue
