// original: 0x009539c0 ensure_cursor_room
/// Check the cursor plus `need` fits the arena; rotate the ring if not.
///
/// Returns 1 when there is room or the rotation succeeds, 0 when the ring
/// reports failure. Only the low byte is significant on the failure path.
export!(cdecl, rw_009539c0(need: u32) -> u32 {
    unsafe {
        let total = (*global::<u32>(0x11FA008)).wrapping_add(need);
        if total <= 0x895430 {
            return (total & 0xFFFFFF00) | 1;
        }
        let rotated: u32 = callee_cdecl!(0, u32,);
        if rotated & 0xFF != 0 {
            (rotated & 0xFFFFFF00) | 1
        } else {
            rotated
        }
    }
});
