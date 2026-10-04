// original: 0x009d0820 ensure_and_read_slot
/// Ensure the shared context exists, then return its locked slot value.
export!(cdecl, rw_009d0820() -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    callee_cdecl!(3, u32,);
    let p: u32 = callee_cdecl!(4, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let v = unsafe { (p as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    v
});
