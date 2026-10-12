// original: 0x00C0E8F0 UIMusicContainer::vf112
/// Query the first music object and return the container method's full result
/// when its low byte is nonzero. Otherwise query a second object and tail-return the
/// container's fallback method result. The singleton address and the two
/// string addresses are relocated image virtual addresses; both object-method
/// calls and the fallback tail call use checker callee stand-ins. `this` is
/// passed in ECX, the lookup helper cleans each string argument, and EAX is
/// returned unchanged from the selected container method.
lf_checker_rt::export!(thiscall, rw_c0e8f0(this: u32) -> u32 {
    unsafe {
        const SINGLETON_VA: u32 = 0x0198_1a4c;
        const FIRST_NAME_VA: u32 = 0x00ec_0be4;
        const FALLBACK_NAME_VA: u32 = 0x00ec_0954;
        let singleton = lf_checker_rt::relocated(SINGLETON_VA);
        let first_object = lf_checker_rt::callee_thiscall!(
            1, u32, singleton, lf_checker_rt::relocated(FIRST_NAME_VA)
        );
        let _ = lf_checker_rt::callee_thiscall!(2, u32, first_object);
        let status = lf_checker_rt::callee_thiscall!(3, u32, this);
        if (status as u8) != 0 {
            return status;
        }
        let fallback_object = lf_checker_rt::callee_thiscall!(
            1, u32, singleton, lf_checker_rt::relocated(FALLBACK_NAME_VA)
        );
        let _ = lf_checker_rt::callee_thiscall!(2, u32, fallback_object);
        lf_checker_rt::callee_thiscall!(4, u32, this)
    }
});
