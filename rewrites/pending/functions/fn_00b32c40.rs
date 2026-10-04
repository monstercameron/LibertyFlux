// original: 0x00b32c40 subtask_setup (proposed)

/// Fill in a sub-task record for a new assignment.
///
/// `this` points to the record. `kind` is stored at `+0x0`; the four words
/// of the vector at `vec` go to `+0x10` through `+0x1c`; `weight` goes to
/// `+0x20`; the bytes at `+0x24` and `+0x25` and the byte at `+0x27` are
/// cleared. `first` and `second` are stored at `+0x28` and `+0x2c`; each
/// non-null one is first retained through a callee that takes the pointer's
/// own address, and a non-null `first` also sets the byte at `+0x26`.
/// `lo` and `hi` go to `+0x30` and `+0x34`. For kinds 17, 18, 24, 25, 29 and
/// 30 the stamp word at `+0x38` takes the shared tick count, the spare word
/// at `+0x3c` takes 1000 and the timer flag byte at `+0x40` is set;
/// otherwise the flag byte is cleared. `tag` goes to `+0x44` either way and
/// is also the return value.
///
/// Original: 0x00b32c40 (thiscall, eight stack words; one one-word callee
/// reached from two sites).
lf_checker_rt::export!(thiscall, rw_00b32c40(
    this: u32,
    kind: u32,
    vec: u32,
    weight: u32,
    first: u32,
    second: u32,
    lo: u32,
    hi: u32,
    tag: u32,
) -> u32 {
    unsafe {
        const TIMED_KINDS: [u32; 6] = [0x11, 0x12, 0x18, 0x19, 0x1d, 0x1e];
        const TICKS: u32 = 0x011735b4;
        const RETAIN: u32 = 1;
        #[inline(always)]
        unsafe fn w32(o: u32, off: u32, v: u32) {
            unsafe { (o as *mut u32).byte_add(off as usize).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn r32(o: u32, off: u32) -> u32 {
            unsafe { (o as *const u32).byte_add(off as usize).read_unaligned() }
        }
        (this as *mut u8).byte_add(0x27).write(0);
        w32(this, 0x00, kind);
        w32(this, 0x10, r32(vec, 0x00));
        w32(this, 0x14, r32(vec, 0x04));
        w32(this, 0x18, r32(vec, 0x08));
        w32(this, 0x1c, r32(vec, 0x0c));
        (this as *mut u16).byte_add(0x24).write_unaligned(0);
        w32(this, 0x20, weight);
        w32(this, 0x28, first);
        if first != 0 {
            (this as *mut u8).byte_add(0x26).write(1);
            lf_checker_rt::callee_thiscall!(RETAIN, u32, first, this.wrapping_add(0x28));
        }
        w32(this, 0x2c, second);
        if second != 0 {
            lf_checker_rt::callee_thiscall!(RETAIN, u32, second, this.wrapping_add(0x2c));
        }
        w32(this, 0x30, lo);
        w32(this, 0x34, hi);
        if TIMED_KINDS.contains(&kind) {
            w32(this, 0x38, lf_checker_rt::global::<u32>(TICKS).read());
            w32(this, 0x3c, 1000);
            (this as *mut u8).byte_add(0x40).write(1);
        } else {
            (this as *mut u8).byte_add(0x40).write(0);
        }
        w32(this, 0x44, tag);
        tag
    }
});
