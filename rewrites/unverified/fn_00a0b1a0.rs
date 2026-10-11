// original: 0x00a0b1a0 RadarBlipMgr_FlushMissionBlips
/// Scan the 256 mission-cleanup entries beginning at manager offset +4.
/// Entries whose kind byte is 0x12 are checked with the input-handle helper;
/// when it returns zero, initialize that entry through its thiscall helper.
/// The method takes only the manager pointer in ECX and has no specified
/// return value.
lf_checker_rt::export!(thiscall, rw_00a0b1a0(this: u32) -> () {
    unsafe {
        const ENTRY_BASE: u32 = 4;
        const ENTRY_STRIDE: u32 = 0x2c;
        const ENTRY_COUNT: u32 = 256;
        const BLIP_KIND: u8 = 0x12;
        const ENTRY_HANDLE: u32 = 4;
        const INPUT_HANDLE_IS_LIVE: u32 = 0;
        const INITIALIZE_ENTRY: u32 = 1;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        for index in 0..ENTRY_COUNT {
            let entry = this
                .wrapping_add(ENTRY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            if read_u8(entry) != BLIP_KIND {
                continue;
            }

            let handle = read_u32(entry.wrapping_add(ENTRY_HANDLE));
            if lf_checker_rt::callee_cdecl!(INPUT_HANDLE_IS_LIVE, u8, handle) == 0 {
                let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_ENTRY, u32, entry);
            }
        }
    }
});
