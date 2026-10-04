// original: 0x00dacdd0 CTaskComplexTrackEntity::vf1
/// Clone helper: pick a source value from the flag byte at +0x4C, allocate a
/// fresh object (a null allocation falls into the original's null store
/// through [0+0x50], reproduced here as the same faulting write),
/// otherwise construct the copy from seven member fields - two of them
/// single-precision bit patterns forwarded unchanged - stamp +0x50 from
/// this object and return the new object.
export!(thiscall, rw_00dacdd0(this: u32) -> u32 {
    unsafe {
        let bytes = this as *const u8;
        let words = this as *const u32;
        let edi = if *bytes.byte_add(0x4C) != 0 {
            (*words.byte_add(0x48))
                .wrapping_sub(*global::<u32>(0x011735B4))
                .wrapping_add(*words.byte_add(0x44))
        } else {
            *words.byte_add(0x34)
        };
        let alloc = *global::<u32>(0x0167E2A0);
        let obj: u32 = callee_thiscall!(1, u32, alloc);
        if obj == 0 {
            // The original stores through [0+0x50] here and faults; the
            // checker compares fault parity, so fault identically.
            let v = *words.byte_add(0x50);
            core::ptr::write_volatile(0x50 as *mut u32, v);
            return 0;
        }
        let m14 = *words.byte_add(0x14);
        let p20 = this.wrapping_add(0x20);
        let b30 = *bytes.byte_add(0x30) as u32;
        let f38 = *words.byte_add(0x38);
        let f3c = *words.byte_add(0x3C);
        let b40 = *bytes.byte_add(0x40) as u32;
        let obj2: u32 = callee_thiscall!(2, u32, obj, m14, p20, b30, edi, f38, f3c, b40);
        *((obj2 as *mut u32).byte_add(0x50)) = *words.byte_add(0x50);
        obj2
    }
});
