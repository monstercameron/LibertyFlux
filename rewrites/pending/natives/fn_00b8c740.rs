// original: 0x00b8c740 GET_BLIP_INFO_ID_POSITION
/// Copy the target vector into context scratch and forward (blip, copy) on.
///
/// The handler stages the 3-word vector through the call context's scratch
/// area: it records the vector pointer at scratch word `idx + 4`, copies the
/// three words to scratch words `(idx + 2) * 4`, advances the scratch index,
/// and calls the engine function with the blip handle and the copy address.
/// All address arithmetic wraps mod 2^32 exactly like the original.
rt::export!(cdecl, rw_00b8c740(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    const SCRATCH_INDEX: u32 = 0x0C;
    const PTR_SLOT_BASE: u32 = 0x10;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let handle = unsafe { *args.add(0) };
    let vecp = unsafe { *args.add(1) };
    let idx = unsafe { *((ctx.wrapping_add(SCRATCH_INDEX)) as *const u32) };
    let y = unsafe { *((vecp.wrapping_add(4)) as *const u32) };
    let z = unsafe { *((vecp.wrapping_add(8)) as *const u32) };
    unsafe {
        *((ctx.wrapping_add(idx.wrapping_mul(4).wrapping_add(PTR_SLOT_BASE))) as *mut u32) =
            vecp;
    }
    let x = unsafe { *(vecp as *const u32) };
    let copy_at = ctx.wrapping_add(idx.wrapping_add(2).wrapping_mul(16));
    unsafe {
        *(copy_at as *mut u32) = x;
        *((copy_at.wrapping_add(4)) as *mut u32) = y;
        *((copy_at.wrapping_add(8)) as *mut u32) = z;
        *((ctx.wrapping_add(SCRATCH_INDEX)) as *mut u32) = idx.wrapping_add(1);
    }
    rt::callee_cdecl!(1, u32, handle, copy_at)
});
