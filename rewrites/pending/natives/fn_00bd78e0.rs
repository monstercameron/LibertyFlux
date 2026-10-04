// original: 0x00bd78e0 FIND_NETWORK_RESTART_POINT
use lf_k2_rt::{callee_cdecl, export};
/// Copy a three-word restart record (one id word plus two floats) from
/// the record addressed by the first script argument into a scratch
/// record inside the call context, file the record pointer in the
/// context's pointer table, bump the context's record count, and call
/// the engine with the scratch record plus the remaining two script
/// arguments. All copies are bitwise; no floating-point arithmetic.
export!(cdecl, rw_00bd78e0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let n = *ctx.add(3) as usize;
        let rec = *args as *const u32;
        let base = ctx as usize;
        *(base.wrapping_add(0x10).wrapping_add(n.wrapping_mul(4)) as *mut u32) =
            rec as u32;
        let dst =
            base.wrapping_add(32).wrapping_add(n.wrapping_mul(16)) as *mut u32;
        *dst = *rec;
        *dst.add(1) = *rec.add(1);
        *dst.add(2) = *rec.add(2);
        *(ctx as *mut u32).add(3) = n.wrapping_add(1) as u32;
        callee_cdecl!(1, u32, dst as u32, *args.add(1), *args.add(2));
        0
    }
});
