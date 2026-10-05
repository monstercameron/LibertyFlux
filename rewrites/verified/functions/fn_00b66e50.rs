// original: 0x00B66E50 CHeli::vf0
/// helicopter deleting-destructor wrapper (scalar form): base teardown, maybe free.
///
/// Calls the base teardown (stubbed, thiscall/0). When the low bit of the flag
/// argument is set, reads the allocator pool from its global and frees `this`
/// (stubbed, thiscall/1). Returns `this`. The pool read uses an unrelocated
/// absolute address, so trials only cover the non-deleting path (flag bit
/// clear). Thiscall, one stack word.
export!(thiscall, rw_00b66e50(this: u32, a0: u32) -> u32 {
    unsafe {
        const POOL: u32 = 0x12E22A4;
        let _: u32 = callee_thiscall!(1, u32, this);
        if (a0 & 0xFF) & 1 == 0 {
            return this;
        }
        let pool = *global::<u32>(POOL);
        let _: u32 = callee_thiscall!(2, u32, pool, this);
        this
    }
});
