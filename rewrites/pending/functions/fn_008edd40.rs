// original: 0x008edd40 NativeImpl_REGISTER_PLAYER_RESPAWN_COORDS
/// Publish one respawn record into the global table.
///
/// Copies the four dwords at `vec` plus the current stamp into slot
/// `idx` of the table at 0x1177680. Returns the stamp.
export!(stdcall, rw_008edd40(idx: u32, vec: *const u32) -> u32 {
    unsafe {
        let g = *global::<u32>(0x11735B4);
        let base = 0x1177680 + idx * 32;
        *global::<u32>(base) = *vec;
        *global::<u32>(base + 4) = *vec.add(1);
        *global::<u32>(base + 8) = *vec.add(2);
        *global::<u32>(base + 12) = *vec.add(3);
        *global::<u32>(base + 16) = g;
        g
    }
});
