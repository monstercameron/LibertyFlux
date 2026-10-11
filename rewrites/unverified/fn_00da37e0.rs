// original: 0x00DA37E0 ped_attractor_accept_via_virtual_method (proposed)

/// Accept an item when the secondary pointer is present and both attractor
/// validators accept it. The accepted item is then sent through virtual slot
/// five on the secondary object, followed by the attractor's completion
/// helper. Failure at either validator returns low byte zero; the success
/// path returns low byte one.
///
/// `self_ptr` is in ECX and the two 32-bit arguments are on the stack. The
/// secondary object's vtable pointer is its first word. The virtual method
/// takes the primary argument and uses thiscall cleanup.
lf_checker_rt::export!(thiscall, rw_00da37e0(self_ptr: u32, primary: u32, secondary: u32) -> u8 {
    if secondary == 0 {
        return 0;
    }
    let eligible = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, primary);
    if eligible as u8 == 0 {
        return 0;
    }
    let available = lf_checker_rt::callee_thiscall!(2, u32, self_ptr, secondary);
    if available as u8 == 0 {
        return 0;
    }
    unsafe {
        const VTABLE_POINTER: u32 = 0;
        const ACCEPT_SLOT: u32 = 0x14;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let table = read_u32(secondary.wrapping_add(VTABLE_POINTER));
        let target = read_u32(table.wrapping_add(ACCEPT_SLOT));
        let method: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let _ = method(secondary, primary);
    }
    let _ = lf_checker_rt::callee_thiscall!(4, u32, self_ptr, secondary);
    1
});
