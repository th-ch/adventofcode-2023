from times import cpuTime
from os import paramStr

import strutils

proc parseInput(s: string): seq[string] = 
    return s.split("\n")

proc bounds(r: seq[string], y,x: int): bool =
    if y < 0 or y >= r.len or x < 0 or x >= r[y].len:
        return false
    return true

proc nextPos(r: seq[string], at, prev: (int, int)): (int, int) =
    if not r.bounds(at[0], at[1]):
        return (-1, -1)
    if not r.bounds(prev[0], prev[1]):
        return (-1, -1)

    let c = r[at[0]][at[1]]
    if c == 'S': return (-1,-1) # not what this function is designed to do.
    if c == '.': return (-1,-1)

    if c == '|':
        if at[0]+1 == prev[0] and at[1] == prev[1]:
            return (at[0]-1, at[1])
        if at[0]-1 == prev[0] and at[1] == prev[1]:
            return (at[0]+1, at[1])
        return (-1,-1)
    if c == '-':
        if at[0] == prev[0] and at[1]+1 == prev[1]:
            return (at[0], at[1]-1)
        if at[0] == prev[0] and at[1]-1 == prev[1]:
            return (at[0], at[1]+1)
        return (-1,-1)

    if c == 'L':
        # right to top
        if at[0] == prev[0] and at[1]+1 == prev[1]:
            return (at[0]-1, at[1])
        # top to right
        if at[0]-1 == prev[0] and at[1] == prev[1]:
            return (at[0], at[1]+1)
        return (-1,-1)

    if c == 'J':
        # left to top
        if at[0] == prev[0] and at[1]-1 == prev[1]:
            return (at[0]-1, at[1])
        if at[0]-1 == prev[0] and at[1] == prev[1]:
            return (at[0], at[1]-1)
        return (-1,-1)

    if c == '7':
        # left to bottom
        if at[0] == prev[0] and at[1]-1 == prev[1]:
            return (at[0]+1, at[1])
        if at[0]+1 == prev[0] and at[1] == prev[1]:
            return (at[0], at[1]-1)
        return (-1,-1)

    if c == 'F':
        # right to bottom
        if at[0] == prev[0] and at[1]+1 == prev[1]:
            return (at[0]+1, at[1])
        if at[0]+1 == prev[0] and at[1] == prev[1]:
            return (at[0], at[1]+1)
        return (-1,-1) 

    return (-1, -1)

proc connected(r: seq[string], a, b: (int, int)): bool =
    return r.nextPos(a, b) != (-1, -1) and r.nextPos(b, a) != (-1, -1)

proc followCycle(r: seq[string], s, dir: (int,int)): int =
    var start = s
    var direction = dir
    var cLength = 0

    while true:        
        var next = r.nextPos(direction, start)
        inc cLength
        if next == (-1, -1): return -1 # not a cycle, probably.
        if not r.bounds(next[0], next[1]): return -1


        if r[next[0]][next[1]] == 'S':
            return cLength # back to the start!

        start = direction
        direction = next

proc run(s: string): string =
    var r = parseInput(s)

    var spos = (-1,-1)

    block seekStart:
        for i in 0..<r.len:
            for j in 0..<r[i].len:
                if r[i][j] == 'S':
                    spos = (i,j)
                    break seekStart

    # seek a loop
    var sdir = spos
    var res = -1
    block cycleTests:
        sdir = (spos[0]+1, spos[1])
        res = r.followCycle(spos, sdir)
        if res != -1:
            break cycleTests
        sdir = (spos[0]-1, spos[1])
        res = r.followCycle(spos, sdir)
        if res != -1:
            break cycleTests
        sdir = (spos[0], spos[1]-1)
        res = r.followCycle(spos, sdir)
        if res != -1:
            break cycleTests 
        sdir = (spos[0], spos[1]-1)
        res = r.followCycle(spos, sdir)
        if res != -1:
            break cycleTests

    return $(res div 2 + 1)


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
