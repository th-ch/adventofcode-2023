from times import cpuTime
from os import paramStr

const sdigits = ["zero","one","two","three","four","five","six","seven","eight","nine"]

proc scmp(bigs: string, smalls: string, idx: int): bool {.inline.} =
    if bigs.len < smalls.len + idx: return false
    for i in 0..<smalls.len:
        if bigs[i + idx] != smalls[i]: return false
    return true

proc isdig(c: char): bool {.inline.} =
    return c <= '9' and '0' <= c

proc parseVal(s: string, i: int): int =
    if isDig s[i]: return cast[int](s[i]) - cast[int]('0')
    for k in 0..<sdigits.len:
        if scmp(s, sdigits[k], i):
            return k
    return -1

proc run(s: string): string =
    var cv = 0
    var lastVal = 0
    var isFirst = true

    for i in 0..<s.len:
        if s[i] == '\n': 
            cv += lastVal
            lastVal = 0
            isFirst = true
        else:
            let v = parseVal(s, i)
            if v != -1:
                if isFirst:
                    cv += v * 10
                    isFirst = false
                lastVal = v
    
    cv += lastVal
    return $cv


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
