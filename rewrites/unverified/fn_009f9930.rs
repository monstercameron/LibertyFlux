// original: 0x009F9930 stat_award_by_action (proposed)

/// Award progression stats for one completed action.
///
/// `action` is an opaque action key. A lookup callee maps it to an object;
/// when the object exists and its redirect slot (dword at `+0x1c`) is not -1,
/// the slot's value replaces the key. A key of 0x2c or less selects one stat
/// id from a fixed 45-entry table (index 8 selects none) and records float
/// 1.0 against it. The object's kind slot (dword at `+0x0c`, always read, so
/// a null object faults) then selects a follow-up: kind 1 records one more
/// stat, kinds 2 through 5 record another and, when the low byte of the
/// original key is zero, a third; any other kind records nothing further.
///
/// The zero-low-byte test reads one word above the argument (caller stack left
/// behind after the object's register saves are popped), so the rewrite takes
/// it as an explicit second parameter read from the same slot.
///
/// Original: cdecl, one stack word plus the word above it, no meaningful
/// return value.
const ONE_BITS: u32 = 0x3f80_0000; // float 1.0
const MAX_TABLE_KEY: u32 = 0x2c;
const NO_REDIRECT: u32 = 0xffff_ffff;
const OBJ_KIND: u32 = 0x0c;
const OBJ_REDIRECT: u32 = 0x1c;
const FOLLOW_KIND_ONE: u32 = 0x122;
const FOLLOW_KIND_FEW: u32 = 0x123;
const FOLLOW_ZERO_LOW_BYTE: u32 = 0x29;
const LOOKUP_CALLEE: u32 = 0;
const STAT_ADD_CALLEE: u32 = 1;

/// Stat id selected by a table key, or `None` for the one empty slot.
fn stat_for_key(key: u32) -> Option<u32> {
    match key {
        0x00 => Some(0x177),
        0x01 => Some(0x178),
        0x02 => Some(0x179),
        0x03 => Some(0x17a),
        0x04 => Some(0x17b),
        0x05 => Some(0x17c),
        0x06 => Some(0x188),
        0x07 => Some(0x17e),
        0x08 => None,
        0x09 => Some(0x17f),
        0x0a => Some(0x180),
        0x0b => Some(0x181),
        0x0c => Some(0x182),
        0x0d => Some(0x183),
        0x0e => Some(0x184),
        0x0f => Some(0x185),
        0x10 => Some(0x186),
        0x11 => Some(0x187),
        0x12 => Some(0x188),
        0x13 => Some(0x189),
        0x14 => Some(0x18a),
        0x15 => Some(0x18b),
        0x16 => Some(0x18c),
        0x17 => Some(0x18d),
        0x18 => Some(0x18e),
        0x19 => Some(0x18f),
        0x1a => Some(0x190),
        0x1b => Some(0x191),
        0x1c => Some(0x192),
        0x1d => Some(0x193),
        0x1e => Some(0x194),
        0x1f => Some(0x195),
        0x20 => Some(0x196),
        0x21 => Some(0x197),
        0x22 => Some(0x198),
        0x23 => Some(0x199),
        0x24 => Some(0x19a),
        0x25 => Some(0x19b),
        0x26 => Some(0x19c),
        0x27 => Some(0x19d),
        0x28 => Some(0x19e),
        0x29 => Some(0x19f),
        0x2a => Some(0x1a0),
        0x2b => Some(0x1a1),
        0x2c => Some(0x1a2),
        _ => None,
    }
}

lf_checker_rt::export!(cdecl, rw_009F9930(action: u32, caller_word: u32) -> u32 {
    unsafe {
        let obj = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, action);
        let mut key = action;
        if obj != 0 {
            let redirect = ((obj + OBJ_REDIRECT) as *const u32).read_unaligned();
            if redirect != NO_REDIRECT {
                key = redirect;
            }
        }
        if key <= MAX_TABLE_KEY {
            if let Some(stat) = stat_for_key(key) {
                lf_checker_rt::callee_cdecl!(STAT_ADD_CALLEE, u32, stat, ONE_BITS);
            }
        }
        let kind = ((obj + OBJ_KIND) as *const u32).read_unaligned();
        if kind == 1 {
            lf_checker_rt::callee_cdecl!(STAT_ADD_CALLEE, u32, FOLLOW_KIND_ONE, ONE_BITS);
        } else if kind.wrapping_sub(2) <= 3 {
            lf_checker_rt::callee_cdecl!(STAT_ADD_CALLEE, u32, FOLLOW_KIND_FEW, ONE_BITS);
            if caller_word & 0xff == 0 {
                lf_checker_rt::callee_cdecl!(STAT_ADD_CALLEE, u32, FOLLOW_ZERO_LOW_BYTE, ONE_BITS);
            }
        }
        0
    }
});
