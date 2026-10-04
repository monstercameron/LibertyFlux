// original: 0x00a019e0 SET_ALL_PICKUPS_OF_TYPE_COLLECTABLE_BY_CAR
/// Forward (pickup-type, coerced flag) to the pickup engine function.
///
/// Stack-slot boolean coercion: pushed dword is the context address with
/// its low byte replaced by (flag != 0).
export!(cdecl, rw_00a019e0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ptype = unsafe { *args.add(0) };
    let flag = unsafe { *args.add(1) };
    // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
    let coerced = ((flag != 0) as u32);
    callee_cdecl!(1, u32, ptype, coerced)
});
