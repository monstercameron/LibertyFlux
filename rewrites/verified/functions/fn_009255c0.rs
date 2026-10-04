// original: 0x009255c0 render_target_release_all (proposed)

/// Release the four cached render-target objects and clear their slots.
///
/// The four global slots (0x119CFF0, 0x119CFF4, 0x119CFE8, 0x119CFEC) each
/// hold either null or a pointer to a refcounted object: vtable pointer at
/// +0x00, a kind byte at +0x08, a 16-bit refcount at +0x0A. For each slot, in
/// order: a null slot is left alone; otherwise, if the refcount is nonzero
/// it is decremented, and when it reaches zero the object is destroyed
/// through vtable slot 0 with argument 1, but only when the kind byte is 2
/// or 4. A zero refcount skips the decrement and the destroy check. A
/// non-null slot is always cleared to null afterwards, whether or not the
/// object was destroyed.
///
/// No arguments, no return value (cdecl, plain `ret`).
lf_checker_rt::export!(cdecl, rw_009255c0() -> u32 {
    unsafe {
        const SLOTS: [u32; 4] = [0x119CFF0, 0x119CFF4, 0x119CFE8, 0x119CFEC];
        const KIND: u32 = 0x08;
        const REFCOUNT: u32 = 0x0A;
        for g in SLOTS {
            let slot = lf_checker_rt::global::<u32>(g);
            let obj = slot.read_unaligned();
            if obj == 0 {
                continue;
            }
            let count = (obj.wrapping_add(REFCOUNT) as *const u16).read_unaligned();
            if count != 0 {
                let new = count.wrapping_sub(1);
                (obj.wrapping_add(REFCOUNT) as *mut u16).write_unaligned(new);
                let kind = (obj.wrapping_add(KIND) as *const u8).read();
                if new == 0 && (kind == 2 || kind == 4) {
                    let vtable = (obj as *const u32).read_unaligned();
                    let target = (vtable as *const u32).read_unaligned();
                    let destroy: extern "stdcall" fn(u32) -> u32 =
                        core::mem::transmute(target as usize);
                    destroy(1);
                }
            }
            slot.write_unaligned(0);
        }
        0
    }
});
