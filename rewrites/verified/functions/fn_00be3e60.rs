// original: 0x00be3e60 CTaskComplexReact::vf18 (symbols)

/// Refresh the reaction target, priming its blend weight when it reacts.
///
/// Loads the target object at `this + TARGET_OFF` (0x8) and asks its kind
/// through its third virtual. When the kind is `REACT_KIND` (0xde) the prime
/// callee runs first with the incoming argument as object and the float 2.0
/// (by bits) as its argument. Then the refresh callee always runs with the
/// incoming argument as object and no arguments. Returns zero in all cases;
/// both callee answers are discarded.
///
/// Original: 0x00be3e60 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be3e60(this: u32, arg: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x08;
        const KIND_SLOT: u32 = 0x0c;
        const REACT_KIND: u32 = 0xde;
        const PRIME_WEIGHT_BITS: u32 = 0x40000000; // 2.0f
        const PRIME: u32 = 2;
        const REFRESH: u32 = 3;
        // The kind query runs through the target's own vtable (stub id 1 in
        // the contract), exactly like the original: no ctable use here.
        let target = (this.wrapping_add(TARGET_OFF) as *const u32).read_unaligned();
        let vtable = (target as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if kind_of(target) == REACT_KIND {
            lf_checker_rt::callee_thiscall!(PRIME, u32, arg, PRIME_WEIGHT_BITS);
        }
        lf_checker_rt::callee_thiscall!(REFRESH, u32, arg);
        0
    }
});
