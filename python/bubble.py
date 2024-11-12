import array
import numpy as np

n = 10
numbers = array.array('i', range(n))
np.random.shuffle(numbers)
print(numbers)
for i in range(n):
    # print(f"i: {i}")
    for j in range(n - 1 - i):
        # print(f"j: {j}")
        if numbers[j] > numbers[j+1]:

            # swap them
            temp = numbers[j]
            # print(f"{temp}")
            numbers[j] = numbers[j+1]
            # print(f"{temp}")
            numbers[j+1] = temp

print(numbers)
