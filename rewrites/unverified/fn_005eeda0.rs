// original: 0x005eeda0 input_ui_create_6arg

/// Allocate a holder through the six-argument constructor, then poll virtual slot 2 twice and fold the answers into bits 0x1FFC000 of the link word.
///
/// The two polled answers are SIGNED 32-bit values: each `% 16` and the final
/// `/ 16` truncate toward zero (the original's `and 0x8000000f`/`jns` and
/// `cdq` idioms), matching Rust's `%` and `/` on `i32`.
///
/// Original: fastcall (ecx ignored, edx plus 9 stack words (first 4 unread)); callee 1 is direct (`new`-like), callee 2
/// is the direct constructor call (the in-batch six-argument holder constructor), callee 3 is the object's virtual
/// slot 2 reached through its vtable; caller cleans up.
lf_checker_rt::export!(fastcall, rw_005eeda0(_ecx: u32, p0: u32, _s0: u32, _s1: u32, _s2: u32, _s3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const NEW_SIZE: u32 = 0xE4;
        const SLOT_OFF: u32 = 0x08;
        const LINK: u32 = 0x04;
        const FIELD_MASK: u32 = 0x1FFC000;
        const CB: u32 = 0x005EE090;
        let raw = lf_checker_rt::callee_cdecl!(1, u32, NEW_SIZE, 0);
        let obj = if raw == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, raw, CB, p0, a4, a5, a6, a7, a8)
        };
        let obj = core::hint::black_box(obj);
        let vt = ((obj) as *const u32).read_unaligned();
        let slot = ((vt + SLOT_OFF) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let v1 = f(obj);
        let r1 = (v1 as i32) % 16;
        let t = (16 - r1) % 16;
        let vt2 = ((obj) as *const u32).read_unaligned();
        let slot2 = ((vt2 + SLOT_OFF) as *const u32).read_unaligned();
        let g: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot2 as usize) };
        let v2 = g(obj);
        let q = (v2 as i32).wrapping_add(t) / 16;
        let link = ((obj + LINK) as *const u32).read_unaligned();
        let m = (((q << 14) as u32) ^ link) & FIELD_MASK;
        ((obj + LINK) as *mut u32).write_unaligned(link ^ m);
        m
    }
});
