// original: 0x00892A00 audsound_teardown_voice
/// Tears down the voice's owned objects under the object lock.
///
/// Takes the lock callee (cdecl, the dword at `this+0x38`). When the dword
/// at `this+0x94` is nonzero, calls the release callee (thiscall, no stack
/// words) on it, re-reads `this+0x94`, and when still nonzero calls the
/// detach callee (thiscall, no stack words) on it and frees it with the free
/// callee (cdecl, one stack word); then zeroes `this+0x94`. (The re-read can
/// only differ if the release callee rewrites the caller's field; the stub
/// cannot, so that null path never fires.) When the dword at `this+0x98` is
/// nonzero, frees it the same way and zeroes `this+0x98`. Releases the lock
/// and returns the unlock callee's answer.
/// Original: 0x00892A00 (thiscall, no stack arguments).
export!(thiscall, rw_00892A00(this: *mut u8) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        const RELEASE: u32 = 3;
        const DETACH: u32 = 4;
        const FREE: u32 = 5;
        const MUTEX: usize = 0x38;
        let m = *(this.add(MUTEX) as *const u32);
        let _: u32 = callee_cdecl!(LOCK, u32, m);
        let o = *(this.add(0x94) as *const u32);
        if o != 0 {
            let _: u32 = callee_thiscall!(RELEASE, u32, o);
            let o2 = *(this.add(0x94) as *const u32);
            if o2 != 0 {
                let _: u32 = callee_thiscall!(DETACH, u32, o2);
                let _: u32 = callee_cdecl!(FREE, u32, o2);
            }
            *(this.add(0x94) as *mut u32) = 0;
        }
        let q = *(this.add(0x98) as *const u32);
        if q != 0 {
            let _: u32 = callee_cdecl!(FREE, u32, q);
            *(this.add(0x98) as *mut u32) = 0;
        }
        callee_cdecl!(UNLOCK, u32, m)
    }
});
