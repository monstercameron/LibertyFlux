// original: 0x00c07f10 stream_array_grow_40 (proposed)

/// Grow the 40-byte-element array when full and return the fresh slot.
///
/// `this` points to the array (`ITEMS` its buffer, `COUNT` its 16-bit length,
/// `CAP` its 16-bit capacity). When the length already equals the capacity the
/// capacity is raised by `grow`, a new buffer comes from the allocator
/// (callee 1), every element is moved over by the mover (callee 2, destination
/// in `ecx`, source on the stack), the old buffer goes to the releaser
/// (callee 3) and the new buffer is installed. Either way the slot after the
/// last element is returned and the length advances by one, wrapping past
/// 0xFFFF.
///
/// Original: 0x00c07f10 (thiscall, one stack word; all callees thiscall).
lf_checker_rt::export!(thiscall, rw_00c07f10(this: u32, grow: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const CAP: u32 = 0x06;
        const STRIDE: u32 = 40;
        const ALLOC: u32 = 1;
        const MOVE: u32 = 2;
        const RELEASE: u32 = 3;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let cap = (this.wrapping_add(CAP) as *const u16).read_unaligned() as u32;
        if count == cap {
            let newcap = grow.wrapping_add(cap);
            (this.wrapping_add(CAP) as *mut u16).write_unaligned(newcap as u16);
            let newbase: u32 =
                lf_checker_rt::callee_thiscall!(ALLOC, u32, this, newcap & 0xffff);
            let old = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count {
                let src = old.wrapping_add(i.wrapping_mul(STRIDE));
                let _r: u32 = lf_checker_rt::callee_thiscall!(
                    MOVE, u32, newbase.wrapping_add(i.wrapping_mul(STRIDE)), src);
                i += 1;
            }
            let c2 = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
            let old2 = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            let _r: u32 =
                lf_checker_rt::callee_thiscall!(RELEASE, u32, this, old2, c2);
            (this.wrapping_add(ITEMS) as *mut u32).write_unaligned(newbase);
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        let slot = base.wrapping_add(count.wrapping_mul(STRIDE));
        (this.wrapping_add(COUNT) as *mut u16).write_unaligned(count.wrapping_add(1) as u16);
        slot
    }
});
