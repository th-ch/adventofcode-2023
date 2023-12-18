from times import cpuTime
from os import paramStr

iterator arraySplit(s: openarray[char], c: char): auto =
    var i = 0
    var prev = 0
    while i < s.len:
        if s[i] == c:
            yield s.toOpenArray(prev, i - 1)
            prev = i + 1 
        inc i
    yield s.toOpenArray(prev, s.len)

proc extractSecond(s: openarray[char]): int =
    for i in 0..<s.len:
        var d = cast[int](s[i]) - cast[int]('0')
        if d >= 0 and d <= 9:
            result *= 10
            result += d
        if s[i] == ' ':
            break

proc movePos(pos: (int,int), dir: char, amount: int = 1): (int, int) =
    if dir == 'U':
        return (pos[0]-amount, pos[1])
    elif dir == 'D':
        return (pos[0]+amount, pos[1])
    elif dir == 'R':
        return (pos[0], pos[1]+amount)
    elif dir == 'L':
        return (pos[0], pos[1]-amount)

proc run(s: string): string =
    var res = 0
    var boundary = 0
    var pos = (0,0)
    
    for l in s.arraySplit('\n'):
        var dir = l[0]
        var size = extractSecond(l.toOpenArray(2, l.len-1))
        var newpos = movePos(pos, dir, size)
        boundary += size
    
        res += pos[1]*newpos[0] - newpos[1]*pos[0]
        pos = newpos

    var area = res div 2
    var corrected = area + boundary div 2 + 1

    return $corrected.int


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
