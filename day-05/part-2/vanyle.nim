from times import cpuTime
from os import paramStr

import sequtils, strutils, sugar

proc parseInput(s: string): (seq[int], seq[seq[(int,int,int)]]) = 
    let l = s.strip.split("\n\n")

    let seeds = l[0].split(": ")[1].split(" ").map(x => x.strip.parseInt)
    let rest = l[1..<l.len]
    
    
    let rest2 = rest.map(x => x.split(":\n")[1])

    var maps: seq[seq[(int,int,int)]] = @[]

    for amap in rest2:
        let triplets = amap.split("\n")
        var ll: seq[(int,int,int)] = @[]
        for t in triplets:
            let tIntegers = t.split(" ").map(x => x.strip.parseInt)
            ll.add(
                (tIntegers[0], tIntegers[1], tIntegers[2])
            )
        maps.add(ll)

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
