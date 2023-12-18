use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    /*
    count the (signed) area between the horizontal segments and the x axis, counting on clockwise
    area is positive while above axis xor being clockwise
    use 2 representations: the step just taken and the step to make
    add area below R on 1st repr and remove area below L on 2nd ()
    URRRRRR   RRRRRRD     0000000  0000000  0000000  0000000  0000000  0000000
    U.....D   U.....D     0111111  0111111  0111111  0111111  0111111  0111111
    LLU...D   ULL...D     0111111  0111111  0111111  0111111  0111111  0111111
    ..U...D   ..U...D     0111111  0111111  0111111  0111111  0111111  0001111
    ..U...D   ..U...D     0111111  0111111  0111111  0111111  0111111  0001111
    URR.LLD   RRU.DLL ==> 0111111  0111111  0111111  0111111  0111111  0001111
    U...D..   U...D..     0111111  0111100  0111100  0111100  0221100  0111100
    LU..DRR   UL..RRD     0111111  0111100  0111100  0111100  0221100  0111100
    .U....D   .U....D     0111111  0111100  0111111  0011111  0121111  0011111
    .LLLLLD   .ULLLLL     0111111  0111100  0111111  0011111  0121111  0011111

    This way we fail to capture
     - all Rs from the 1st because we take the area below the line
     - all U from the 2nd because Rs form the 1st start right next to the Up line leading to them
     - all U from 1st & R from 2nd i.e up->right corners

    But this accounts 2 times for R from the 1st & U from 2nd = right->up corner
    On a clockwise loop, theres is 1 more U->R than R->U => add 1 + R + U

    Similarly for a counterclockwise loop, we get the dual:
    LLLLLLU DLLLLLL
    D.....U D.....U
    DRR...U RRD...U
    ..D...U ..D...U
    ..D...U ..D...U
    LLD.URR DLL.RRU
    D...U.. D...U..
    DR..LLU RD..ULL
    .D....U .D....U
    .DRRRRR .RRRRRU

    This way we fail to capture
     - all Ds from the 1st
     - all L from the 2nd
     - all L from 1st & D from 2nd i.e Left->Down corners
    But this accounts 2 times for L from the 1st & D from 2nd = Down->Left corner
    On a counterclockwise loop, theres is 1 more L->D than D->L => add 1 + D + L

    Or on a loop, R=L & D=U
    ==> return |area| + 1 + R + U
    */
    let mut cur = 0;
    let bytes = input.as_bytes();
    let mut area = 0;
    let mut h = 0;
    let mut correction = 1;
    while cur < bytes.len() {
        let (horizontal, sign) = match bytes[cur] {
            b'L' => (true, false),
            b'R' => (true, true),
            b'U' => (false, true),
            b'D' => (false, false),
            _ => panic!("Unexpected char {} for direction", bytes[cur]),
        };
        cur += 1;
        let mut parsed_nb = 0;
        loop {
            cur += 1;
            match bytes[cur] {
                b'0'..=b'9' => {
                    parsed_nb *= 10;
                    parsed_nb += (bytes[cur] - b'0') as isize;
                }
                _ => break,
            }
        }
        if horizontal {
            if sign {
                correction += parsed_nb;
                area += h * (parsed_nb);
            } else {
                area -= h * (parsed_nb);
            }
        } else if sign {
            correction += parsed_nb;
            h += parsed_nb;
        } else {
            h -= parsed_nb;
        }
        cur += 11;
    }
    area.abs() + correction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"),
            62
        )
    }
}
