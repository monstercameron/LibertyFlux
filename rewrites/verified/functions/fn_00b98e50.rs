// original: 0x00b98e50 NativeImpl_IS_IN_CAR_FIRE_BUTTON_PRESSED_2

/// Returns whether the in-car fire button is pressed for the current player.
///
/// Loads the player index from `IDX_GLOB` (-1 selects a null row and faults
/// on the row dereference, like the original), follows the player table at
/// `TABLE` and the row's inner object at `INNER_OFF`, and requires its flag
/// bit 2 at `FLAG_OFF`. The key-state block from `STATE_CALLEE` then
/// decides: with mask byte `KEY_MASK`, the answer is 1 exactly when the
/// first key byte xored with the mask exceeds 0x7F while the second does
/// not.
///
/// Upper return bytes are residue (the inner-object or key-state pointer),
/// reproduced exactly.
///
/// Original: 0x00B98E50 (cdecl, no stack arguments, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b98e50() -> u32 {
    unsafe {
        const IDX_GLOB: u32 = 0x01036F14;
        const TABLE: u32 = 0x011A8808;
        const INNER_OFF: u32 = 0x598;
        const FLAG_OFF: u32 = 0x26C;
        const FLAG_BIT: u8 = 4;
        const STATE_CALLEE: u32 = 1;
        const KEY_MASK: u32 = 0x28FC;
        const KEY_A: u32 = 0x28FE;
        const KEY_B: u32 = 0x28FF;
        const THRESH: u8 = 0x7F;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let idx = lf_checker_rt::global::<u32>(IDX_GLOB).read();
        let row = if idx == 0xFFFF_FFFF {
            0
        } else {
            rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)))
        };
        let inner = rd32(row.wrapping_add(INNER_OFF));
        if rd8(inner + FLAG_OFF) & FLAG_BIT == 0 {
            return inner & 0xFFFF_FF00;
        }
        let ks: u32 = lf_checker_rt::callee_cdecl!(STATE_CALLEE, u32,);
        let dl = rd8(ks + KEY_MASK);
        if (rd8(ks + KEY_A) ^ dl) <= THRESH {
            return ks & 0xFFFF_FF00;
        }
        if (rd8(ks + KEY_B) ^ dl) > THRESH {
            return ks & 0xFFFF_FF00;
        }
        (ks & 0xFFFF_FF00) | 1
    }
});
