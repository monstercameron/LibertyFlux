// original: 0x00889650 stream_manager_allocate_arrays (proposed)

/// Allocate a voice manager's slot table and mark array.
///
/// `this` points to the new manager, `count` is the slot count (`name` is
/// accepted but never read). Initializes the embedded guard at `this + 8`
/// (patched callee), allocates `count * 0x4f0` bytes of slots and `count`
/// bytes of marks (patched callee, twice), stores both at `this + 0` and
/// `this + 4`, writes `count` to `+0x28`/`+0x2c`, -1 to `+0x30`, 1 to
/// `+0x34`, and sets bit 7 of each mark byte. A null allocation with positive
/// count would fault the original; the proof only runs null with count 0.
/// Returns `this`.
///
/// Original: 0x00889650 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00889650(this: u32, count: u32, name: u32) -> u32 {
    unsafe {
        const GUARD_INIT: u32 = 1;
        const ALLOCATOR: u32 = 2;
        const SLOT_SIZE: u32 = 0x4f0;
        const GUARD_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x28;
        const MARK_BIT: u8 = 0x80;

        let _ = name;
        lf_checker_rt::callee_thiscall!(GUARD_INIT, u32, this.wrapping_add(GUARD_OFF));
        let slots = lf_checker_rt::callee_cdecl!(ALLOCATOR, u32, count.wrapping_mul(SLOT_SIZE));
        let marks = lf_checker_rt::callee_cdecl!(ALLOCATOR, u32, count);
        (this as *mut u32).write_unaligned(slots);
        ((this + 4) as *mut u32).write_unaligned(marks);
        ((this + 0x34) as *mut u8).write(1);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(count);
        ((this + 0x2c) as *mut u32).write_unaligned(count);
        ((this + 0x30) as *mut u32).write_unaligned(0xffff_ffff);
        if (count as i32) > 0 {
            let mut i = 0u32;
            while i < count {
                let cell = marks.wrapping_add(i) as *mut u8;
                cell.write(cell.read() | MARK_BIT);
                i = i.wrapping_add(1);
            }
        }
        this
    }
});
