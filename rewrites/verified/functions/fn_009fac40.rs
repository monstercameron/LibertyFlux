// original: 0x009fac40 CPlayStatFloat::vf3
/// Publish this float stat, then forward its value.
///
/// Runs the shared publish chain with the incoming argument as the chain
/// object and this object as the stat, then forwards the float stored in
/// this object bit-exact to the float sink. Any failed check aborts with
/// 0, else returns 1.
export!(thiscall, rw_009fac40(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        // Note the order: the incoming argument plays the chain's object
        // role while this object plays the stat role.
        if callee_cdecl!(1, u32, arg, this_ptr) & 0xFF == 0 {
            return 0;
        }
        let bits = (this_ptr as *const u32).add(0x34 / 4).read();
        u32::from(callee_thiscall!(2, u32, arg, bits) & 0xFF != 0)
    }
});
