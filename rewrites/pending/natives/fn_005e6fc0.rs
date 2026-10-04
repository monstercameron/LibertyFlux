// original: 0x005e6fc0 CAN_PHONE_BE_SEEN_ON_SCREEN
use lf_k2_rt::{callee_addr, export, global, relocated};
// CAN_PHONE_BE_SEEN_ON_SCREEN: look up the phone object through a global
// index and pointer table, ask the engine whether it is blocked, and report
// the negation (1 = visible) through the return slot. Returns the slot.
export!(cdecl, rw_005E6FC0(ctx: u32) -> u32 {
    unsafe {
        let index = *global::<u32>(0x118EF9C);
        let table = relocated(0x118E7F8) as *const u32;
        let entry = *table.add(index as usize);
        let is_blocked: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        // The original tests only the low byte of the answer.
        let visible = u32::from(is_blocked(entry) & 0xFF == 0);
        let slot = ret_slot(ctx);
        *slot = visible;
        slot as u32
    }
});
