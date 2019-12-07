

# runtime: missing file -> Error
# missing error check
# 	-> runtime error
# error message is ValueError

# data is invalid -> ignored: ValueError
# 	-> runtime error

#undefined * is OK
# unhandled swtich case


# un-used variable turned out to be the problem

#def cvt(x):
#    r = 0
#    try:
#        r = int(x)
#    except ValueError:
#        print("INVALID VALUE:" + x)
#    return r

d = None

d.toUpperCase()

def x():
    print("OK")

from random import random
if random() > 0.9:
    x()
else:
    this_never_happens()

r = open("input.txt","r")
lines = r.read().split("\n")
s = 0
for x in lines:
    if x=='':
        continue
    s += int(x)


print("SUM:" , s)
r.close()
