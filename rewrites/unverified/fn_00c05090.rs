// original: 0x00c05090 stream_copy_3 (proposed)

/// Pass two fields and an argument to a callee, then copy three fields out.
///
/// `this` points to the object. Calls the worker callee with (`a1`, the
/// words at `+0x18` and `+0x1c`), then copies the words at `+0x20`, `+0x24`
/// and `+0x28` into the caller buffer `out`. Returns the third word (what
/// the original leaves in `eax`).
///
/// Original: 0x00c05090 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c05090(this: u32, a1: u32, out: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(
            CALLEE,
            u32,
            a1,
            (this.wrapping_add(0x18) as *const u32).read_unaligned(),
            (this.wrapping_add(0x1c) as *const u32).read_unaligned()
        );
        let v0 = (this.wrapping_add(0x20) as *const u32).read_unaligned();
        (out as *mut u32).write_unaligned(v0);
        let v1 = (this.wrapping_add(0x24) as *const u32).read_unaligned();
        (out.wrapping_add(4) as *mut u32).write_unaligned(v1);
        let v2 = (this.wrapping_add(0x28) as *const u32).read_unaligned();
        (out.wrapping_add(8) as *mut u32).write_unaligned(v2);
        v2
    }
});
