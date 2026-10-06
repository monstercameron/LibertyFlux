// original: 0x0089d4b0 audio_slot_retrigger (proposed)

/// Retriggers the voice slot selected by bank and slot bytes, then chains on.
///
/// `obj` carries a bank byte at `+0x40` and a slot byte at `+0x48`. A slot
/// of 0xFF returns 0xFF at once (the loaded slot value is still in EAX).
/// Otherwise the target is `TABLE[bank*0x6f40+0x6f10] + STRIDE*slot`; a
/// null target returns the table base itself (also still in EAX). Otherwise
/// the setup call (callee 1, thiscall/2) runs on the target with
/// (`[obj+0x54]`, 0) and the retrigger call (callee 2, thiscall/1) with
/// `a1`, and control tail-jumps to the chain routine (callee 3, thiscall/0
/// on `obj`), whose answer is returned.
///
/// Original: 0x0089d4b0 (cdecl, two stack words; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_0089d4b0(obj: u32, a1: u32) -> u32 {
    unsafe {
        const BANK_OFF: u32 = 0x40;
        const SLOT_OFF: u32 = 0x48;
        const PARAM_OFF: u32 = 0x54;
        const NO_SLOT: u32 = 0xff;
        const TABLE_GLOB: u32 = 0x0115d988;
        const STRIDE_GLOB: u32 = 0x0115d964;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_SLOT: u32 = 0x6f10;
        const SETUP: u32 = 1;
        const RETRIG: u32 = 2;
        const CHAIN: u32 = 3;
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        let slot = rd8(obj + SLOT_OFF);
        if slot == NO_SLOT {
            return NO_SLOT;
        }
        let bank = rd8(obj + BANK_OFF);
        let table = lf_checker_rt::global::<u32>(TABLE_GLOB).read_unaligned();
        let stride = lf_checker_rt::global::<u32>(STRIDE_GLOB).read_unaligned();
        let row = (table.wrapping_add(bank.wrapping_mul(ROW_STRIDE)) as *const u32)
            .byte_add(ROW_SLOT as usize).read_unaligned();
        let target = row.wrapping_add(stride.wrapping_mul(slot));
        if target == 0 {
            return table;
        }
        let param = (obj + PARAM_OFF as u32) as *const u32;
        let param = param.read_unaligned();
        lf_checker_rt::callee_thiscall!(SETUP, u32, target, param, 0);
        lf_checker_rt::callee_thiscall!(RETRIG, u32, target, a1);
        lf_checker_rt::callee_thiscall!(CHAIN, u32, obj)
    }
});
