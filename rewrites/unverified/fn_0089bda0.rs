// original: 0x0089bda0 audio_dual_slot_max (proposed)

/// Runs the slot operation on two slots and returns the SIGNED max of the answers.
///
/// `this` carries a bank byte at `+0x40` and two slot bytes at `+0x48`/`+0x49`.
/// Each slot selects a target (`TABLE[bank*0x6f40+0x6f10]+STRIDE*slot`, or
/// null for a 0xFF slot) on which the operation (callees 1 and 2, thiscall/1)
/// runs with a scratch out-flag byte; the flags start zeroed. When `out` is
/// non-null and either flag came back nonzero, `1` is stored to `out`. The
/// scratch bytes live in a padded C-layout struct so the word-granular
/// scripted writes land where the original's frame puts them. Returns the
/// greater of the two answers as SIGNED integers (the original uses `cmovg`).
///
/// Original: 0x0089bda0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0089bda0(this: u32, out: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SLOT0_OFF: u32 = 0x48;
        const SLOT1_OFF: u32 = 0x49;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const OP0: u32 = 1;
        const OP1: u32 = 2;
        #[repr(C)]
        struct Scratch {
            _pad: [u8; 8],
            s0: u8,
            s1: u8,
        }
        let mut scr = Scratch { _pad: [0; 8], s0: 0, s1: 0 };
        #[inline(always)]
        unsafe fn target(this: u32, slot_off: u32, table: u32, stride: u32, bank: u32) -> u32 {
            unsafe {
                let slot = (this as *const u8).byte_add(slot_off as usize).read() as u32;
                if slot == NO_SLOT {
                    return 0;
                }
                let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
                    .byte_add(ROW_SLOT as usize).read_unaligned();
                row.wrapping_add(stride.wrapping_mul(slot))
            }
        }
        let bank = (this as *const u8).byte_add(BANK_OFF as usize).read() as u32;
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let r0: u32 = lf_checker_rt::callee_thiscall!(
            OP0, u32, target(this, SLOT0_OFF, table, stride, bank),
            (&mut scr.s0 as *mut u8) as u32);
        let r1: u32 = lf_checker_rt::callee_thiscall!(
            OP1, u32, target(this, SLOT1_OFF, table, stride, bank),
            (&mut scr.s1 as *mut u8) as u32);
        if out != 0 && (scr.s0 != 0 || scr.s1 != 0) {
            (out as *mut u8).write(1);
        }
        // SIGNED max (original `cmovg`).
        if (r0 as i32) > (r1 as i32) { r0 } else { r1 }
    }
});
