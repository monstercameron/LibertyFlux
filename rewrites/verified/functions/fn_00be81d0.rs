// original: 0x00BE81D0 pair_rule_check (proposed)
/// Decide whether the pair (a, b) is rejected (1) or accepted (0).
///
/// The original dispatches on `a` through a jump table:
/// - a == 1: accept when b <= 1, else reject;
/// - a == 2: accept when b is 2 or 3, else reject;
/// - a == 3 or a == 4: accept when b - 1 <= 2 (wrapping), i.e. b in
///   1..=3, else reject (b == 0 wraps and is rejected);
/// - anything else: reject.
/// All comparisons are unsigned. Pure function of its two words.
///
/// Original: 0x00BE81D0 (cdecl, two stack words, byte result).
lf_checker_rt::export!(cdecl, rw_00BE81D0(a: u32, b: u32) -> u32 {
    match a {
        1 => if b <= 1 { 0 } else { 1 },
        2 => if b == 2 || b == 3 { 0 } else { 1 },
        3 | 4 => if b.wrapping_sub(1) <= 2 { 0 } else { 1 },
        _ => 1,
    }
});

