// original: 0x00cfcbb0 euphoria_ctor_vec_flag
/// Constructor for an animation blend object holding one 3-word vector.
///
/// Runs the shared base constructor, stamps this object's virtual table,
/// stores two scalar parameters, a float factor (forwarded as bits) and
/// one input vector, keeps the low byte of a flags word, zeroes the state
/// words and sets the default factor to 1.0. Returns the object pointer.
lf_rs85_rt::export!(thiscall, rw_00cfcbb0(this: *mut u8, first_scalar: u32, second_scalar: u32, factor_bits: u32, vec: *const u32, flags: u32) -> u32 {
    unsafe {
        /// Bit pattern of the 1.0f default factor.
        const ONE: u32 = 0x3F800000;
        lf_rs85_rt::callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0x14) = first_scalar;
        *w(0x18) = second_scalar;
        *w(0) = lf_rs85_rt::relocated(0x00EE063C);
        *w(0x1C) = factor_bits;
        for i in 0..3 {
            *w(0x20 + i * 4) = *vec.add(i);
        }
        *(this.add(0x30) as *mut u8) = 0;
        *w(0x40) = 0;
        *w(0x44) = 0;
        *w(0x48) = 0;
        *w(0x50) = ONE;
        *(this.add(0x54) as *mut u8) = (flags & 0xFF) as u8;
        this as u32
    }
});
