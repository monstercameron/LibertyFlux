// original: 0x009e9780 ped_can_activate
/// True when the object may activate: no lock flag (`+0x26C &
/// 0x2000` clear), no blocking mode (`+0xA74` of 1 or 2 while `+0x210`
/// is set), a live slot table (`+0x224`), the virtual gate at slot
/// `+0x128` silent or the mode at `+0xA70` moved on, and no live
/// blocker (`+0x6C` null or its byte at `+0xE` clear). (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e9780(this_ptr: u32) -> u32 {
    unsafe {
        const LOCK_OFF: u32 = 0x26C;
        const LOCK_BIT: u32 = 0x2000;
        const MODESET_OFF: u32 = 0x210;
        const MODE_OFF: u32 = 0xA74;
        const SLOTS_OFF: u32 = 0x224;
        const SLOT: u32 = 0x128;
        const GATE_MODE_OFF: u32 = 0xA70;
        const BLOCKER_OFF: u32 = 0x6C;
        const BLOCKER_FLAG: u32 = 0xE;
        if (this_ptr.wrapping_add(LOCK_OFF) as *const u32).read_unaligned() & LOCK_BIT != 0 {
            return 0;
        }
        if (this_ptr.wrapping_add(MODESET_OFF) as *const u8).read() != 0 {
            let mode = (this_ptr.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
            if mode == 1 || mode == 2 {
                return 0;
            }
        }
        if (this_ptr.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        if gate(this_ptr) & 0xFF != 0 {
            if (this_ptr.wrapping_add(GATE_MODE_OFF) as *const u32).read_unaligned() == 1 {
                return 0;
            }
        }
        let blocker = (this_ptr.wrapping_add(BLOCKER_OFF) as *const u32).read_unaligned();
        if blocker == 0 {
            return 1;
        }
        if (blocker.wrapping_add(BLOCKER_FLAG) as *const u8).read() != 0 {
            0
        } else {
            1
        }
    }
});
