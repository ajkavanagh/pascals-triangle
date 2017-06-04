# Code to generate pascals triangle

import functools
import gc
import timeit


class PascalsTriangle1:

    def __init__(self, rows):
        self.num_rows = rows

    def rows(self):
        rs = []
        if self.num_rows == 0:
            return rs
        rs.append([1])
        if self.num_rows == 1:
            return rs
        current_row = [1]
        for _ in range(1, self.num_rows):
            current_row = make_next_row(current_row)
            rs.append(current_row)
        return rs

def make_next_row(current_row):
    return [a + b for a, b in zip([0] + current_row,
                                  current_row + [0])]


class PascalsTriangle2:

    def __init__(self, rows):
        self.num_rows = rows

    def rows(self):
        if self.num_rows == 0:
            return []
        if self.num_rows == 1:
            return [[1]]

        return functools.reduce(
            lambda a, _: a + [make_next_row(a[-1])],
            range(1, self.num_rows),
            [[1]])

def make_next_row(current_row):
    offset_row = current_row[:]
    offset_row.append(0)
    return [a + b for a, b in zip([0] + current_row,
                                  current_row + [0])]


if __name__ == '__main__':
    total_time1 = timeit.Timer(
        'p = PascalsTriangle1(10); p.rows()',
        'gc.enable()',
        globals=globals()).timeit(1000000)
    print("PascalsTriangle1:", total_time1)
    total_time2 = timeit.Timer(
        'p = PascalsTriangle2(10); p.rows()',
        'gc.enable()',
        globals=globals()).timeit(1000000)
    print("PascalsTriangle2:", total_time2)
