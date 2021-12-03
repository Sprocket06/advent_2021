inputFile = open('input.txt').read().split('\n')

gamma = [0]*12

for line in inputFile:
    if len(line) == 0:
        break
    for i in range(0,12):
        if int(line[i]):
            gamma[i] += 1
        else:
            gamma[i] -= 1

gamma = list(map(lambda i : "1" if i>0 else "0" , gamma))
epsilon = list(map(lambda i : "0" if i == "1" else "1", gamma))
gamma = int("".join(gamma), 2)
epsilon = int("".join(epsilon), 2)

print(gamma * epsilon)
