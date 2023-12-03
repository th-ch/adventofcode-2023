from times import cpuTime
from os import paramStr

proc isdig(c: char): bool {.inline.} =
    return c <= '9' and '0' <= c

proc run(s: string): string =
    var cv = 0
    var isFirst = true
    var lastVal = 0
    for c in s:
        if c == '\n':
            cv += lastVal
            isFirst = true
            lastVal = 0
            continue

        let d = isdig(c)
        var v = (cast[int](c) - cast[int]('0')) * cast[int](d)
        cv += (v * 10) * cast[int](isFirst and d)
        isFirst = isFirst and not d
        lastVal = lastVal * cast[int](not d) + v * cast[int](d)

    cv += lastVal

    return $cv


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
