// original: 0x00bc6c50 IS_GARAGE_OPEN
use lf_k2_rt::{callee_cdecl, export};
// IS_GARAGE_OPEN: forward the garage id; store the answer's low byte
// through the return slot. Returns the slot.
export!(cdecl, rw_00BC6C50(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let open = callee_cdecl!(1, u32, *a) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = open;
        slot as u32
    }
});
