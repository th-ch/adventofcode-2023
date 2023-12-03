from times import cpuTime
from os import paramStr

import strutils

proc parseInput(s: string): seq[seq[int]] = 
    var c = s.strip.split("\n")
    var r: seq[seq[int]] = @[]
    for i in 0..<c.len:
        var t: seq[int] = @[]
        for j in 0..<c[i].len:
            if c[i][j].isDigit:
                t.add parseInt($c[i][j])
        r.add t
    return r

proc run(s: string): string =
    var r = parseInput(s)
    var cv = 0
    for l in r:
        if l.len == 0: continue
        cv += l[0]*10 + l[l.len - 1]

    return $cv


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
