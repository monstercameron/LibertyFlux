// original: 0x009f53e0 flag_row_clear
/// Clear the 32-byte flag row plus a counter word and a state byte.
export!(cdecl, rw_009f53e0() -> u32 {
    let row = global::<u8>(0x012B_61B0);
    let mut i = 0usize;
    while i < 32 {
        // SAFETY: worker maps the original image; range is .data.
        unsafe { row.add(i).write(0) };
        i += 1;
    }
    unsafe {
        global::<u16>(0x012B_6174).write(0);
        global::<u8>(0x012B_6170).write(0);
    }
    0
});
