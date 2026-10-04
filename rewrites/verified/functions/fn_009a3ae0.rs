// original: 0x009a3ae0 audio_gate_tail_dispatch
/// Original 0x009a3ae0 (unnamed): gate a tail dispatch on audio state.
///
/// Probes the audio backend up to three times, then checks the owner words:
/// a null first probe or a tag mismatch skips the remaining probes and joins
/// the owner check directly, while a nonzero third-probe state exits with 1.
/// The owner word at +0x6c or +0x70 must equal 2; then the handle at +0x74
/// is resolved and the dispatcher is tail-called with the scaled record
/// pointer when its kind byte equals 2. Returns 1 on every early exit.
export!(thiscall, rw_009a3ae0(this_: u32) -> u32 {
    let p1 = callee_cdecl!(1, u32, 0, 0);
    if p1 != 0 {
        let p2 = callee_cdecl!(1, u32, 0, 0);
        let tag = unsafe { ((p2 + 0x2e) as *const u16).read() as i16 as i32 as u32 };
        let want = unsafe { (relocated(0x012F9F00) as *const u32).read() };
        if tag == want {
            let p3 = callee_cdecl!(1, u32, 0, 0);
            let state = unsafe { ((p3 + 0xa64) as *const u32).read() };
            if state != 0 {
                return 1;
            }
        }
    }
    let w6c = unsafe { ((this_ + 0x6c) as *const u32).read() };
    if w6c != 2 && unsafe { ((this_ + 0x70) as *const u32).read() } != 2 {
        return 1;
    }
    let h = unsafe { ((this_ + 0x74) as *const u32).read() };
    let q = callee_cdecl!(2, u32, h);
    if q == 0 {
        return 1;
    }
    let k = unsafe { ((q + 0x1917) as *const u8).read() as u32 };
    let r = q.wrapping_add(k.wrapping_mul(0xbd0));
    if r == 0 {
        return 1;
    }
    if unsafe { ((r + 0xbc0) as *const u8).read() } != 2 {
        return 1;
    }
    callee_thiscall!(3, u32, r)
});
