from times import cpuTime
from os import paramStr

from math import sqrt, floor, ceil

proc isDigi(c: char): bool = c >= '0' and c <= '9'

proc parseInput2(s: string): (int, int) = 
    var i = 12
    var n1 = 0
    var n2 = 0
    while s[i] != '\n':
        if isDigi(s[i]):
            n1 *= 10
            n1 += cast[int](s[i]) - cast[int]('0')
        inc i
    i += 12
    while i < s.len:
        if isDigi(s[i]):
            n2 *= 10
            n2 += cast[int](s[i]) - cast[int]('0')
        inc i

    return (n1, n2)

proc run(s: string): string =
    let (time, distance) = parseInput2(s)

    var delta = sqrt((time*time - 4 * (distance+1)).float)
    var x1 = ((time.float + delta) / 2).floor
    var x2 = ((time.float - delta) / 2).ceil
    var ways = (x1 - x2 + 1).int

    return $ways


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
