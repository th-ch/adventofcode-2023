from times import cpuTime
from os import paramStr

proc scmp(bigs: string, smalls: static[string], idx: int): bool {.inline.} =
    if bigs.len < smalls.len + idx: return false
    for i in 0..<smalls.len:
        if bigs[i + idx] != smalls[i]: return false
    return true

proc isdig(c: char): bool {.inline.} =
    return c <= '9' and '0' <= c

# Returns one plus the index of the least significant 1-bit of x, or if x is zero, returns zero.
func builtin_ffs(x: cint): cint {.importc: "__builtin_ffs", nodecl.}

const fchar = {'o','t','f','s','e','n'}

proc parseVal(s: string, i: int): int =
    if isDig s[i]: return cast[int](s[i]) - cast[int]('0')
    if s[i] notin fchar: return -1
    if scmp(s, "one", i): return 1
    if scmp(s, "two", i): return 2
    if scmp(s, "three", i): return 3
    if scmp(s, "four", i): return 4
    if scmp(s, "five", i): return 5
    if scmp(s, "six", i): return 6
    if scmp(s, "seven", i): return 7
    if scmp(s, "eight", i): return 8
    if scmp(s, "nine", i): return 9
    return -1

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

proc firstNum(s: string, b: int, e: int): int =
    for i in b..<e:
        let v = parseVal(s, i)
        if v != -1:
            return v
    return 0

proc lastNum(s: string, b: int, e: int): int =
    for i in countdown(e-1, b):
        let v = parseVal(s, i)
        if v != -1:
            return v
    return 0

proc run(s: string): string =
    var cv = 0
    var b = 0
    var e = 0

    var i = 0

    var sptr = cast[int](s.cstring)
    const ldelim: uint16 = cast[uint16](cast[uint8]('\n') * 256.uint16 + cast[uint8]('\n'))
    var mask = mm256_set_epi16(ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim,ldelim)

    while i < s.len - 32:
        var stmp = mm256_loadu_si256(cast[ptr char](sptr + i))
        var matches_bytes = mm256_cmpeq_epi8(stmp, mask)
        var cmpResult = mm256_movemask_epi8(matches_bytes)
        cmpResult = builtin_ffs(cmpResult)

        if cmpResult != 0:
            i += cmpResult 
            e = i
            cv += firstNum(s, b, e) * 10
            cv += lastNum(s, b, e)
            b = i
            inc i
        else:
            i += 32

    # Handle the last 32 bytes by hand
    while i < s.len:
        if s[i] == '\n': 
            e = i
            cv += firstNum(s, b, e) * 10
            cv += lastNum(s, b, e)
            b = i
        inc i

    e = s.len
    cv += firstNum(s, b, e) * 10
    cv += lastNum(s, b, e)
    
    return $cv


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
