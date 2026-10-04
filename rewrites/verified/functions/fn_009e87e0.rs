// original: 0x009e87e0 ped_clear_and_notify
/// Zeroes three words at the out-pointer, then runs the notify
/// hook from virtual slot `+0x128`, returning its answer. The second
/// stack argument is ignored. (thiscall, 2 args.)
lf_checker_rt::export!(thiscall, rw_009e87e0(this_ptr: u32, out_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x128;
        (out_ptr as *mut u32).write_unaligned(0);
        (out_ptr.wrapping_add(4) as *mut u32).write_unaligned(0);
        (out_ptr.wrapping_add(8) as *mut u32).write_unaligned(0);
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot = (vt.wrapping_add(SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        f(this_ptr)
    }
});
