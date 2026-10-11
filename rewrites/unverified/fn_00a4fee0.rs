// original: 0x00a4fee0 vehicle_init_slots (proposed)

/// Initialise all 256 slots, clear the table state, report done.
///
/// Each slot address (`this + 0x100` stepping 0xe0) goes to the slot-init
/// callee in ecx with eight zero words, a scratch word and 0x64 on the
/// stack; the callee never cleans up (the frame pointer restores the stack
/// at the end), so the stub is declared noclean and the scratch word is
/// skipped. Afterwards the counter at +0x12104 is zeroed, +0xe100 takes
/// 0xf423f, 0x1000 words at +0xe104 are cleared, and the done callee runs.
/// 257 calls per trial, hence the raised call-log cap. Thiscall, one stack
/// word (unread), two callees, 1 in eax.
lf_checker_rt::export!(thiscall, rw_00a4fee0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const DONE: u32 = 2;
        const SLOTS_BASE: u32 = 0x100;
        const SLOT_STRIDE: u32 = 0xe0;
        const NSLOTS: u32 = 0x100;
        const RATE: u32 = 0x64;
        const MAGIC: u32 = 0xf423f;
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let mut slot = this.wrapping_add(SLOTS_BASE);
        let mut left = NSLOTS;
        loop {
            let mut scratch = [0u32; 2];
            let frame = (&mut scratch as *mut u32) as u32;
            lf_checker_rt::callee_thiscall!(INIT, u32, slot, 0, frame, 0, 0, 0, 0, 0, RATE, 0);
            slot = slot.wrapping_add(SLOT_STRIDE);
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
        wr32(this.wrapping_add(0x12104), 0);
        wr32(this.wrapping_add(0xe100), MAGIC);
        let mut p = this.wrapping_add(0xe104);
        let mut n: u32 = 0x1000;
        while n != 0 {
            wr32(p, 0);
            p = p.wrapping_add(4);
            n = n.wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(DONE, u32,);
        1
    }
});
