// original: 0x009d09b0 locked_predicate_forward
/// Read a locked context word and forward it into a predicate call.
///
/// Returns whatever the predicate call returns.
export!(cdecl, rw_009d09b0() -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    let p: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let v = unsafe { (p.wrapping_add(8) as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    callee_cdecl!(4, u32, v)
});
