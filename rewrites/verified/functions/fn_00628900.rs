// original: 0x00628900 lazy_init_tls_object_a (proposed)

/// Lazily create and register the singleton object behind `OBJ_SLOT`, once.
///
/// If the slot is already non-null, returns at once (the value left in the
/// return register is the incoming one, so the return channel is not part of
/// the proof). Otherwise asks the thread-local allocator (TLS slot 0,
/// `+8`, virtual slot 2) for a block, runs the initializer callee on it,
/// publishes the result in the slot, fills the object's header fields and a
/// table of global constants, runs the setup callee on the object, and
/// registers the slot with the context object. A null answer from the
/// allocator faults on the first header store, exactly like the original
/// (both sides fault identically). The two early-out compares are plain
/// equality (`jne` on the slot, `je` on the allocator's answer); no value a
/// callee returns is ever compared for signedness.
///
/// Original: 0x00628900 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00628900() -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;
        const SETUP: u32 = 3;
        const REGISTER: u32 = 4;
        const OBJ_SLOT: u32 = 0x018b74a8;
        const CTX_GLOBAL: u32 = 0x01bb5520;
        const SETUP_ARG: u32 = 0x0108a2a8;
        const OBJ_PTR: u32 = 0x00f9660c;
        const OBJ_KIND: u32 = 0x60;
        const OBJ_CB_A: u32 = 0x0062bd10;
        const OBJ_CB_B: u32 = 0x0062bd40;
        const CTX_OFF: u32 = 0x18;
        const INITS: [(u32, u32); 12] = [
            (0x010b1b2c, 4),
            (0x01082100, 8),
            (0x0108a280, 0x0c),
            (0x010b1ba4, 0x10),
            (0x010af790, 0x14),
            (0x010a0fa8, 0x18),
            (0x0108a268, 0x20),
            (0x010a33bc, 0x30),
            (0x010a3350, 0x40),
            (0x0108a298, 0x50),
            (0x010b1b68, 0x54),
            (0x01094bf8, 0x58),
        ];

        let slot = lf_checker_rt::global::<u32>(OBJ_SLOT);
        if slot.read() != 0 {
            return 0;
        }
        // TLS slot 0 -> +8 -> vtable -> slot 2, called as thiscall(0x40,0x10,0).
        let inner = ((lf_checker_rt::tls_slot(0).wrapping_add(8)) as *const u32).read_unaligned();
        let vt = (inner as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((vt.wrapping_add(8)) as *const u32).read_unaligned()) as usize);
        let mem = alloc(inner, 0x40, 0x10, 0);
        let obj = if mem != 0 {
            lf_checker_rt::callee_thiscall!(INIT, u32, mem)
        } else {
            0
        };
        slot.write(obj);
        ((obj.wrapping_add(4)) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJ_PTR));
        ((obj.wrapping_add(0x10)) as *mut u32).write_unaligned(OBJ_KIND);
        ((obj.wrapping_add(0x20)) as *mut u32).write_unaligned(0);
        ((obj.wrapping_add(0x24)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(OBJ_CB_A));
        ((obj.wrapping_add(0x28)) as *mut u32).write_unaligned(0);
        ((obj.wrapping_add(0x2c)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(OBJ_CB_B));
        for (addr, val) in INITS {
            (lf_checker_rt::global::<u32>(addr)).write(val);
        }
        ((obj.wrapping_add(8)) as *mut u32).write_unaligned(0);
        ((obj.wrapping_add(0x0c)) as *mut u32).write_unaligned(0);
        let flags = ((obj.wrapping_add(0x1c)) as *const u32).read_unaligned();
        ((obj.wrapping_add(0x1c)) as *mut u32).write_unaligned(flags & 0xffff0000);
        ((obj.wrapping_add(0x1e)) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(SETUP, u32, obj, lf_checker_rt::relocated(SETUP_ARG));
        let ctx = (lf_checker_rt::global::<u32>(CTX_GLOBAL)).read().wrapping_add(CTX_OFF);
        let mut value = ((obj.wrapping_add(4)) as *const u32).read_unaligned();
        let mut cell = lf_checker_rt::relocated(OBJ_SLOT);
        lf_checker_rt::callee_thiscall!(
            REGISTER,
            u32,
            ctx,
            &mut value as *mut u32 as u32,
            &mut cell as *mut u32 as u32
        )
    }
});
