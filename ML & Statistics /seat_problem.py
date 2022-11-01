#!/usr/bin/env python3
from random import choice as rand_choice
from matplotlib import pyplot as plt


def irange(n):
    """inclusive range starting from 1"""
    return range(1, n + 1)


def seat(n, seats, seat_size):
    if n == 1:
        seat_random(n, seats, seat_size)
    elif n not in seats:
        seats[n] = n
    else:
        seat_random(n, seats, seat_size)


def seat_random(n, seats, seat_size):
    seats[rand_choice(get_available_seats(seats, seat_size))] = n


def get_available_seats(seats, seat_size):
    available_seats = list(irange(seat_size))
    for seat in seats.keys():
        available_seats.remove(seat)
    return available_seats


def seat_all(seat_size):
    # seat number => id
    seats = dict()
    for n in irange(seat_size):
        seat(n, seats, seat_size)
    return seats


def sample_over(n, seat_size, cmp):
    correct = 0
    last_seat_index = seat_size  # for readability
    for _ in range(n):
        seats = seat_all(seat_size)
        if cmp(last_seat_index, seats):
            correct += 1
    return correct, n - correct  # incorrect


def last_n_seated_correctly(n, last_seat_index, seats):
    return all(seats[last_seat_index - k] == last_seat_index - k for k in range(n))


def plot(probs):
    print(probs)
    plt.style.use("seaborn-deep")
    plt.ylabel("Probability (in 0-1 scale)")
    plt.xlabel("Last x passengers seated correctly")
    plt.title(f"In a sample of 10M with a seat count of 100")
    plt.xticks(irange(len(probs)))
    plt.yticks(list(n / 100 for n in range(0, 55, 5)))
    plt.grid()
    plt.plot(irange(len(probs)), probs)
    plt.show()


def main():
    n = 10_000_000
    seat_count = 100
    last = 100

    # probabilities of last {index + 1} passenger(s) being seated correctly
    probs = []
    for i in irange(last):
        correct, _ = sample_over(
            n,
            seat_count,
            lambda last_seat_index, seats: last_n_seated_correctly(
                i, last_seat_index, seats
            )
        )
        probs.append(correct / n)

    plot(probs, n, seat_count)


if __name__ == "__main__":
    main()
