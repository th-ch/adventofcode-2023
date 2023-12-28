from times import cpuTime
from os import paramStr

import strutils

proc parseInput(s: string): seq[string] = 
    return s.strip.split("\n")

proc tiltNorth(s: var seq[string]) =
    for i in 1..<s.len:
        for j in 0..<s[i].len:
            var pi = i
            if s[pi][j] == 'O':
                while pi-1 >= 0 and s[pi-1][j] == '.':
                    s[pi-1][j] = 'O'
                    s[pi][j] = '.'
                    dec pi

proc run(s: string): string =
    var r = parseInput(s)
    r.tiltNorth()

    var res = 0
    for i in 0..<r.len:
        for j in 0..<r[i].len:
            if r[i][j] == 'O':
                res += r.len - i

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
