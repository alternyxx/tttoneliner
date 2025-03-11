# a script to visualise the dataset
import os
import sys
import json
import math
import matplotlib.pyplot as plt
import numpy as np

def main() -> None:
    try:
        with open(os.path.join("src", "datasets", "dataset.json")) as file:
            dataset: dict = json.loads(file.read())
    except Exception as e:
        sys.exit(e)   

    ds: list[list] = [[], []]
    freq: list[int] = []
    gcds: int = 0
    for board, optimal_move in dataset.items():
        ds[0].append(int(board))
        ds[1].append(optimal_move)
        freq.append(int(board) * 11 % 9)
        if math.gcd(int(board), 9) > 1:
            gcds += 1
        print(int(board)%9)

    print(gcds)

    fig, ax = plt.subplots()
    ax.scatter(*ds)
    # ax.hist(freq)
    plt.show()


if __name__ == "__main__":
    main()