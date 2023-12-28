from times import cpuTime
from os import paramStr

import strutils, sequtils, sets

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

proc whatIsS(beforeposS, posS, afterposS: (int, int), checked = false): char =
        let dx = abs(afterposS[1] - beforePosS[1])
        let dy = abs(afterposS[0] - beforePosS[0])
        if dx == 2: return '-'
        if dy == 2: return '|'

        let dxB = posS[1] - beforePosS[1]
        if dxB == 1:
            # bS S => 7 / J
            let dyA = posS[0] - afterPosS[0]
            if dyA == 1:
                #    aS
                # bS  S
                return 'J'
            elif dyA == -1:
                return '7'
        elif dxB == -1:
            # S bS => L / F
            let dyA = posS[0] - afterPosS[0]
            if dyA == 1:
                # aS
                # S  bS
                return 'L'
            elif dyA == -1:
                return 'F'

        if not checked:
            return whatIsS(afterposS, posS, beforePosS, true)
        return '?'

proc followCycle2(r: var seq[string], s, dir: (int,int)): seq[(int,int)] =
    var start = s
    var direction = dir

    result = @[s, dir]

    while true:        
        var next = r.nextPos(direction, start)
        
        if next == (-1, -1): return @[]
        if not r.bounds(next[0], next[1]): return @[]

        if r[next[0]][next[1]] == 'S':
            # replace 'S' with the correct thing.
            var charS = whatIsS((direction[0], direction[1]),(next[0], next[1]), (dir[0], dir[1]))
            r[next[0]][next[1]] = charS
            return result # back to the start!
        result.add next

        start = direction
        direction = next

proc connected(r: seq[string], a, b: (int, int)): bool =
    return r.nextPos(a, b) != (-1, -1) and r.nextPos(b, a) != (-1, -1)

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
    var cycle: seq[(int, int)]
    block cycleTests:
        sdir = (spos[0]+1, spos[1])
        cycle = r.followCycle2(spos, sdir)
        if cycle.len != 0:
            break cycleTests
        sdir = (spos[0]-1, spos[1])
        cycle = r.followCycle2(spos, sdir)
        if cycle.len != 0:
            break cycleTests
        sdir = (spos[0], spos[1]-1)
        cycle = r.followCycle2(spos, sdir)
        if cycle.len != 0:
            break cycleTests 
        sdir = (spos[0], spos[1]-1)
        cycle = r.followCycle2(spos, sdir)
        if cycle.len != 0:
            break cycleTests

    # We use the definition of "inside" as defined
    # by some mathematician:
    # we go through an odd number of borders!

    var ctop = cycle.foldl(min(a,b[0]), cycle[0][0])
    var cbottom = cycle.foldl(max(a,b[0]), cycle[0][0])

    var cycleSet = cycle.toHashSet
    var insideCounter = 0

    for line in ctop..cbottom:
        var isInside = false
        
        # line polarity: u ^ / 4 s
        var linePolarity = false # false = top
        var mustSwitch = false

        for i in 0..<r[0].len:
            if (line, i) in cycleSet:
                if r[line][i] == '|':
                    isInside = not isInside
                    continue # polarity switch, obvious.
                # check if we are connected to the previous part.
                if (line, i-1) notin cycleSet or not r.connected((line, i), (line, i-1)):
                        # we are not connected to previous, compute
                        # for line polarity.
                        if r[line][i] == 'F':
                            linePolarity = true # bottom
                        if r[line][i] == 'L':
                            linePolarity = false # top                    

                if (line, i+1) in cycleSet:
                    if r.connected((line, i+1), (line, i)):
                        # a difference in polarity means we must switch.
                        if r[line][i + 1] == 'J':
                            mustSwitch = (linePolarity == true) # top
                        if r[line][i + 1] == '7':
                            mustSwitch = (linePolarity == false) # bottom   

                if mustSwitch:
                    isInside = not isInside
                    mustSwitch = false

            elif isInside:
                inc insideCounter

    return $insideCounter


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
