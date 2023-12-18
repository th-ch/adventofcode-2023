from times import cpuTime
from os import paramStr

import parseutils

iterator arraySplit(s: openarray[char], c: char): auto =
    var i = 0
    var prev = 0
    while i < s.len:
        if s[i] == c:
            yield s.toOpenArray(prev, i - 1)
            prev = i + 1 
        inc i
    yield s.toOpenArray(prev, s.len)

proc movePos(pos: (int,int), dir: char, amount: int = 1): (int, int) =
    if dir == 'U':
        return (pos[0]-amount, pos[1])
    elif dir == 'D':
        return (pos[0]+amount, pos[1])
    elif dir == 'R':
        return (pos[0], pos[1]+amount)
    elif dir == 'L':
        return (pos[0], pos[1]-amount)

proc takeHex(s: openarray[char]): array[6,char] =
    var i = 0
    while s[i] != '(':
        inc i
    i += 2 # skip ( and #
    for j in 0..<6:
        result[j] = s[i+j]

proc run(s: string): string =
    var res = 0
    var boundary = 0
    var pos = (0,0)
    for l in s.arraySplit('\n'):
        var c = takeHex(l)
        var dirIndication = c[5]
        var dir = ' '
        var size = 0

        if dirIndication == '0':
            dir = 'R'
        elif dirIndication == '1':
            dir = 'D'
        elif dirIndication == '2':
            dir = 'L'
        elif dirIndication == '3':
            dir = 'U'
        discard parseHex(c.toOpenArray(0, 4), size)

        var newpos = movePos(pos, dir, size)
        boundary += size
    
        # Shoelace formula
        res += pos[1]*newpos[0] - newpos[1]*pos[0]
        pos = newpos

    var area = res div 2
    # pick's theorem, solve for "i", interior points.
    var corrected = area + boundary div 2 + 1

    return $corrected.int


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
