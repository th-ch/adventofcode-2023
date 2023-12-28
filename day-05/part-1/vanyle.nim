from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

proc parseInput(s: string): (seq[int], array[7, seq[(int,int,int)]]) =
    var t: Tokenizer = Tokenizer(s:s,offset:0)
    t.advance('\n')

    let seeds = ints(s.toOpenArray(0, t.offset-1))
    var maps: array[7, seq[(int,int,int)]]
    var i = 0

    t.advanceFixed(2)

    while t.offset < s.len:
        # Ignore the first line:
        t.advance('\n')
        t.advanceFixed(1)
        var nextGroup = t.findNext("\n\n", s.len)

        var ll: seq[(int,int,int)] = newSeqOfCap[(int, int, int)](40)

        while t.offset < nextGroup:
            var p = t.offset
            t.advance('\n', s.len)
            let tIntegers = ints(s.toOpenArray(p, t.offset-1))
            ll.add(
                (tIntegers[0], tIntegers[1], tIntegers[2])
            )
            t.advanceFixed(1)

        maps[i] = ll
        inc i

        t.offset = nextGroup + 2

    return (seeds, maps)

proc run(s: string): string =
    var (currentMap, maps) = parseInput(s)

    for map in maps:
        for j in 0..<currentMap.len:
            let e = currentMap[j]
            for (dest, source,ran) in map:
                if e >= source and e < source + ran:
                    currentMap[j] += dest - source
                    break

    return $min(currentMap)


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
