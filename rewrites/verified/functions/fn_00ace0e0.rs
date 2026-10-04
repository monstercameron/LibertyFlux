// original: 0x00ace0e0 copy_src_block_to_dst
/// Copy a 16-byte field block between objects when the destination exists.
///
/// Reads a source object pointer and a destination pointer from the stack,
/// and copies four dwords from 0x40 above the source to the destination.
/// When the destination is null nothing happens. The returned dword is the
/// last word copied (left in the return register by the original).
export!(cdecl, rw_00ace0e0(
    _a0: u32,
    src: *const u8,
    _a2: u32,
    _a3: u32,
    _a4: u32,
    _a5: u32,
    _a6: u32,
    dst: *mut u8,
) -> u32 {
    unsafe {
        if dst.is_null() {
            return 0;
        }
        let s = src.add(0x40) as *const u32;
        let d = dst as *mut u32;
        *d = *s;
        *d.add(1) = *s.add(1);
        *d.add(2) = *s.add(2);
        let last = *s.add(3);
        *d.add(3) = last;
        last
    }
});
