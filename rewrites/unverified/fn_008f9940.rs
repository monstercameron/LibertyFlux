// original: 0x008F9940 text_match_state_machine
/// Match a 16-bit string against a four-state word pattern.
///
/// A null string matches at once. Otherwise the length comes from
/// the length routine and each word steps one of four states: state
/// 0 accepts the marker words 0x7e or 0x807e into state 1, state 1
/// accepts the two expected words into states 2 or 3, state 2
/// accepts a marker as a full match (returns 1), and state 3 accepts
/// a marker back into state 0. Any other word fails (returns 0), as
/// does running out of words. Cdecl, three stack arguments
/// (string, first expected word, second expected word).
export!(cdecl, rw_008f9940(s: u32, x: u32, y: u32) -> u32 {
    unsafe {
        const MARKER: u16 = 0x7e;
        const MARKER_ALT: u16 = 0x807e;
        if s == 0 {
            return 1;
        }
        let len: u32 = callee_cdecl!(1, u32, s);
        let n = (len & 0xFFFF) as i32;
        let mut ok = true;
        let mut state: u32 = 0;
        let mut i: i32 = 0;
        while ok && i < n {
            if state <= 3 {
                let w = ((s + (i as u32).wrapping_mul(2)) as *const u16).read_unaligned();
                match state {
                    0 => {
                        if w == MARKER || w == MARKER_ALT {
                            state = 1;
                        } else {
                            ok = false;
                        }
                    }
                    1 => {
                        if (w as u32) == x {
                            state = 2;
                        } else if (w as u32) == y {
                            state = 3;
                        } else {
                            ok = false;
                        }
                    }
                    2 => {
                        if w == MARKER || w == MARKER_ALT {
                            return 1;
                        }
                        ok = false;
                    }
                    _ => {
                        if w == MARKER || w == MARKER_ALT {
                            state = 0;
                        } else {
                            ok = false;
                        }
                    }
                }
            }
            i += 1;
        }
        0
    }
});
