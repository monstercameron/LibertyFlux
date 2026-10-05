// original: 0x00d54650 ccam_free_clear_vec

/// Zero the three-word vector at offsets `+0x150`..`+0x158` of the camera
/// object.
///
/// `this` points to the object. The three consecutive words are written with
/// zero; nothing is read and no value is returned (EAX passes through
/// untouched, so the return channel is not compared).
///
/// Original: 0x00d54650 (thiscall, no stack arguments, no calls).
lf_checker_rt::export!(thiscall, rw_00d54650(this: u32) -> u32 {
    unsafe {
        /// First of the three cleared words.
        const VEC: u32 = 0x150;
        ((this + VEC) as *mut u32).write_unaligned(0);
        ((this + VEC + 4) as *mut u32).write_unaligned(0);
        ((this + VEC + 8) as *mut u32).write_unaligned(0);
        0
    }
});
