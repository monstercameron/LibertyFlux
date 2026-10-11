// original: 0x00B11B30 CHeli::vf18

/// Notify the base object, then update optional resources stored at offsets
/// 0x100 and 0x78. When the release flag is set and the resource at 0xd0 is
/// present, ask its helper for a range table and pass that table with the
/// method argument to the range updater. Each direct helper call is routed
/// through the checker's scripted stand-in. This is a thiscall method with one
/// stack argument.
lf_checker_rt::export!(thiscall, rw_00b11b30(this: u32, value: u32) -> () {
    const PRIMARY_RESOURCE: u32 = 0x78;
    const RELEASE_RESOURCE: u32 = 0xD0;
    const RELEASE_FLAG: u32 = 0xD4;
    const SECONDARY_RESOURCE: u32 = 0x100;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, value);

        let secondary = read_u32(this.wrapping_add(SECONDARY_RESOURCE));
        if secondary != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, secondary, value);
        }

        let primary = read_u32(this.wrapping_add(PRIMARY_RESOURCE));
        if primary != 0 {
            let _ = lf_checker_rt::callee_thiscall!(3, u32, primary, value);
        }

        if read_u32(this.wrapping_add(RELEASE_FLAG)) != 0 {
            let resource = read_u32(this.wrapping_add(RELEASE_RESOURCE));
            if resource != 0 {
                let table = lf_checker_rt::callee_thiscall!(4, u32, resource, 3u32);
                if table != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(5, u32, table, value);
                }
            }
        }
    }
});
