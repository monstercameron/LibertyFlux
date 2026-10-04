// original: 0x00e63400 zero_and_init_1195eb0
/// Zeroes 0x30 dwords at 0x1195EB0, then invokes the shared init helper.
///
/// Runs one `rep stosd` over the global range, then calls the helper
/// (cdecl/1, stubbed) with the relocated address 0xE71410 and returns its
/// answer.
export!(cdecl, rw_00e63400() -> u32 {
    unsafe {
        core::ptr::write_bytes(global::<u32>(0x1195EB0), 0, 0x30);
        callee_cdecl!(1, u32, relocated(0xE71410))
    }
});
