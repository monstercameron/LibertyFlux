// original: 0x00ba29f0 UNLOCK_RAGDOLL
/// Forward (char, coerced flag) to the ragdoll engine function.
///
/// Stack-slot boolean coercion on the second argument.
export!(cdecl, rw_00ba29f0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    let flag = unsafe { *args.add(1) };
    // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
    let coerced = ((flag != 0) as u32);
    callee_cdecl!(1, u32, ch, coerced)
});
