// original: 0x00bc8270 SWITCH_RANDOM_BOATS
/// Forward the coerced toggle to the boat-spawn engine function.
///
/// Stack-slot boolean coercion on the first argument.
export!(cdecl, rw_00bc8270(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let toggle = unsafe { *args.add(0) };
    // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
    let coerced = ((toggle != 0) as u32);
    callee_cdecl!(1, u32, coerced)
});
