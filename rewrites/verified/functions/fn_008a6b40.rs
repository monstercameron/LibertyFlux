// original: 0x008a6b40 aud_reactor_init_triple
/// Initialize three fixed sub-objects, then finalize the parent.
///
/// Calls the sub-initializer on the three consecutive records starting at
/// +0x1620 (each 0x28 bytes), then the finalizer on the object itself.
/// Returns the object pointer.
export!(thiscall, rw_008a6b40(this: *mut u8) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        while i < 3 {
            let rec = (this as u32).wrapping_add(0x1620).wrapping_add(i.wrapping_mul(0x28));
            let _: u32 = callee_thiscall!(1, u32, rec);
            i = i.wrapping_add(1);
        }
        let _: u32 = callee_thiscall!(2, u32, this as u32);
        this as u32
    }
});
