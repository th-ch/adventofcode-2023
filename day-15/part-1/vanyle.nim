from times import cpuTime
from os import paramStr

proc run(s: string): string =
    var res = 0
    var h = 0
    for c in s:
        if c == ',':
            res += h
            h = 0
        else:
            var ascii = cast[int](c)
            h += ascii
            h = h * 17
            h = h mod 256
    res += h
    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
