from times import cpuTime
from os import paramStr

import strutils, tables, sets

proc parseInput(s: string): seq[string] = 
    return s.strip.split("\n")

proc isBounds(r: seq[string], p: (int, int)): bool =
    if p[0] < 0 or p[1] < 0 or p[0] >= r.len or p[1] >= r[0].len:
        return false
    return true

proc isValid(r: seq[string], p: (int, int), ff: (int, int), vtype: static[bool] = false): bool =
    if not r.isBounds(p):
        return false
    if r[p[0]][p[1]] == '#':
        return false
    if r[p[0]][p[1]] != '.':
        return true
    var ffc = r[ff[0]][ff[1]]
    if ffc == '.':
        return true

    if ffc == '>' and p[1] > ff[1]:
        return true # >.
    if ffc == 'v' and p[0] > ff[0]:
        return true

    return false # no climbing!

iterator nei(r: seq[string], p: (int, int), vtype: static[bool] = false): (int, int) =
    if r.isValid((p[0]+1, p[1]), p, vtype):
        yield (p[0]+1, p[1])
    if r.isValid((p[0]-1, p[1]), p, vtype):
        yield (p[0]-1, p[1])
    if r.isValid((p[0], p[1]+1), p, vtype):
        yield (p[0], p[1]+1)
    if r.isValid((p[0], p[1]-1), p, vtype):
        yield (p[0], p[1]-1)


type Graph[T: static[int]] = object
    # adjacency matrix with costs
    adj: array[T, array[4, (int16, int16)]]
    nodeMap: Table[(int, int), int16]
    start: int16
    goal: int16
    nodeCount: int16

proc addArr(ar: var array[4, (int16, int16)], e: (int16, int16)) =
    var i = 0
    while ar[i][0] != -1:
        if ar[i][0] == e[0]:
            ar[i][1] = max(ar[i][1], e[1])
            return
        inc i

    ar[i] = e

const vo = (-1.int16, -1.int16)

proc exploreCrossRoads(r: seq[string], p: (int, int), p2: (int, int), vtype: static[bool] = false): ((int, int), int16) =
    # Explore from p2, but ignoring p.
    var current = p2
    var previous = p # a cross-roads usually.
    var nc = 1
    var dist = 0.int16

    while nc == 1:
        nc = 0
        var next: (int, int)
        for i in r.nei(current, vtype):
            if i != previous:
                inc nc
                next = i

        inc dist
        previous = current
        current = next

    return (previous, dist)

proc recursiveExplorer(
            r: seq[string],
            seen: var HashSet[(int, int)],
            g: var Graph, p: (int, int),
            vtype: static[bool] = false
        ) =
    
    if p in seen: return
    seen.incl p

    var currentIdx = g.nodeMap[p]

    for i in r.nei(p, vtype):
        var (otherNode, d) = r.exploreCrossRoads(p, i, vtype)
        var otherNodeIndex = g.nodeCount
        if otherNode in g.nodeMap:
            otherNodeIndex = g.nodeMap[otherNode]
        else:
            g.nodeMap[otherNode] = otherNodeIndex
            g.adj[g.nodeCount] = [vo,vo,vo,vo]
            inc g.nodeCount
        if currentIdx != otherNodeIndex:
            g.adj[currentIdx].addArr (otherNodeIndex, d)
            if vtype:
                g.adj[otherNodeIndex].addArr (currentIdx, d)

        r.recursiveExplorer(seen, g, otherNode, vtype)



proc compress[T](r: seq[string], p: (int, int), goal: (int, int), res: var Graph[T], vtype: static[bool] = false) =
    var seen: HashSet[(int, int)]
    res.start = 0
    res.nodeMap[p] = 0
    res.goal = 1
    res.nodeMap[goal] = 1
    res.nodeCount = 2
    res.adj[0] = [vo,vo,vo,vo]
    res.adj[1] = [vo,vo,vo,vo]

    r.recursiveExplorer(seen, res, p, vtype)


var global1G: Graph[127]
proc findLongestPath(seen: var set[0..127], p: int, g: int): int16 =
    if p == g:
        return 0

    seen.incl p
    var cost: int16 = -1
    var noPath = true

    for (nodeId, dist) in global1G.adj[p]:
        if nodeId == -1: break
        noPath = false
        if nodeId notin seen:
            cost = max(cost, dist.int16 + findLongestPath(seen, nodeId, g))

    seen.excl p
    if noPath:
        return -1

    return cost

var global2G: Graph[63]
proc findLongestPath2(seen: var set[0..63], p: int, g: int): int16 =
    if p == g:
        return 0

    seen.incl p
    var cost: int16 = -1
    var noPath = true

    for (nodeId, dist) in global2G.adj[p]:
        if nodeId == -1: break
        noPath = false
        if nodeId notin seen:
            cost = max(cost, dist.int16 + findLongestPath2(seen, nodeId, g))

    seen.excl p
    if noPath:
        return -1

    return cost

proc run(s: string): string =
    var r = parseInput(s)

    var pos = (0, 1)
    var goal = (r.len-1, r.len-2)
    compress(r, pos, goal, global1G, false)

    var seen: set[0..127]
    var maxCost = findLongestPath(seen, global1G.start, global1G.goal)
    
    return $maxCost



var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
