// original: 0x00c07f90 stream_array_grow_80 (proposed)

/// Grow the 80-byte-element array when full and return the fresh slot.
///
/// `this` points to the array (`ITEMS` its buffer, `COUNT` its 16-bit length,
/// `CAP` its 16-bit capacity). When the length already equals the capacity the
/// capacity is raised by `grow`; a buffer of the whole new capacity comes from
/// the allocator (callee 1, byte size on the stack), every new slot is built
/// by the constructor (callee 2, slot in `ecx`, skipped for a null slot),
/// every old element is copied over by the copier (callee 3) and torn down by
/// the destructor (callee 4), the old buffer goes to the releaser (callee 5)
/// and the new buffer is installed. Either way the slot after the last element
/// is returned and the length advances by one, wrapping past 0xFFFF.
///
/// Original: 0x00c07f90 (thiscall, one stack word; 1 and 5 cdecl, rest thiscall).
lf_checker_rt::export!(thiscall, rw_00c07f90(this: u32, grow: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const CAP: u32 = 0x06;
        const STRIDE: u32 = 80;
        const MALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const COPY: u32 = 3;
        const DTOR: u32 = 4;
        const FREE: u32 = 5;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let cap = (this.wrapping_add(CAP) as *const u16).read_unaligned() as u32;
        if count == cap {
            let newcap = grow.wrapping_add(cap) & 0xffff;
            (this.wrapping_add(CAP) as *mut u16).write_unaligned(newcap as u16);
            let newbase: u32 =
                lf_checker_rt::callee_cdecl!(MALLOC, u32, newcap.wrapping_mul(STRIDE));
            if (newcap as i32) > 0 {
                let mut k = 0u32;
                while k < newcap {
                    let slot = newbase.wrapping_add(k.wrapping_mul(STRIDE));
                    if slot != 0 {
                        let _r: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, slot);
                    }
                    k += 1;
                }
            }
            let old = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count {
                let src = old.wrapping_add(i.wrapping_mul(STRIDE));
                let _r: u32 = lf_checker_rt::callee_thiscall!(
                    COPY, u32, newbase.wrapping_add(i.wrapping_mul(STRIDE)), src);
                i += 1;
            }
            if (count as i32) > 0 {
                let mut k = 0u32;
                while k < count {
                    let _r: u32 = lf_checker_rt::callee_thiscall!(
                        DTOR, u32, old.wrapping_add(k.wrapping_mul(STRIDE)));
                    k += 1;
                }
            }
            let old2 = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            let _r: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, old2);
            (this.wrapping_add(ITEMS) as *mut u32).write_unaligned(newbase);
        }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        let slot = base.wrapping_add(count.wrapping_mul(STRIDE));
        (this.wrapping_add(COUNT) as *mut u16).write_unaligned(count.wrapping_add(1) as u16);
        slot
    }
});
