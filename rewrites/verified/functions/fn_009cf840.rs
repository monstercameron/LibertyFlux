// original: 0x009cf840 ensure_context_block
/// Ensure the shared context block exists, allocating it when the slot is empty.
///
/// Returns 0 when the locked slot read is already non-null; otherwise
/// allocates a 0x20 block, stores it through a fresh context pointer, and
/// returns 0, or 0x80040901 when allocation fails.
export!(cdecl, rw_009cf840() -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    let p: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let v = unsafe { (p as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    if v != 0 {
        return 0;
    }
    let block: u32 = callee_stdcall!(4, u32, 0x20);
    if block == 0 {
        return 0x80040901;
    }
    let q: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    unsafe { (q as *mut u32).write(block) };
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    0
});
