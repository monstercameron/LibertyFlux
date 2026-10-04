// original: 0x00cfcb20 euphoria_ctor_dual_vec
/// Constructor for an animation blend object holding two 3-word vectors.
///
/// Runs the shared base constructor, stamps this object's virtual table,
/// stores two scalar parameters and a float factor (forwarded as bits),
/// copies the two input vectors into the object, then zeroes the state
/// words and sets two default factors to 1.0. Returns the object pointer.
lf_rs85_rt::export!(thiscall, rw_00cfcb20(this: *mut u8, first_scalar: u32, second_scalar: u32, factor_bits: u32, first_vec: *const u32, second_vec: *const u32) -> u32 {
    unsafe {
        /// Bit pattern of the 1.0f default factor.
        const ONE: u32 = 0x3F800000;
        lf_rs85_rt::callee_thiscall!(1, u32, this as u32);
        let w = |off: usize| this.add(off) as *mut u32;
        *w(0x14) = first_scalar;
        *w(0x18) = second_scalar;
        *w(0) = lf_rs85_rt::relocated(0x00EE05E4);
        *w(0x1C) = factor_bits;
        for i in 0..3 {
            *w(0x20 + i * 4) = *second_vec.add(i);
            *w(0x30 + i * 4) = *first_vec.add(i);
        }
        *w(0x40) = 0;
        *w(0x44) = 0;
        *w(0x48) = 0;
        *w(0x50) = ONE;
        *(this.add(0x54) as *mut u16) = 0;
        *(this.add(0x56) as *mut u8) = 0;
        *w(0x58) = ONE;
        this as u32
    }
});
