// original: 0x00DB8F20 UIMusicClip::release_audio_resources
/// Checks the low byte returned by the clip's virtual method at vtable offset
/// `0x140`. When it is nonzero, the method stops playback, releases each
/// nonnull resource pointer at offsets `0x1e4`, `0x1e0`, `0x1e8`, `0x1ec`,
/// `0x1f0`, and `0x1f4` in that order, clears each released slot, then calls
/// the virtual method at offset `0x13c` with a zero stack argument. The six
/// fields are pointer-like slots; their more specific meanings are unknown.
/// This is a 32-bit MSVC thiscall method with no incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_00db8f20(this_ptr: u32) -> () {
    const RESOURCE_FIELD_OFFSETS: [usize; 6] = [
        0x1e4, 0x1e0, 0x1e8, 0x1ec, 0x1f0, 0x1f4,
    ];
    const FINAL_METHOD_ARGUMENT: u32 = 0;

    let has_resources = lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
    if (has_resources & 0xff) == 0 {
        return;
    }

    let _ = lf_checker_rt::callee_thiscall!(3, u32, this_ptr);
    let object_bytes = this_ptr as *mut u8;
    for field_offset in RESOURCE_FIELD_OFFSETS {
        let field = unsafe { object_bytes.add(field_offset) as *mut u32 };
        let resource_pointer = unsafe { field.read_unaligned() };
        if resource_pointer != 0 {
            let _ = lf_checker_rt::callee_cdecl!(4, u32, resource_pointer);
            unsafe { field.write_unaligned(0) };
        }
    }

    let _ = lf_checker_rt::callee_thiscall!(2, u32, this_ptr, FINAL_METHOD_ARGUMENT);
});
