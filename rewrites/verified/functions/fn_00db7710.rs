// original: 0x00DB7710 UIFontString_ctor_like

/// Construct: forward both arguments to the base constructor (thiscall, two
/// stack words), install the vtable pointer, and initialise the state
/// words (two all-ones tags at `+0x1F8`/`+0x218`, zeros elsewhere).
///
/// Proposed name: unnamed in symbols; the shape (base call, vtable store,
/// field initialisation, returns this) is a constructor. `thiscall`.
lf_checker_rt::export!(thiscall, rw_00db7710(this_ptr: u32, arg0: u32, arg1: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this_ptr, arg0, arg1);
    unsafe {
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(0x00EF34F4));
        for off in [
            0x1E4u32, 0x1E0, 0x1E8, 0x1EC, 0x1F0, 0x1F4, 0x1FC, 0x204, 0x200,
            0x208, 0x210, 0x21C,
        ] {
            ((this_ptr + off) as *mut u32).write_unaligned(0);
        }
        for off in [0x1F8u32, 0x218] {
            ((this_ptr + off) as *mut u32).write_unaligned(0xFFFFFFFF);
        }
        ((this_ptr + 0x214) as *mut u8).write(0);
    }
    this_ptr
});
