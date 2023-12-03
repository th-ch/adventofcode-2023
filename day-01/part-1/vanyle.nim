from times import cpuTime
from os import paramStr

# Simd magic!
type
  M256* {.importc: "__m256", header: "immintrin.h".} = object
  M256i* {.importc: "__m256i", header: "immintrin.h".} = object
  M256d* {.importc: "__m256d", header: "immintrin.h".} = object

{.push header: "immintrin.h".}

func mm256_cmpeq_epi8*(a, b: M256i): M256i {.importc: "_mm256_cmpeq_epi8".}
func mm256_movemask_epi8*(a: M256i): int32 {.importc: "_mm256_movemask_epi8".}
func mm256_set_epi16*(e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0: int16 | uint16): M256i {.importc: "_mm256_set_epi16".}
func mm256_loadu_si256*(p: pointer): M256i {.importc: "_mm256_loadu_si256".}

{.pop.}

func builtin_ffs(x: cint): cint {.importc: "__builtin_ffs", nodecl.}

proc isdig(c: char): bool {.inline.} =
    return c <= '9' and '0' <= c

proc firstNum(s: string, b: int, e: int): int =
    for i in b..<e:
        if isdig(s[i]): return (cast[int](s[i]) - cast[int]('0'))
    return 0

proc lastNum(s: string, b: int, e: int): int =
    for i in countdown(e-1, b):
        if isdig(s[i]): return (cast[int](s[i]) - cast[int]('0'))
    return 0

proc run(s: string): string =
    var cv = 0
    var i = 0

    var b = 0
    var e = 0

    var sptr = cast[int](s.cstring)
    const ldelim: uint16 = cast[uint16](cast[uint8]('\n') * 256.uint16 + cast[uint8]('\n'))
    var mask = mm256_set_epi16(ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim)

    while i < s.len:
        var stmp = mm256_loadu_si256(cast[ptr char](sptr + i))
        var matches_bytes = mm256_cmpeq_epi8(stmp, mask)
        var cmpResult = mm256_movemask_epi8(matches_bytes)
        cmpResult = builtin_ffs(cmpResult)

        if cmpResult != 0:
            i += cmpResult
            e = i
            cv += lastNum(s, b , e)
            cv += 10*firstNum(s, b , e)
            b = i
        else:
            i += 32

    while i < s.len:
        if s[i] == '\n': 
            e = i
            cv += firstNum(s, b, e) * 10
            cv += lastNum(s, b, e)
            b = i
        inc i

    e = s.len
    cv += lastNum(s, b , e)
    cv += 10*firstNum(s, b , e)

    return $cv


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
