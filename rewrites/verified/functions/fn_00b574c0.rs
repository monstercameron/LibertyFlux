// original: 0x00b574c0 init_blend_with_frame_params
/// Initialise a blend object from a source, floats and constant tables.
///
/// Resolves the incoming source word through the first helper, stores the
/// two incoming floats into the object's fields, then builds a seven-word
/// frame block from a 16-byte constant row plus three constant floats and
/// hands it to the second helper with the constant words 5, 6, 0 and a
/// trailing 0. Finishes by resetting the object through the third helper
/// and clearing its last two fields. Returns the third helper's answer.
export!(thiscall, rw_00b574c0(this_: *mut u8, source: u32, first: f32, second: f32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ as u32, source, 0);
        *(this_.add(0x48) as *mut f32) = first;
        *(this_.add(0x4c) as *mut f32) = second;
        let row = global::<u32>(0x011100d0) as *const u32;
        let block = [
            *row,
            *row.add(1),
            *row.add(2),
            *row.add(3),
            *(global::<u32>(0x01b4b2a0) as *const u32),
            *(global::<u32>(0x01b4b2a4) as *const u32),
            *(global::<u32>(0x01b4b2a8) as *const u32),
        ];
        callee_thiscall!(
            2,
            u32,
            this_ as u32,
            5,
            6,
            0,
            block.as_ptr() as u32,
            0
        );
        let answer = callee_thiscall!(3, u32, (this_ as u32).wrapping_add(0x24), 0);
        *(this_.add(0x50) as *mut u32) = 0;
        *(this_.add(0x54) as *mut u32) = 0;
        answer
    }
});
