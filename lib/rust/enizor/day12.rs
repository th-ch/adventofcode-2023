const DAMAGED: u8 = b'.';
const OPERATIONAL: u8 = b'#';
const UNKNOWN: u8 = b'?';

fn groups_size(groups: &[usize]) -> usize {
    if groups.is_empty() {
        0
    } else {
        let mut size = 0;
        for g in groups {
            size += g + 1;
        }
        size - 1
    }
}

fn strip_ends(row: &[u8]) -> &[u8] {
    let first = row.iter().position(|s| *s != DAMAGED);
    let last = row.iter().rev().position(|s| *s != DAMAGED);
    if let Some(f) = first {
        let l = row.len() - last.unwrap();
        &row[f..l]
    } else {
        &[]
    }
}

// Count the number of arrangements if row immediately starts with a group of size >= 0
// presupposes that groups isn't empty and can fit inside row on only jokers
fn count_start(row: &[u8], groups: &mut [usize]) -> usize {
    let g = groups[0];
    // from before the groups fit i.e row.len >= g
    for s in &row[..g] {
        if *s == DAMAGED {
            // the first group would be < g
            return 0;
        }
    }
    if row.len() == g {
        // groups should fit
        assert!(groups.len() == 1);
        return 1;
    }
    if row[g] == OPERATIONAL {
        // the first group would be > g
        return 0;
    }
    // count the arrangements without this first group we determined uniquely=
    count_arrangements(&row[g + 1..], &mut groups[1..])
}

// Count the number of arrangements if row ends with a group of size >= 0
// presupposes that groups isn't empty and can fit inside row on only jokers
fn count_end(row: &[u8], groups: &mut [usize]) -> usize {
    let g = *groups.last().unwrap();
    let row_len = row.len();
    let groups_len = groups.len();
    for s in &row[row_len - g..] {
        if *s == DAMAGED {
            // the last group would be < g
            return 0;
        }
    }
    if row.len() == g {
        // groups should fit
        assert!(groups.len() == 1);
        return 1;
    }
    if row[row_len - g - 1] == OPERATIONAL {
        // the last group would be > g
        return 0;
    }
    // count the arrangements without this last group we determined uniquely
    count_arrangements(&row[..row_len - g - 1], &mut groups[..groups_len - 1])
}

const MAX_LEEWAY: usize = 40;
const MAX_GROUPS: usize = 40;
const COUNT_LEEWAY: [[usize; MAX_GROUPS]; MAX_LEEWAY] = count_leeway();
const MAX_SIZE: usize = 64;

// Count the number of way to fit the groups inside a row of row_len ? only with leeway . to add besides the packed representation
// e.g.
const fn count_leeway() -> [[usize; MAX_GROUPS]; MAX_LEEWAY] {
    let mut res = [[0usize; MAX_GROUPS]; MAX_LEEWAY];
    let mut leeway;
    let mut groups_len;
    let mut k;
    // with 0 leeway or 0 groups theres is only 1 way
    // with 1 groups theres is exactly `leeways+1` ways: place [0;leeway] before and the rest after
    leeway = 1;
    while leeway < MAX_LEEWAY {
        res[leeway][0] = 1;
        res[leeway][1] = leeway + 1;
        leeway += 1;
    }
    groups_len = 0;
    while groups_len < MAX_GROUPS {
        res[0][groups_len] = 1;
        groups_len += 1;
    }
    leeway = 1;
    while leeway < MAX_LEEWAY {
        leeway += 1;
    }
    leeway = 1;
    while leeway < MAX_LEEWAY {
        groups_len = 2;
        while groups_len < MAX_GROUPS {
            k = 0;
            if groups_len + leeway > MAX_SIZE {
                break;
            }
            while k <= leeway {
                // count when placing `k` additional . before the 1st group
                res[leeway][groups_len] += res[leeway - k][groups_len - 1];
                k += 1;
            }
            groups_len += 1;
        }
        leeway += 1;
    }
    res
}

