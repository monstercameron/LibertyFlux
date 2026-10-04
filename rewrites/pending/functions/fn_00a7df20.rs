// original: 0x00a7df20 angle_ctor
/// Angle-carrying constructor: base-construct, store the timer words, the
/// mode byte and the has-descriptor flag, stamp our vtable, wrap the angle
/// into (-2pi, 2pi], then copy the 16-byte descriptor (or clear it and store
/// a zero angle-rate when none was given). Returns the object.
///
/// The wrap loops use `>` exactly as the original's comiss/ja pair does, so
/// NaN takes no iterations on either loop, matching the original bit for bit.
export!(thiscall, rw_00a7df20(
    this: *mut u8,
    a: u32,
    b: u32,
    fbits: u32,
    blo: u32,
    src: u32,
) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let two_pi: f32 = *global::<f32>(0xFE8AEC);
        let neg_two_pi: f32 = *global::<f32>(0xFE8DD4);
        st32(this, 0x18, a);
        st32(this, 0x1C, b);
        st8(this, 0x24, blo as u8);
        st8(this, 0x25, (src != 0) as u8);
        st32(this, 0, relocated(0xEA18BC));
        let mut x = f32::from_bits(fbits);
        while x > two_pi {
            x -= two_pi;
        }
        st32(this, 0x20, x.to_bits());
        while neg_two_pi > x {
            x += two_pi;
        }
        st32(this, 0x20, x.to_bits());
        if src != 0 {
            let s = src as *const u32;
            st32(this, 0x30, *s);
            st32(this, 0x34, *(s.add(1)));
            st32(this, 0x38, *(s.add(2)));
            st32(this, 0x3C, *(s.add(3)));
        } else {
            // The original reads an uninitialized scratch word for the last
            // field on this path; the contract defines that fill as zero.
            st32(this, 0x30, 0);
            st32(this, 0x34, 0);
            st32(this, 0x38, 0);
            st32(this, 0x3C, 0);
        }
        this as u32
    }
});
