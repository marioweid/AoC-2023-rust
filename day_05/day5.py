def get_input():
    with open('example.txt', 'r') as f:
        input = [line.strip() for line in f.readlines()]
    return input

def a():
    lines = get_input()
    seeds = [int(seed) for seed in lines[0][7:].split()]
    changed = [False] * len(seeds)
    for line in lines[1:]:
        if line:
            if not line.endswith(':'):
                map = [int(val) for val in line.split()]
                for i in range(len(seeds)):
                    if not changed[i] and map[1] <= seeds[i] < map[1] + map[2]:
                        seeds[i] = map[0] + seeds[i] - map[1]
                        changed[i] = True
            else:
                changed = [False] * len(seeds)

    return min(seeds)

def get_overlap(source, map, target):
    left = max(source[0], map[0])
    right = min(source[1], map[1])
    remaining = []
    if left <= right:
        if source[0] < left:
            remaining.append((source[0], left - 1))
        if source[1] > right:
            remaining.append((right + 1, source[1]))
        next = (left + target[0] - map[0], right + target[0] - map[0])
        return next, remaining
    return None, [source]
    
def b():
    lines = get_input()
    next_intervals = [int(seed) for seed in lines[0][7:].split()]
    next_intervals = [(next_intervals[i], next_intervals[i] + next_intervals[i + 1] - 1) for i in range(0, len(next_intervals), 2)]
    intervals = []
    for line in lines[1:]:
        if line:
            if not line.endswith(':'):
                map = [int(val) for val in line.split()]
                map = [(map[1], map[1] + map[2] - 1), (map[0], map[0] + map[2] - 1)]
                remaining_intervals = []
                while intervals:
                    interval = intervals.pop(0)
                    next_interval, remaining = get_overlap(interval, map[0], map[1])
                    if next_interval:
                        next_intervals.append(next_interval)
                    remaining_intervals.extend(remaining)
                intervals = remaining_intervals
            else:
                intervals.extend(next_intervals)
                next_intervals = []

    intervals.extend(next_intervals)
    return min([i[0] for i in intervals])
        
if __name__ == '__main__':
    print(a())
    print(b())