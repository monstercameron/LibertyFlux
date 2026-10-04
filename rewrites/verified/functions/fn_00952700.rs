// original: 0x00952700 stamp_sum
/// Return the sum of the two stamp words.
export!(cdecl, rw_00952700() -> u32 {
    unsafe {
        let a = *global::<u32>(0x011f7030);
        let b = *global::<u32>(0x011f7028);
        a.wrapping_add(b)
    }
});
