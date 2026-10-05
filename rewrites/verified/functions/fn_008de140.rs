// original: 0x008de140 flip_selector_and_rebuild

/// Flip the header's two-state selector, rebuild both embedded members and
/// re-key the selector's hash row, then release the manager guard.
///
/// The old selector becomes the lap counter and vice versa (`new = 1 - old`);
/// the selector byte at `+0x08 + old` and the row byte at `+0x848 + old` are
/// cleared, and the rebuild call receives the row address
/// `this + 0x48 + (old << 10)` with a zero tag and a `0x400` capacity. Both
/// member releases target `this + 0x30` and `this + 0x3c`; the manager calls
/// (open and tail) target the fixed manager.
///
/// Original: thiscall; `old` is 0 or 1; returns the tail call's answer.
lf_checker_rt::export!(thiscall, rw_008de140(this: *mut u32) -> u32 {
    unsafe {
        let mgr = lf_checker_rt::relocated(0x11737D0);
        lf_checker_rt::callee_thiscall!(1, u32, mgr);
        let old = *this.add(4);
        *this.add(4) = 1u32.wrapping_sub(old);
        *this.add(5) = old;
        *this.add(6) = 0;
        *this.add(7) = 0;
        *(this.byte_add(8 + old as usize) as *mut u8) = 0;
        lf_checker_rt::callee_thiscall!(2, u32, this.byte_add(0x30) as u32);
        lf_checker_rt::callee_thiscall!(2, u32, this.byte_add(0x3c) as u32);
        let sel = *this.add(5);
        let row = (this as u32)
            .wrapping_add(0x48)
            .wrapping_add(sel.wrapping_shl(10));
        lf_checker_rt::callee_cdecl!(3, u32, row, 0u32, 0x400u32);
        let sel2 = *this.add(5);
        *(this.byte_add(0x848 + sel2 as usize) as *mut u8) = 0;
        lf_checker_rt::callee_thiscall!(1, u32, mgr)
    }
});
