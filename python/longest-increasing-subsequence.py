def longest_increasing_subsequence(sequence):
    seq_list = list(sequence)
    dp = [(i, 1, -1) for i in range(len(seq_list))]
    for i, x in enumerate(seq_list):
        if i == 0:
            continue
        j, l, prev = max(dp[:i], key=lambda z: z[1])
        if seq_list[j] < x:
            dp[i] = (i, l+1, j)

    longest_end = max(dp, key=lambda z: z[1])[0]
    trace = [longest_end]
    while trace[-1] != -1:
        trace.append(dp[trace[-1]][2])
    return reversed(trace[:-1])
