// original: 0x0099DEB0 audio_release_handles_and_key (proposed)

/// Release the entity's two owned handles and its manager key.
///
/// Each of `this`+0x94 and `this`+0x98, when nonzero, is passed with a zero
/// word to the handle release (callee 1, thiscall/1) and then cleared. When
/// the key at `this`+0xA0 is not negative (SIGNED test) the manager revoke
/// (callee 2, thiscall/3 of the key, the argument and zero on the shared
/// audio manager) runs and the key is set to -1. `this`+0xC is always
/// cleared. A nonzero buffer at `this`+0x204 is freed (callee 3, cdecl/1)
/// and cleared. The return is the free's answer when it ran, else zero: the
/// original falls through with `eax` untouched, which the rewrite states
/// explicitly. Thiscall with one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_0099DEB0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE_A: u32 = 0x94;
        const HANDLE_B: u32 = 0x98;
        const KEY: u32 = 0xA0;
        const STATE: u32 = 0x0C;
        const BUFFER: u32 = 0x204;
        const MANAGER: u32 = 0x01288780;
        const RELEASE_CALLEE: u32 = 1;
        const REVOKE_CALLEE: u32 = 2;
        const FREE_CALLEE: u32 = 3;
        let ha = ((this.wrapping_add(HANDLE_A)) as *const u32).read_unaligned();
        if ha != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, ha, 0);
            ((this.wrapping_add(HANDLE_A)) as *mut u32).write_unaligned(0);
        }
        let hb = ((this.wrapping_add(HANDLE_B)) as *const u32).read_unaligned();
        if hb != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, hb, 0);
            ((this.wrapping_add(HANDLE_B)) as *mut u32).write_unaligned(0);
        }
        let key = ((this.wrapping_add(KEY)) as *const u32).read_unaligned();
        // Signed non-negative test, as the original's `js`.
        if (key as i32) >= 0 {
            lf_checker_rt::callee_thiscall!(REVOKE_CALLEE, u32,
                                            lf_checker_rt::relocated(MANAGER), key, arg, 0);
            ((this.wrapping_add(KEY)) as *mut u32).write_unaligned(0xFFFF_FFFF);
        }
        ((this.wrapping_add(STATE)) as *mut u32).write_unaligned(0);
        let buf = ((this.wrapping_add(BUFFER)) as *const u32).read_unaligned();
        if buf == 0 {
            return 0;
        }
        let ans = lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, buf);
        ((this.wrapping_add(BUFFER)) as *mut u32).write_unaligned(0);
        ans
    }
});
