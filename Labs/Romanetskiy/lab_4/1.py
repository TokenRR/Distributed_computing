import random

# Генеруємо квадратну матрицю 5x5 з випадковими елементами від -100 до 100
matrix = [[random.randint(-100, 100) for _ in range(6)] for _ in range(5)]

# Виводимо матрицю
for row in matrix:
    print(row)
