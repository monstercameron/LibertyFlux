// original: 0x00981f40 audio_detach_table_entry
/// Original 0x00981f40 (unnamed): detach an entry from the ambient table.
///
/// When the global audio gate is set, resolves the caller's token through two
/// lookups and, on success, scans the 245-entry table at +0x5c08 for the entry
/// matching `arg`, clearing it. Void; the checker compares calls and heap.
export!(thiscall, rw_00981f40(this_: u32, arg: u32) -> u32 {
    let gate = unsafe { (relocated(0x01038A20) as *const u8).read() };
    if gate == 0 {
        return 0;
    }
    let p = unsafe { ((arg + 0x78) as *const u32).read() };
    let q = if p == 0 {
        0
    } else {
        unsafe { ((p + 0xf8) as *const u32).read() }
    };
    let tok = callee_cdecl!(1, u32, q, 0);
    let h = callee_thiscall!(2, u32, relocated(0x0115DC18), tok);
    if h == 0 {
        return 0;
    }
    let guard = [0u32; 2];
    callee_thiscall!(3, u32, guard.as_ptr() as u32, this_.wrapping_add(0x6f3c));
    let mut i = 0u32;
    while i < 0xf5 {
        let e = this_.wrapping_add(0x5c08).wrapping_add(i.wrapping_mul(0x14));
        let live = unsafe { ((e + 4) as *const u8).read() };
        if live != 0 {
            let key = unsafe { (e as *const u32).read() };
            if key == arg {
                unsafe { (e as *mut u32).write(0); }
                callee_thiscall!(4, u32, guard.as_ptr() as u32);
                return 0;
            }
        }
        i += 1;
    }
    callee_thiscall!(4, u32, guard.as_ptr() as u32);
    0
});
