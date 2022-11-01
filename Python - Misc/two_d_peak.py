from matplotlib import pyplot as plt
from random import randint
from time import time


def timeit(f):
    start = time()
    ret = f()
    end = time()
    print("Time taken:", round(end-start, 6))
    return ret


def flatten(arr):
    m = []
    for sub in arr:
        for i in sub:
            m.append(i)
    return m


def rand_arr():
    return [randint(0, 100), randint(0, 100), randint(0, 100)]


def rand_mat(len):
    if len == 1_000_000_000:
        return rand_mat(1_000_000) * 1000
    if len == 100_000_000:
        return rand_mat(1_000_000) * 100
    if len == 10_000_000:
        return rand_mat(1_000_000) * 10
    if len == 1_000_000:
        return rand_mat(100_000) * 10
    mat = []
    for _ in range(len):
        mat.append(rand_arr())
    return mat


def peak_element_idx(arr, start, end):
    if start == end:
        return start
    while start < end:
        mid = start + (end - start) // 2
        if arr[mid] < arr[mid + 1]:
            return peak_element_idx(arr, mid + 1, end)
        elif arr[mid] < arr[mid - 1]:
            return peak_element_idx(arr, start, mid - 1)
        return mid


def peak_element(arr):
    return arr[peak_element_idx(arr, 0, len(arr) - 1)]


def two_d_peak_element_idx(arr):
    peaks = []
    for inner in arr:
        peaks.append(peak_element(inner))
    peak = peak_element(peaks)
    row_idx = peaks.index(peak)
    return (row_idx, arr[row_idx].index(peak))


def two_d_peak_element(arr):
    idxs = two_d_peak_element_idx(arr)
    return arr[idxs[0]][idxs[1]]


def greedy_ascent(mat):
    if len(mat) == 1:
        return max(mat[0])
    n = len(mat)
    j = (n - 1) // 2
    mid_max = max(mat[j])
    mid_max_index = mat[j].index(mid_max)
    if mat[j + 1][mid_max_index] > mid_max:
        return greedy_ascent(mat[j+1:])
    elif mat[j - 1][mid_max_index] > mid_max:
        return greedy_ascent(mat[:j])
    return mid_max


arr = [[0, 3], [2, 1]]
# arr = rand_mat(10_000_000)
ans_1 = timeit(lambda: two_d_peak_element(arr))
ans_2 = timeit(lambda: greedy_ascent(arr))
print("Answer:", ans_1)
print("Answer:", ans_2)
# plt.plot(flatten(arr), marker="o")
# plt.axhline(ans_1, color="red")
# plt.axhline(ans_2, color="red")
