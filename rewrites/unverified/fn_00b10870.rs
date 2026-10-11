// original: 0x00B10870 heli_release_attached_references

/// When the release flag at `this + 0xd4` is set, sign-extend the 16-bit table
/// index, read its record's signed tag at +0x58, and pass that tag to the
/// reference-release helper unless it equals -1. Then clear the flag and
/// release the optional resource at `this + 0xd0` through its destructor and
/// delete helper, finally clearing the resource slot. This is a no-argument
/// thiscall method.
lf_checker_rt::export!(thiscall, rw_00b10870(this: u32) -> () {
    const RELEASE_FLAG: u32 = 0xD4;
    const RESOURCE: u32 = 0xD0;
    const TABLE_VA: u32 = 0x0129_5CD8;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        if read_u32(this.wrapping_add(RELEASE_FLAG)) != 0 {
            let signed_index = (this.wrapping_add(0x2E) as *const i16).read_unaligned() as i32;
            let table = lf_checker_rt::global::<u32>(TABLE_VA) as u32;
            let entry_address = table.wrapping_add((signed_index as u32).wrapping_mul(4));
            let entry = read_u32(entry_address);
            let release_tag = (entry.wrapping_add(0x58) as *const i16).read_unaligned() as i32;
            if release_tag != -1 {
                let _ = lf_checker_rt::callee_cdecl!(1, u32, release_tag as u32);
            }
        }

        let resource = read_u32(this.wrapping_add(RESOURCE));
        (this.wrapping_add(RELEASE_FLAG) as *mut u32).write_unaligned(0);
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, resource);
            let _ = lf_checker_rt::callee_stdcall!(3, u32, resource);
        }
        (this.wrapping_add(RESOURCE) as *mut u32).write_unaligned(0);
    }
});
