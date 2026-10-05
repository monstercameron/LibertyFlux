// original: 0x00c24340 CCamInterp::vf0 (symbols)
/// Deleting destructor: tear the object down through the shared destructor,
/// then free it through the heap context from 0x012fb1a0 when the low bit
/// of `flags` is set. Returns the object pointer.
///
/// Original: 0x00c24340 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c24340(obj: u32, flags: u32) -> u32 {
    unsafe {
        const HEAP_CTX: u32 = 0x012fb1a0;
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        if flags & 1 != 0 {
            let ctx = (lf_checker_rt::relocated(HEAP_CTX) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, ctx, obj);
        }
        obj
    }
});
