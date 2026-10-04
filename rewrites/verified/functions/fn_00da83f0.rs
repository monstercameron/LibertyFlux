// original: 0x00da83f0 flee_event_dispatch
/// Dispatch a flee-task event to its handler.
///
/// Event 0xCB draws a random roll, scales it into a 0-99 range and forwards
/// it with fixed weights; event 0x38E stamps the task, copies the current
/// target block into it and forwards the stored parameters; any other event
/// answers 0. A missing manager also answers 0 on both live events.
export!(thiscall, rw_00da83f0(this: u32, ev: u32) -> u32 {
    unsafe {
        /// Handled event codes.
        const EV_ROLL: u32 = 0xCB;
        const EV_FLEE: u32 = 0x38E;
        /// Global slots: clock and manager.
        const CLOCK: u32 = 0x011735B4;
        const MANAGER: u32 = 0x0167E2A0;
        /// Roll scaling: 16-bit roll times 2^-15 times 50.
        const ROLL_SCALE1: f32 = f32::from_bits(0x38000000);
        const ROLL_SCALE2: f32 = 50.0;
        /// Fixed weight forwarded with the roll.
        const ROLL_W: u32 = 0x41000000;
        /// Task fields touched by the flee event.
        const PREV: u32 = 0x3c;
        const STAMP: u32 = 0x48;
        const STAMP_PREV: u32 = 0x4c;
        const STAMP_FLAG: u32 = 0x50;
        const TARGET_LINK: u32 = 0x14;
        const TARGET_BLOCK: u32 = 0x20;
        if ev == EV_ROLL {
            let mgr: u32 = callee_thiscall!(1, u32, global::<u32>(MANAGER).read());
            if mgr == 0 {
                return 0;
            }
            let r: u32 = callee_cdecl!(3, u32,);
            // Same order as the original's mulss chain; the product stays in
            // 0..100 so the truncation below never overflows.
            let x = ((r & 0xFFFF) as f32) * ROLL_SCALE1 * ROLL_SCALE2;
            let n = ({ let __x = (x); if __x.is_nan() || __x >= 2147483648.0 || __x < -2147483648.0 { 0x80000000 } else { __x as i32 as u32 } });
            callee_thiscall!(4, u32, mgr, n, 0, 0, ROLL_W)
        } else if ev == EV_FLEE {
            ((this.wrapping_add(STAMP)) as *mut u32).write_unaligned(global::<u32>(CLOCK).read());
            ((this.wrapping_add(STAMP_PREV)) as *mut u32).write_unaligned(((this.wrapping_add(PREV)) as *const u32).read_unaligned());
            (*((this.wrapping_add(STAMP_FLAG)) as *mut u8) = (1));
            let b = ((this.wrapping_add(TARGET_LINK)) as *const u32).read_unaligned();
            let inner = ((b.wrapping_add(0x20)) as *const u32).read_unaligned();
            let src = if inner != 0 {
                inner.wrapping_add(0x30)
            } else {
                b.wrapping_add(0x10)
            };
            let dst = this.wrapping_add(TARGET_BLOCK);
            ((dst) as *mut u32).write_unaligned(((src) as *const u32).read_unaligned());
            ((dst.wrapping_add(4)) as *mut u32).write_unaligned((f32::from_bits(((src.wrapping_add(4)) as *const u32).read_unaligned())).to_bits());
            ((dst.wrapping_add(8)) as *mut u32).write_unaligned((f32::from_bits(((src.wrapping_add(8)) as *const u32).read_unaligned())).to_bits());
            ((dst.wrapping_add(0x0c)) as *mut u32).write_unaligned(((src.wrapping_add(0x0c)) as *const u32).read_unaligned());
            let mgr: u32 = callee_thiscall!(1, u32, global::<u32>(MANAGER).read());
            if mgr == 0 {
                return 0;
            }
            callee_thiscall!(
                2,
                u32,
                mgr,
                dst,
                (*((this.wrapping_add(0x38)) as *const u8)) as u32,
                f32::from_bits(((this.wrapping_add(0x34)) as *const u32).read_unaligned()).to_bits(),
                ((this.wrapping_add(0x30)) as *const u32).read_unaligned(),
                (*((this.wrapping_add(0x39)) as *const u8)) as u32
            )
        } else {
            0
        }
    }
});
