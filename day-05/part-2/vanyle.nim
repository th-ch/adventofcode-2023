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

    var ranges: seq[(int,int)] = @[]
    for i in countup(0, currentMap.len-1, 2):
        ranges.add((currentMap[i], currentMap[i] + currentMap[i+1] - 1))

    for map in maps:
        var tmpRange: seq[(int,int)] = @[]

        while ranges.len > 0:
            let r = ranges.pop()
            assert r[0] <= r[1]
            var mapped = false
            for (dest, source, ran) in map:
                
                if r[0] >= source and r[1] < source + ran:
                    # Naive range mapping.
                    var range1 = (r[0], r[1])
                    range1[0] += dest - source
                    range1[1] += dest - source
                    tmpRange.add(range1)
                    mapped = true
                    break

                if r[0] >= source and r[0] < source + ran:
                    # Split at source+ran
                    var range1 = (r[0], source + ran - 1)
                    let range2 = (source + ran, r[1])

                    range1[0] += dest - source
                    range1[1] += dest - source
                    tmpRange.add(range1)
                    ranges.add(range2)
                    mapped = true
                    break

                if r[1] >= source and r[1] < source + ran:
                    # Also split, but transform the top
                    var range1 = (r[0], source-1)
                    var range2 = (source, r[1])
                    
                    range2[0] += dest - source
                    range2[1] += dest - source
                    ranges.add(range1)
                    tmpRange.add(range2) # mapped!
                    mapped = true
                    break

                if r[0] < source and r[1] >= source + ran:
                    # double split
                    var range1 = (r[0], source-1)
                    var range2 = (source, source + ran - 1)
                    var range3 = (source + ran, r[1])
                    
                    range2[0] += dest - source
                    range2[1] += dest - source
                    ranges.add(range1)
                    tmpRange.add(range2)
                    ranges.add(range3)
                    mapped = true
                    break

            if not mapped:
                tmpRange.add(r) # no map corresponding to this range at this stage.

        ranges = tmpRange

    var minPos = ranges[0][0]
    for i in ranges:
        minPos = min(i[0], minPos)

    return $minPos


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
