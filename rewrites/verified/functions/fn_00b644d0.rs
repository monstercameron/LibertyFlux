// original: 0x00B644D0 veh_promote_detach
/// Promote the reference at `[a0]`, then detach it and reset.
///
/// Does nothing but the reset helper (stubbed, thiscall/0) when `[a0]` is
/// null. When the marker byte at +0x22a is not 2 and flag 0x200000 at +0x24
/// is clear, invokes virtual slots 0x114 (argument 8) and 0xb0 (planted
/// stubs), stamps marker 3, count 1, and clears flag bits 0 and 5. Otherwise
/// checks the auxiliary object at +0x38 (virtual slot 0xac when its word at
/// +8 is 0xffff) and invokes slot 0x114 with 0xb. Detaches (stubbed,
/// stdcall/1), zeroes `[a0]`, and runs the reset helper. Thiscall, one word.
export!(thiscall, rw_00b644d0(this: u32, a0: u32) -> u32 {
    unsafe {
        const READY: u32 = 0x22a;
        const FLAGS: u32 = 0x24;
        const COUNT: u32 = 0x224;
        const AUX: u32 = 0x38;
        const CODE: u8 = 2;
        const ALT: u32 = 0x200000;
        let obj = (a0 as *const u32).read_unaligned();
        if obj == 0 {
            let _: u32 = callee_thiscall!(5, u32, this);
            return 0;
        }
        let ready = ((obj + READY) as *const u8).read() == CODE;
        let alt = ((obj + FLAGS) as *const u32).read_unaligned() & ALT != 0;
        if !(ready || alt) {
            let vt = (obj as *const u32).read_unaligned();
            let f1: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((((vt + 0x114) as *const u32).read_unaligned()) as usize);
            let _ = f1(obj, 8);
            let obj = (a0 as *const u32).read_unaligned();
            let vt = (obj as *const u32).read_unaligned();
            let f2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((((vt + 0xb0) as *const u32).read_unaligned()) as usize);
            let _ = f2(obj);
            let obj = (a0 as *const u32).read_unaligned();
            ((obj + READY) as *mut u8).write(3);
            ((obj + COUNT) as *mut u32).write_unaligned(1);
            let f = ((obj + FLAGS) as *const u32).read_unaligned();
            ((obj + FLAGS) as *mut u32).write_unaligned(f & 0xfffffffe);
            let f = ((obj + FLAGS) as *const u32).read_unaligned();
            ((obj + FLAGS) as *mut u32).write_unaligned(f & 0xffffffdf);
        } else {
            let aux = ((obj + AUX) as *const u32).read_unaligned();
            if ((aux + 8) as *const u16).read_unaligned() == 0xffff {
                let vt = (obj as *const u32).read_unaligned();
                let f3: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute((((vt + 0xac) as *const u32).read_unaligned()) as usize);
                let _ = f3(obj);
            }
            let obj = (a0 as *const u32).read_unaligned();
            let vt = (obj as *const u32).read_unaligned();
            let f1: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((((vt + 0x114) as *const u32).read_unaligned()) as usize);
            let _ = f1(obj, 0xb);
        }
        let obj = (a0 as *const u32).read_unaligned();
        if obj != 0 {
            let _: u32 = callee_stdcall!(4, u32, a0);
        }
        (a0 as *mut u32).write_unaligned(0);
        let _: u32 = callee_thiscall!(5, u32, this);
        0
    }
});
