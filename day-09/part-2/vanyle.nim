from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

proc isAllZero(v: seq[int]): bool =
    for i in v:
        if i != 0: return false
    return true

proc extrapolate2(v: seq[int]): int =
    var exStack: seq[seq[int]] = @[v]
    while (not isAllZero exStack[^1]):
        var p: seq[int]
        for i in 1..<exStack[^1].len:
            p.add(exStack[^1][i] - exStack[^1][i-1])

        exStack.add p

    var nextValue = 0
    for i in countdown(exStack.len - 1, 0):
        nextValue = exStack[i][0] - nextValue

    return nextValue

proc run(s: string): string =
    var res = 0
    for l in s.fastSplit('\n'):
        if l.len == 0: continue
        var i = ints(l)
        res += extrapolate2(i)

    return $res

var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
