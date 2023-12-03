from times import cpuTime
from os import paramStr

import strutils

var sdigits = @["zero","one","two","three","four","five","six","seven","eight","nine"]

proc parseInput2(s: string): seq[seq[int]] =
    var c = s.strip.split("\n")
    var r: seq[seq[int]] = @[]
    for i in 0..<c.len:
        var t: seq[int] = @[]
        var j = 0
        while j < c[i].len:
            if c[i][j].isDigit:
                t.add parseInt($c[i][j])
            else:
                for k in 0..<sdigits.len:
                    var dd = sdigits[k]
                    if c[i].len >= (j+dd.len):
                        if c[i][j..<(j+dd.len)] == dd:
                            t.add k
                            # j += dd.len - 1
                            break
            inc j

        r.add t
    return r

proc run(s: string): string =
    # Your code here
    var r = parseInput2(s)
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
