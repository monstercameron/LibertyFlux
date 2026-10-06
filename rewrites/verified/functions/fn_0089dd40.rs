// original: 0x0089dd40 audio_slot_op_forward (proposed)

/// Forwards one operation to the voice slot selected by bank and slot bytes.
///
/// `obj` carries a bank byte at `+0x40` (`BANK_OFF`) and a slot byte at
/// `+0x48` (`SLOT_OFF`). A slot of 0xFF (`NO_SLOT`) returns 0 at once.
/// Otherwise the target is `TABLE[bank*ROW_STRIDE+ROW_SLOT] +
/// STRIDE*slot` with the table base and stride read from the globals
/// `TABLE_GLOB`/`STRIDE_GLOB`; a null target returns 0. The operation
/// (callee 1, thiscall/1) then runs on the target with `a1`, and its answer
/// is returned. The original redundantly re-tests the slot for 0xFF just
/// before the call; that branch is dead (0xFF already returned) and is not
/// reproduced.
///
/// Original: 0x0089dd40 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0089dd40(obj: u32, a1: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SLOT_OFF: u32 = 0x48;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const OP: u32 = 1;
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        let slot = rd8(obj + SLOT_OFF);
        if slot == NO_SLOT {
            return 0;
        }
        let bank = rd8(obj + BANK_OFF);
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(ROW_SLOT as usize).read_unaligned();
        let target = row.wrapping_add(stride.wrapping_mul(slot));
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(OP, u32, target, a1)
    }
});
