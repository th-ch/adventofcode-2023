from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

type Graph = array[26 * 26 * 26, (int16, int16)]

proc cToI(c: char): int16 {.inline.} =
    return cast[int16](c) - cast[int16]('A')

proc nodeToIdx(n: array[3, char]): int16 {.inline.} =
    return cToI(n[0]) * 26 * 26 + cToI(n[1]) * 26 + cToI(n[2])

import math

proc run(s: string): string =
    var stepsLen = 0
    while s[stepsLen] != '\n':
        inc stepsLen

    var graph: Graph

    for i in s.toOpenArray(stepsLen+2, s.len-1).fastSplit('\n'):
        var start: array[3, char]
        var left: array[3, char]
        var right: array[3, char]
        
        var idx = 0
        for j in 0..2:
            start[j] = i[j]
        for j in 7..9:
            left[idx] = i[j]
            inc idx
        idx = 0
        for j in 12..14:
            right[idx] = i[j]
            inc idx

        graph[nodeToIdx(start)] = (nodeToIdx(left), nodeToIdx(right))


    var startingPoints: seq[int16] = newSeqOfCap[int16](26*3)

    for i in 0..<graph.len:
        if i mod 26 == cToI('A') and (graph[i][0] != 0 or graph[i][1] != 0):
            startingPoints.add(i.int16)

    var cycleData: seq[int] = newSeqOfCap[int](startingPoints.len) # length

    for pt in startingPoints:
        var pos = pt
        var posInStep = 0
        var step = 0
        while (pos mod 26) != cToI('Z'):
            let d = s[posInStep]
            if d == 'R':
                pos = graph[pos][1]
            else:
                pos = graph[pos][0]

            inc step
            inc posInStep
            if posInStep >= stepsLen:
                posInStep = 0
        cycleData.add(step)

    var prod = 1
    for c in cycleData:
        prod = lcm(prod, c)

    return $prod

var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
