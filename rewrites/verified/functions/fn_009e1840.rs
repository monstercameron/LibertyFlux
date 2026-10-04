// original: 0x009e1840 portal_vis_pair_store
/// Store two words into global slots. (cdecl/2)
export!(cdecl, rw_009e1840(a: u32, b: u32) -> u32 {
    unsafe {
        *global::<u32>(0x12B41A4) = a;
        *global::<u32>(0x103B110) = b;
        b
    }
});
