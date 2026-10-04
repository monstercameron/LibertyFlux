// original: 0x0099deb0 entity_release_voice_handles
/// Releases the voice handles of an audio entity.
///
/// Frees the objects at offsets 0x94 and 0x98 through callee 1 when set
/// (nulling them), releases the non-negative handle at 0xa0 through callee 2
/// together with the argument (stamping -1), clears the word at offset 0xc,
/// and frees the pointer at 0x204 through callee 3 when set (nulling it).
export!(thiscall, rw_0099deb0(this: u32, a0: u32) -> () {
    unsafe {
        const MANAGER: u32 = 0x1288780;
        let p = *((this.wrapping_add(0x94)) as *const u32);
        if p != 0 {
            callee_thiscall!(1, u32, p, 0);
            *((this.wrapping_add(0x94)) as *mut u32) = 0;
        }
        let q = *((this.wrapping_add(0x98)) as *const u32);
        if q != 0 {
            callee_thiscall!(1, u32, q, 0);
            *((this.wrapping_add(0x98)) as *mut u32) = 0;
        }
        let h = *((this.wrapping_add(0xa0)) as *const i32);
        if h >= 0 {
            callee_thiscall!(2, u32, relocated(MANAGER), h as u32, a0, 0);
            *((this.wrapping_add(0xa0)) as *mut u32) = 0xffff_ffff;
        }
        *((this.wrapping_add(0x0c)) as *mut u32) = 0;
        let r = *((this.wrapping_add(0x204)) as *const u32);
        if r != 0 {
            callee_cdecl!(3, u32, r);
            *((this.wrapping_add(0x204)) as *mut u32) = 0;
        }
    }
});
