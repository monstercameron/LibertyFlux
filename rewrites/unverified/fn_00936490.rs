// original: 0x00936490 initialize_network_state_globals

/// Initialize the two consecutive network state objects with the per-entry
/// helper, passing indices zero and one. Then clear the dword at image global
/// `0x011a2ea4`, clear the first two status bytes, set the third status byte
/// to one, clear the fourth status byte, and clear the byte at `0x011a2ea8`.
/// The helper call uses ECX for each object and one caller-cleaned stack
/// argument. This initializer has no arguments and no defined return value.
lf_checker_rt::export!(cdecl, rw_00936490() -> () {
    unsafe {
        const OBJECT_ARRAY: u32 = 0x011a_32c0;
        const OBJECT_STRIDE: u32 = 0x0db0;
        const OBJECT_COUNT: u32 = 2;
        const GENERATION_WORD: u32 = 0x011a_2ea4;
        const STATUS_BYTE_0: u32 = 0x011a_2ea0;
        const STATUS_BYTE_1: u32 = 0x011a_2ea1;
        const STATUS_BYTE_2: u32 = 0x011a_2ea2;
        const STATUS_BYTE_3: u32 = 0x011a_2ea3;
        const STATUS_BYTE_8: u32 = 0x011a_2ea8;

        let first_object = lf_checker_rt::relocated(OBJECT_ARRAY);
        for index in 0..OBJECT_COUNT {
            let object = first_object.wrapping_add(index.wrapping_mul(OBJECT_STRIDE));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, object, index);
        }

        unsafe { lf_checker_rt::global::<u32>(GENERATION_WORD).write_unaligned(0) };
        unsafe { lf_checker_rt::global::<u8>(STATUS_BYTE_1).write(0) };
        unsafe { lf_checker_rt::global::<u8>(STATUS_BYTE_0).write(0) };
        unsafe { lf_checker_rt::global::<u8>(STATUS_BYTE_2).write(1) };
        unsafe { lf_checker_rt::global::<u8>(STATUS_BYTE_3).write(0) };
        unsafe { lf_checker_rt::global::<u8>(STATUS_BYTE_8).write(0) };
    }
});
