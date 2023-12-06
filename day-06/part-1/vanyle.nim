from times import cpuTime
from os import paramStr

from math import sqrt, floor, ceil

proc isDigi(c: char): bool = c >= '0' and c <= '9'

proc parseInput(s: string, idx: int): (seq[int], int) = 
    var r: seq[int] = @[]
    var i = idx
    var n = 0
    while i < s.len and s[i] != '\n':
        if isDigi(s[i]):
            n *= 10
            n += cast[int](s[i]) - cast[int]('0')
        elif n != 0:
            r.add n
            n = 0
        inc i
    if n != 0:
        r.add n
    return (r, i)

proc run(s: string): string =
    var (times, idx) = parseInput(s, 0)
    var (distances, _) = parseInput(s, idx+1)
    
    var res = 1

    for i in 0..<times.len:
        var delta = sqrt((times[i]*times[i] - 4 * distances[i]).float)
        var x1 = ((times[i].float + delta) / 2).floor
        var x2 = ((times[i].float - delta) / 2).ceil
        res *= (x1 - x2 + 1).int

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
