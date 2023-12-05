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
