// original: 0x00b9ad40 REMOVE_NAVMESH_REQUIRED_REGION
/// Native handler `REMOVE_NAVMESH_REQUIRED_REGION`: remove a navmesh required-region: forward the region coords, return 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00b9ad40(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1),);
        // The engine returns a byte-wide boolean; the handler stores it
        // zero-extended to 32 bits.
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
