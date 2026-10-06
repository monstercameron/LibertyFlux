// original: 0x005eee30 input_ui_create_inline

/// Fill a 0x1c-byte holder inline (vtable, masked link word, helper address, dereferenced head word and three dereferenced words), then poll virtual slot 2 twice and fold the answers into bits 0x1FFC000 of the link word.
///
/// The two polled answers are SIGNED 32-bit values: each `% 16` and the final
/// `/ 16` truncate toward zero (the original's `and 0x8000000f`/`jns` and
/// `cdq` idioms), matching Rust's `%` and `/` on `i32`.
///
/// Original: fastcall (ecx ignored, edx plus 3 stack words); callee 1 is direct (`new`-like), callee 2
/// is the direct constructor call (absent here: the holder is filled inline, and the virtual slot is reached through the real vtable hooked per trial), callee 3 is the object's virtual
/// slot 2 reached through its vtable; caller cleans up.
lf_checker_rt::export!(fastcall, rw_005eee30(_ecx: u32, p0: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const NEW_SIZE: u32 = 0x1c;
        const SLOT_OFF: u32 = 0x08;
        const LINK: u32 = 0x04;
        const FIELD_MASK: u32 = 0x1FFC000;
        const VTABLE: u32 = 0x00FE1534;
        const FNPTR: u32 = 0x08;
        const HEAD: u32 = 0x0C;
        const W0: u32 = 0x10;
        const W1: u32 = 0x14;
        const W2: u32 = 0x18;
        const FNHELPER: u32 = 0x005EE2F0;
        const COUNTER: u32 = 0x010327A0;
        const LINK_MASK: u32 = 0x3FFF;
        let raw = lf_checker_rt::callee_cdecl!(1, u32, NEW_SIZE, 0);
        let obj = if raw == 0 {
            0
        } else {
            let counter = (lf_checker_rt::global::<u32>(COUNTER)).read_unaligned();
            let link0 = ((raw + LINK) as *const u32).read_unaligned();
            (raw as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
            ((raw + LINK) as *mut u32)
                .write_unaligned(link0 ^ ((link0 ^ counter) & LINK_MASK));
            ((raw + FNPTR) as *mut u32).write_unaligned(lf_checker_rt::relocated(FNHELPER));
            ((raw + HEAD) as *mut u32)
                .write_unaligned(((p0) as *const u32).read_unaligned());
            (lf_checker_rt::global::<u32>(COUNTER)).write_unaligned(counter.wrapping_add(1));
            ((raw + W0) as *mut u32).write_unaligned((a0 as *const u32).read_unaligned());
            ((raw + W1) as *mut u32).write_unaligned((a1 as *const u32).read_unaligned());
            ((raw + W2) as *mut u32).write_unaligned((a2 as *const u32).read_unaligned());
            raw
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
