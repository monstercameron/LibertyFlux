// original: 0x009a2970 native_impl_set_ambient_voice_name
/// Sets the ambient voice name of a speech entity.
///
/// Unless the slot at offset 0x9c already holds the argument, the old voice
/// is released through callees 1 and 2 and, when a previous voice was set,
/// detached through callees 1 and 3. The slot is then updated, validated
/// through callee 4 (falling back to a global voice on a zero answer), and
/// the resulting id from callee 5 is stored at offset 0xc8.
export!(thiscall, rw_009a2970(this: u32, a0: u32) -> () {
    unsafe {
        const MANAGER: u32 = 0x1288780;
        const FALLBACK: u32 = 0x12844b4;
        if *((this.wrapping_add(0x9c)) as *const u32) != a0 {
            let r1 = callee_thiscall!(1, u32, this, 1, a0);
            callee_thiscall!(2, u32, relocated(MANAGER), r1);
            let cur = *((this.wrapping_add(0x9c)) as *const u32);
            if cur != 0 {
                let r1b = callee_thiscall!(1, u32, this, 1, cur);
                callee_thiscall!(3, u32, relocated(MANAGER), r1b);
            }
        }
        *((this.wrapping_add(0x9c)) as *mut u32) = a0;
        if (callee_cdecl!(4, u32, a0) as u8) == 0 {
            *((this.wrapping_add(0x9c)) as *mut u32) = *global::<u32>(FALLBACK);
        }
        let v = *((this.wrapping_add(0x9c)) as *const u32);
        let r5 = callee_thiscall!(5, u32, relocated(MANAGER), v);
        *((this.wrapping_add(0xc8)) as *mut u16) = r5 as u16;
    }
});
