// original: 0x009d08a0 select_locked_field
/// Return one of two locked context fields selected by a predicate call.
///
/// The predicate's low byte selects the word at offset 0x238 (nonzero)
/// or 0x234 (zero) of a fresh context pointer.
export!(cdecl, rw_009d08a0() -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    let t: u32 = callee_cdecl!(3, u32,);
    let p: u32 = callee_cdecl!(4, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let off = if (t & 0xff) != 0 { 0x238 } else { 0x234 };
    let v = unsafe { (p.wrapping_add(off) as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    v
});
