from times import cpuTime
from os import paramStr

from math import sqrt, floor, ceil

proc isDigi(c: char): bool {.inline.} = c >= '0' and c <= '9'

proc parseInput(s: string): array[8, int] = 
    var i = 12
    var r: array[8, int]
    var j = 0
    var n = 0
    while i < s.len:
        if s[i] == ' ' or s[i] == '\n':
            if n != 0:
                r[j] = n
                inc j
                n = 0
            if s[i] == '\n':
                i += 12
        elif isDigi(s[i]):
            n *= 10
            n += cast[int](s[i]) - cast[int]('0')
        inc i

    if n != 0:
        r[j] = n
    return r

proc run(s: string): string =
    var times = parseInput(s)
    
    var res = 1

    for i in 0..<4:
        var delta = sqrt((times[i]*times[i] - 4 * (times[i+4]+1)).float)
        var x1 = ((times[i].float + delta) / 2).floor
        var x2 = ((times[i].float - delta) / 2).ceil
        res *= (x1 - x2 + 1).int

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
