// original: 0x00954700 record_class_check (proposed)

/// Classify a record by key match or by its flag field.
///
/// Reads a signed 16-bit key at `KEY_OFF` (0x2E) in the record. When it
/// equals any of the three global keys `K1..K3` (exact signed equality)
/// the original returns with only AL set, so the result is the key with
/// its low byte forced to 1. Otherwise takes bits 6..9 of the dword at
/// `FLAGS_OFF` (0x28) and returns 0 when that field is 1, 5, 6, 7, 8 or
/// 0xA, else 1. Original is cdecl/1, returns EAX.
lf_checker_rt::export!(cdecl, rw_00954700(rec: u32) -> u32 {
    const KEY_OFF: u32 = 0x2E;
    const FLAGS_OFF: u32 = 0x28;
    const K1: u32 = 0x012FA2D8;
    const K2: u32 = 0x012FA0E0;
    const K3: u32 = 0x012F9FFC;
    let key = unsafe { (rec.wrapping_add(KEY_OFF) as *const u16).read_unaligned() } as i16 as i32;
    let matched = ((key as u32) & 0xFFFFFF00) | 1;
    let k1 = unsafe { lf_checker_rt::global::<i32>(K1).read() };
    if key == k1 {
        return matched;
    }
    let k2 = unsafe { lf_checker_rt::global::<i32>(K2).read() };
    if key == k2 {
        return matched;
    }
    let k3 = unsafe { lf_checker_rt::global::<i32>(K3).read() };
    if key == k3 {
        return matched;
    }
    let field = (unsafe { (rec.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned() } >> 6) & 0xF;
    if field == 1 || field == 5 || field == 6 || field == 7 || field == 8 || field == 0xA {
        0
    } else {
        1
    }
});
