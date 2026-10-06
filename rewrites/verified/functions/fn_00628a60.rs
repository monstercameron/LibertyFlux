// original: 0x00628a60 lazy_init_tls_object_b (proposed)

/// Second instance of the lazy singleton template (see `rw_00628900`): same
/// shape, its own slot, constants and global table (sixteen entries).
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
/// Original: 0x00628a60 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00628a60() -> u32 {
    unsafe {
        const OBJ_SLOT: u32 = 0x018b74ac;
        const CTX_GLOBAL: u32 = 0x01bb5520;
        const SETUP_ARG: u32 = 0x01092810;
        const OBJ_PTR: u32 = 0x00f962a8;
        const OBJ_KIND: u32 = 0xd0;
        const OBJ_CB_A: u32 = 0x0062bd50;
        const OBJ_CB_B: u32 = 0x0043ead0;
        const INITS: [(u32, u32); 16] = [
            (0x010a3398, 0x10),
            (0x01084070, 0x20),
            (0x01084094, 0x30),
            (0x010a0fcc, 0x40),
            (0x010b1b44, 0x50),
            (0x0108c668, 0x60),
            (0x010af778, 0x61),
            (0x010b1b08, 0x70),
            (0x010b1b80, 0x80),
            (0x0109285c, 0x90),
            (0x010a3374, 0xa0),
            (0x0108a2e4, 0xb0),
            (0x010a0f90, 0xc0),
            (0x010af760, 0xc4),
            (0x0108a250, 0xc8),
            (0x01094be0, 0xcc),
        ];

        let slot = lf_checker_rt::global::<u32>(OBJ_SLOT);
        if slot.read() != 0 {
            return 0;
        }
        let inner = ((lf_checker_rt::tls_slot(0).wrapping_add(8)) as *const u32).read_unaligned();
        let vt = (inner as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((vt.wrapping_add(8)) as *const u32).read_unaligned()) as usize);
        let mem = alloc(inner, 0x40, 0x10, 0);
        let obj = if mem != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, mem)
        } else {
            0
        };
        slot.write(obj);
        ((obj.wrapping_add(4)) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJ_PTR));
        ((obj.wrapping_add(0x10)) as *mut u32).write_unaligned(OBJ_KIND);
        ((obj.wrapping_add(0x20)) as *mut u32).write_unaligned(0);
        ((obj.wrapping_add(0x24)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(OBJ_CB_A));
        ((obj.wrapping_add(0x28)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(OBJ_CB_B));
        for (addr, val) in INITS {
            (lf_checker_rt::global::<u32>(addr)).write(val);
        }
        ((obj.wrapping_add(8)) as *mut u32).write_unaligned(0);
        ((obj.wrapping_add(0x0c)) as *mut u32).write_unaligned(0);
        let flags = ((obj.wrapping_add(0x1c)) as *const u32).read_unaligned();
        ((obj.wrapping_add(0x1c)) as *mut u32).write_unaligned(flags & 0xffff0000);
        ((obj.wrapping_add(0x1e)) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(3, u32, obj, lf_checker_rt::relocated(SETUP_ARG));
        let ctx = (lf_checker_rt::global::<u32>(CTX_GLOBAL)).read().wrapping_add(0x18);
        let mut value = ((obj.wrapping_add(4)) as *const u32).read_unaligned();
        let mut cell = lf_checker_rt::relocated(OBJ_SLOT);
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            ctx,
            &mut value as *mut u32 as u32,
            &mut cell as *mut u32 as u32
        )
    }
});
