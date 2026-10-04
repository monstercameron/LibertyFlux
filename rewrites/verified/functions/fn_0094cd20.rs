// original: 0x0094cd20 NativeImpl_IS_CHAR_DEAD
/// Script native `IS_CHAR_DEAD`: report whether a character is dead.
///
/// Returns true when the active flag is set and the state field holds 1 or
/// 2; otherwise returns true when the link field is null.
export!(cdecl, rw_0094cd20(obj: *const u8) -> u32 {
    unsafe {
        let active = *obj.add(0x210);
        let state = *(obj.add(0xA74) as *const u32);
        if active != 0 && (state == 1 || state == 2) {
            1
        } else {
            (*(obj.add(0x224) as *const u32) == 0) as u32
        }
    }
});
