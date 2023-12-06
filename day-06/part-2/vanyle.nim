from times import cpuTime
from os import paramStr

from math import sqrt, floor, ceil

proc isDigi(c: char): bool = c >= '0' and c <= '9'

proc parseInput2(s: string, idx: int): (int, int) = 
    var i = idx
    var n = 0
    while i < s.len and s[i] != '\n':
        if isDigi(s[i]):
            n *= 10
            n += cast[int](s[i]) - cast[int]('0')
        inc i
    return (n, i)

proc run(s: string): string =
    let (time, idx) = parseInput2(s, 0)
    let (distance, _) = parseInput2(s, idx+1)

    var delta = sqrt((time*time - 4 * distance).float)
    var x1 = ((time.float + delta) / 2).floor
    var x2 = ((time.float - delta) / 2).ceil
    var ways = (x1 - x2 + 1).int

    return $ways


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
