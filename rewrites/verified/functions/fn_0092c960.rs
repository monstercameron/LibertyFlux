// original: 0x0092C960 release_dead_ref_slots (proposed)

/// Release two global ref slots: drop the count, destroy at zero, clear the slot.
///
/// Each of the globals `SLOT_A`/`SLOT_B` holds either null or an object with
/// a type byte at `+0x08`, a 16-bit refcount at `+0x0a` and a vtable pointer
/// at `+0x00`. A live slot with a nonzero count has its count decremented;
/// when the count reaches zero and the type is 2 or 4, the slot-0 handler
/// (callee 1, thiscall) is invoked with `(obj, 1)`. Both slots are then
/// cleared to null unconditionally. Returns nothing meaningful.
///
/// Original: 0x0092C960 (cdecl, no arguments). Up to two indirect calls.
lf_checker_rt::export!(cdecl, rw_0092C960() -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x011A_1B8C;
        const SLOT_B: u32 = 0x011A_1B90;
        #[inline(always)]
        unsafe fn release(slot: u32) {
            unsafe {
                let obj = (lf_checker_rt::relocated(slot) as *const u32).read_unaligned();
                if obj != 0 {
                    let count = (obj.wrapping_add(0x0a) as *const u16).read_unaligned();
                    if count != 0 {
                        (obj.wrapping_add(0x0a) as *mut u16)
                            .write_unaligned(count.wrapping_sub(1));
                        let ty = (obj.wrapping_add(0x08) as *const u8).read();
                        let armed = ty == 2 || ty == 4;
                        let now = (obj.wrapping_add(0x0a) as *const u16).read_unaligned();
                        if now == 0 && armed {
                            let vt = (obj as *const u32).read_unaligned();
                            let fp = (vt as *const u32).read_unaligned();
                            let handler: extern "thiscall" fn(u32, u32) -> u32 =
                                core::mem::transmute(fp as usize);
                            handler(obj, 1);
                        }
                    }
                }
                (lf_checker_rt::relocated(slot) as *mut u32).write_unaligned(0);
            }
        }
        release(SLOT_A);
        release(SLOT_B);
        0
    }
});
