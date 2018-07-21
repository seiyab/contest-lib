# maximum flow
from collections import defaultdict

def ford_fulkerson(graph, start, end):
    """ 
    graph: [dict<int,int>]
    start: int
    end: int
    """
    INF = 10**9
    network = [defaultdict(int, paths) for paths in graph]

    def dfs(start, end, visited):
        """
        None: unreachable
        (): reached
        """
        if start == end:
            return ()
        visited.add(start)
        for next_, capacity in filter(lambda next_capacity: next_capacity[0] not in visited and next_capacity[1] > 0, network[start].items()):
            path = dfs(next_, end, visited)
            if path is not None:
                return (next_, capacity, path)
        return None

    def margin(path, min_margin=INF):
        if path == ():
            return min_margin
        _, capacity, remaining_path = path
        return margin(remaining_path, min(min_margin, capacity))

    def flush(start, path, amount):
        if path == ():
            return
        next_, _, remaining_path = path
        network[start][next_] -= amount
        network[next_][start] += amount
        flush(next_, remaining_path, amount)

    while True:
        path = dfs(start, end, set())
        if path == None:
            break
        flush(start, path, margin(path))

    return sum(cap for _, cap in graph[start].items()) - sum(cap for _, cap in network[start].items())
