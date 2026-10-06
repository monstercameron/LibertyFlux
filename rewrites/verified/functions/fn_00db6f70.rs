// original: 0x00DB6F70 UIFontString::vf115

/// Reset the string state from the given metrics unless the guard hook vetoes.
///
/// `thiscall`, four stack words (size float bits, source pointer, two tag
/// words). The guard hook at vtable `+0x140` (thiscall, no stack args) vetoes
/// everything when its low byte is nonzero. Otherwise: store the size at
/// `+0x1E0`, forward the global default float through the slot at `+0x94`
/// (thiscall, one word), store the tags at `+0x1D8`/`+0x1D4`, the constant 7
/// at `+0x1F4`, the dereferenced source word at `+0x1DC`, reset the scale and
/// flag fields (`+0x1E4`/`+0x1EC` to 1.0, `+0x1E8` to 0, bytes `+0x208..0x20D`
/// to 0), notify slot `+0x14` with 1, run the backend (cdecl: 2, 0, pointer
/// to `+0x200`, 0; the pointer is skipped with a 4-word snapshot), and notify
/// slot `+0x13C` with 1. Void.
lf_checker_rt::export!(thiscall, rw_00db6f70(this_ptr: u32, size_bits: u32, src_ptr: u32, tag_a: u32, tag_b: u32) -> u32 {
    const DEFAULT_FLOAT: u32 = 0x010576F8;
    const ONE_BITS: u32 = 0x3F800000;
    unsafe {
        let vtable = (this_ptr as *const u32).read_unaligned();
        let call0 = |slot: u32| unsafe {
            let target = ((vtable + slot) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this_ptr)
        };
        let call1 = |slot: u32, arg: u32| unsafe {
            let target = ((vtable + slot) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(this_ptr, arg)
        };
        if call0(0x140) & 0xFF != 0 {
            return 0;
        }
        ((this_ptr + 0x1E0) as *mut u32).write_unaligned(size_bits);
        call1(0x94, lf_checker_rt::global::<u32>(DEFAULT_FLOAT).read());
        ((this_ptr + 0x1D8) as *mut u32).write_unaligned(tag_a);
        ((this_ptr + 0x1D4) as *mut u32).write_unaligned(tag_b);
        ((this_ptr + 0x1F4) as *mut u32).write_unaligned(7);
        let w = (src_ptr as *const u32).read_unaligned();
        ((this_ptr + 0x1DC) as *mut u32).write_unaligned(w);
        ((this_ptr + 0x1E4) as *mut u32).write_unaligned(ONE_BITS);
        ((this_ptr + 0x20B) as *mut u16).write_unaligned(0);
        ((this_ptr + 0x208) as *mut u16).write_unaligned(0);
        ((this_ptr + 0x20D) as *mut u8).write(0);
        ((this_ptr + 0x20A) as *mut u8).write(0);
        ((this_ptr + 0x1E8) as *mut u32).write_unaligned(0);
        ((this_ptr + 0x1EC) as *mut u32).write_unaligned(ONE_BITS);
        call1(0x14, 1);
        lf_checker_rt::callee_cdecl!(5, u32, 2, 0, this_ptr + 0x200, 0);
        call1(0x13C, 1);
    }
    0
});
