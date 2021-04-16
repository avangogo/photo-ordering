import sys, itertools, math

def main():
    # Read file
    filename = sys.argv[1]
    with open(filename, 'r') as f:
        n, m, k = (int(num) for num in f.readline().split())
        graph = { i:[] for i in range(1, n+1) }
        for i in range(k):
            u, v = (int(num) for num in f.readline().split())
            graph[u].append(v)

    # Output
    if m <= 0:
        raise Exception("Bad value for m ({})".format(m))
    if is_acyclic(graph):
        print(min_page_feasible(graph, m))
    else:
        print("Impossible")

# Compute the minimal number of pages needed given constraint graph
def min_page_feasible(graph: dict, max_by_page: int) -> int:
    n_photos = len(graph)
    if n_photos == 0:
        return 0
    if max_by_page == 1:
        return n_photos
    # Case 1: Remove photos without dependency
    photos_no_dependency = isolated_vertices(graph)
    if photos_no_dependency:
        for photo in photos_no_dependency:
            del graph[photo]
        return max(min_page_feasible(graph, max_by_page), int(math.ceil(n_photos/max_by_page)))
    # Get available photos
    photos_ready = roots(graph)
    # Case 2: all of them fits on the next page 
    if len(photos_ready) <= max_by_page:
        for photo in photos_ready:
            del graph[photo]
        return 1 + min_page_feasible(graph, max_by_page)
    # Case 3: Bruteforce on all combination for the next page
    else:
        result = n_photos
        for page in itertools.combinations(photos_ready, max_by_page):
            subgraph = graph.copy()
            for photo in page:
                del subgraph[photo]
            result = min(result, 1 + min_page_feasible(subgraph, max_by_page))
        return result

# Return the vertices with no ingoing edges
def roots(graph: dict):
    result = set(graph.keys())
    for neighbourhood in graph.values():
        for v in neighbourhood:
            if v in result:
                result.remove(v)
    return result

# Return the vertices adjacent to no edge
def isolated_vertices(graph: dict):
    return [ v for v in roots(graph) if len(graph[v]) == 0 ]  

# Return True if the graph has no directed cycle
def is_acyclic(graph: dict):
    g = graph.copy()
    while len(g) != 0:
        g_roots = roots(g)
        if len(g_roots) == 0: return False
        for v in g_roots:
            del g[v]
    return True


if __name__ == "__main__":
    main()