pub fn count_arrangements(mut row: &[u8], groups: &mut [usize]) -> usize {
    // starting and ending . are useless
    row = strip_ends(row);
    let row_len = row.len();
    let groups_len = groups.len();
    if groups.is_empty() {
        if row.contains(&OPERATIONAL) {
            return 0;
        } else {
            return 1;
        }
    }
    if row.is_empty() {
        return 0;
    }
    let size: usize = groups_size(groups);
    if size > row_len {
        return 0;
    }
    if size == row_len {
        let mut cur = 0;
        for g in groups {
            for _ in 0..*g {
                if row[cur] == DAMAGED {
                    return 0;
                }
                cur += 1;
            }
            if cur < row_len && row[cur] == OPERATIONAL {
                return 0;
            }
            cur += 1;
        }
        return 1;
    }
    // we now know that both row and groups are nonempty, and it there was only ? the groups would fit
    // try to see if there is already group at the start of the row
    if row[0] == OPERATIONAL {
        return count_start(row, groups);
    }
    if row.last() == Some(&OPERATIONAL) {
        return count_end(row, groups);
    }
    assert!(row[0] == UNKNOWN);
    if row.iter().all(|s| *s == UNKNOWN) {
        let leeway = row_len - size;
        return COUNT_LEEWAY[leeway][groups_len];
    }
    let run_length = row.iter().position(|s| *s != UNKNOWN).unwrap();
    if row[run_length] == DAMAGED {
        // try to fit as much groups as possible in the first run of `?`
        // all other groups must be strictly after
        let mut res = 0;
        for k in 0..=groups_len {
            let size = groups_size(&groups[..k]);
            if size > run_length {
                break;
            }
            // number of ways to fit the first `k` groups in to the run
            // times the number of way to fit the rest of the groups into the rest of the row
            let leeway = run_length - size;
            res += COUNT_LEEWAY[leeway][k]
                * count_arrangements(&row[run_length + 1..], &mut groups[k..]);
        }
        res
    } else {
        // it must be part of a group, try all possibilities
        let mut res = 0;
        for k in 0..groups_len {
            // try to use the kth group for it
            let mut size = groups_size(&groups[..k]);
            // there must be a . between the preceding groups and the located #
            if size > run_length - 1 {
                break;
            }
            if size > 0 {
                // a non empty group has to finish with .
                size += 1;
            }
            // the kth group will have `offset` # before the located one
            // to still be able to fit everything, we must have size+offset<=run_length
            // we also need offset < groups[k]
            let max_offset = groups[k].min(run_length - size + 1);
            // split into [0..run_length-2-offset].[run_length-offset..run_length-offset+group[k]]
            for offset in (0..max_offset).rev() {
                let leeway = run_length - offset - size;
                // number of ways to fit the first `k` groups in to the run
                // times number of ways of using the located # as the start
                groups[k] -= offset + 1;
                // Ensure rest can still be placed
                if groups_size(&groups[k..]) > row_len - run_length - 1 {
                    groups[k] += offset + 1;
                    // assert_eq!(count_arrangements(b"??#?????#?????", &mut [5,7]), 1);
                    break;
                }
                res +=
                    COUNT_LEEWAY[leeway][k] * count_start(&row[run_length + 1..], &mut groups[k..]);
                groups[k] += offset + 1;
            }
        }
        res
    }
}

pub fn copy_springs(input: &[u8]) -> Vec<u8> {
    let mut res = input.to_vec();
    res.reserve(input.len() * 4 + 4);
    for _ in 0..4 {
        res.push(UNKNOWN);
        res.extend(input);
    }
    res
}

pub fn copy_groups(input: &[usize]) -> Vec<usize> {
    let mut res = input.to_vec();
    res.reserve(input.len() * 4);
    for _ in 0..4 {
        res.extend(input);
    }
    res
}

pub fn count_with_copies(row: &[u8], groups: &[usize]) -> usize {
    count_arrangements(&copy_springs(row), &mut copy_groups(groups))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_test() {
        assert_eq!(count_arrangements(b"??", &mut [1]), 2);
        assert_eq!(count_arrangements(b"???.###", &mut [1, 1, 3]), 1);
        assert_eq!(count_arrangements(b".??..??...?##.", &mut [1, 1, 3]), 4);
        assert_eq!(count_arrangements(b"?#?#?#?#?#?#?#?", &mut [1, 3, 1, 6]), 1);
        assert_eq!(count_arrangements(b"????.#...#...", &mut [4, 1, 1]), 1);
        assert_eq!(
            count_arrangements(b"????.######..#####.", &mut [1, 6, 5]),
            4
        );
        assert_eq!(count_arrangements(b"?###????????", &mut [3, 2, 1]), 10);
        assert_eq!(count_arrangements(b"?##.?.?", &mut [3, 1, 1]), 1);
        assert_eq!(count_arrangements(b"??#?????#?????", &mut [5, 7]), 3);
        assert_eq!(
            count_arrangements(b"???#????###?????##??", &mut [4, 6, 3]),
            20
        );
    }

    #[test]
    fn copy_test() {
        assert_eq!(count_with_copies(b"???.###", &[1, 1, 3]), 1);
        assert_eq!(count_with_copies(b".??..??...?##.", &[1, 1, 3]), 16384);
        assert_eq!(count_with_copies(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(count_with_copies(b"????.#...#...", &[4, 1, 1]), 16);
        assert_eq!(count_with_copies(b"????.######..#####.", &[1, 6, 5]), 2500);
        assert_eq!(count_with_copies(b"?###????????", &[3, 2, 1]), 506250);
        assert_eq!(
            count_with_copies(b"???#????###?????##??", &[4, 6, 3]),
            3200000
        );
    }
}
