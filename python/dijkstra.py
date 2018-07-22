import heapq

def dijkstra(graph, start, end):
    q = []
    heapq.heapify(q)
    visited = set()

    for next_, cost in graph[start].items():
        heapq.heappush(q, (cost, next_))

    while len(q) > 0:
        node_cost, current = heapq.heappop(q)
        if current in visited:
            continue
        if current == end:
            return node_cost

        visited.add(current)
        for next_, path_cost in graph[current].items():
            heapq.heappush(q, (node_cost + path_cost, next_))

    return None
