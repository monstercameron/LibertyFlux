// original: 0x00bf7650 veh_overheat_apply_state
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr8(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

// original: 0x00bf7650 veh_overheat_apply_state (proposed)
/// Store a four-float state vector plus a flag bit into this object, then
/// classify the object's stored name hash against nine known overheating
/// part names and record which group matched.
///
/// Behaviour: forwards two setup calls (ids 1 and 2), copies the four
/// float arguments into `this+0x18..0x24`, sets bit 0 of `this+0x28` to
/// bit 0 of the last argument, then hashes each candidate name (id 3) and
/// compares with the key at `this+8`. A match in the first two names
/// writes 0 to `this+0x14`; a match in the remaining seven writes
/// `(this+0x28 & 1) + 1`; no match writes 0xFF. Always writes 2 to
/// `this+2`. Returns the matching hash answer on the first two names,
/// the value just written on the other seven, and the last hash answer
/// when nothing matched.
export!(thiscall, rw_bf7650(
    this_ptr: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> u32 {
    const SETUP: u32 = 1; // 0xBF40F0 thiscall/1
    const CONFIG: u32 = 2; // 0xBF41B0 thiscall/5
    const HASH: u32 = 3; // 0x40BA60 cdecl/2 (string, 0) -> hash
    const KIND: u32 = 0x2e;
    const G_MODE: u32 = 0x011735A4;
    // Candidate name string file-VAs, in probe order.
    const NAMES: [u32; 9] = [
        0x00EBBF88, 0x00EBBFA4, 0x00EBBFBC, 0x00EBBFD4, 0x00EBBFEC, 0x00EBC010,
        0x00EBC028, 0x00EBC040, 0x00EBC058,
    ];
    unsafe {
        callee_thiscall!(SETUP, u32, this_ptr, KIND);
        let mode = global::<u32>(G_MODE).read();
        callee_thiscall!(CONFIG, u32, this_ptr, KIND, mode, a1, a0, 1);
        // Four-float state vector (bit copies).
        wr32(this_ptr + 0x18, a2);
        wr32(this_ptr + 0x1c, a3);
        wr32(this_ptr + 0x20, a4);
        wr32(this_ptr + 0x24, a5);
        // Bit 0 of the flag byte := bit 0 of a6.
        let flags = rd8(this_ptr + 0x28);
        wr8(this_ptr + 0x28, flags ^ (((flags ^ (a6 as u8)) & 1)));
        // Name-hash dispatch on the stored key.
        let mut ans: u32 = 0;
        let mut i: usize = 0;
        while i < NAMES.len() {
            ans = callee_cdecl!(HASH, u32, relocated(NAMES[i]), 0);
            if rd32(this_ptr + 8) == ans {
                if i < 2 {
                    wr8(this_ptr + 0x14, 0);
                } else {
                    let v = (rd8(this_ptr + 0x28) & 1) + 1;
                    wr8(this_ptr + 0x14, v);
                    wr8(this_ptr + 2, 2);
                    return v as u32;
                }
                wr8(this_ptr + 2, 2);
                return ans;
            }
            i += 1;
        }
        wr8(this_ptr + 0x14, 0xff);
        wr8(this_ptr + 2, 2);
        ans
    }
});
