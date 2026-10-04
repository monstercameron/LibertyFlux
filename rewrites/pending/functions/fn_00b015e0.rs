// original: 0x00b015e0 T_CB_Generic_4Args<void(*)(int, bool, bool, bool), int, bool, bool, bool>::vf1
/// Fire the stored four-argument callback with the captured values.
export!(thiscall, rw_00b015e0(this: u32) -> u32 {
    unsafe {
        let callback: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((this + 8) as *const u32));
        callback(
            *((this + 0xc) as *const u32),
            *((this + 0x10) as *const u8) as u32,
            *((this + 0x11) as *const u8) as u32,
            *((this + 0x12) as *const u8) as u32,
        )
    }
});
