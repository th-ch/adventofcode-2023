from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

proc run(s: string): string =
    var llen = 0
    while s[llen] != '\n':
        inc llen
    
    var galaxyPos: seq[(int, int)] = newSeqOfCap[(int, int)](200)
    var emptyCol = newSeq[int](llen+1)
    var emptyLine = @[0]

    var i = 0
    for ll in s.fastSplit('\n'):
        var galaxyFound = false
        for j in 0..<ll.len:
            if ll[j] == '#':
                galaxyPos.add((i,j))
                galaxyFound = true
                if emptyCol[j+1] == 0:
                    emptyCol[j+1] = 1

        if not galaxyFound:
            emptyLine.add(emptyLine[emptyLine.len-1] + 1)
        else:
            emptyLine.add(emptyLine[emptyLine.len-1])

        inc i

    # Integrate
    for i in 1..<emptyCol.len:
        if emptyCol[i] == 0:
            emptyCol[i] = emptyCol[i-1] + 1
        else:
            emptyCol[i] = emptyCol[i-1]

    const offsetAmount = 1_000_000 - 1 # lensing

    var s = 0
    for i in 0..<galaxyPos.len:
        for j in 0..<i:
            var p1 = galaxyPos[i]
            var p2 = galaxyPos[j]

            var p10 = p1[0] + emptyLine[p1[0]] * offsetAmount
            var p11 = p1[1] + emptyCol[p1[1]] * offsetAmount
            var p20 = p2[0] + emptyLine[p2[0]] * offsetAmount
            var p21 = p2[1] + emptyCol[p2[1]] * offsetAmount

            var d = abs(p10 - p20) + abs(p11 - p21)
            s += d

    return $s


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
