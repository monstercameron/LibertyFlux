// original: 0x008ad9a0 audio_curve_init_from_desc
/// Count-dispatched float initializer from a descriptor record.
///
/// Copies breakpoint pairs out of the descriptor at `[this]` according to its
/// count field (counts above 3 take an early-out path that clears the mode
/// byte), stamps the mode word, then derives two reciprocal-span factors.
/// Always reports success.
export!(thiscall, rw_008ad9a0(this: u32) -> u32 {
    unsafe {
        let edx = ld32(this);
        let count = ld32(edx.wrapping_add(0x15));
        if count > 3 {
            ((this.wrapping_add(0x26)) as *mut u8).write_unaligned(0);
            return 1;
        }
        let mut x1: f32;
        let mut x3: f32;
        if count == 0 {
            st32(this.wrapping_add(4), 0);
            st32(this.wrapping_add(8), 0);
            x1 = 0.0;
            x3 = 0.0;
        } else {
            x1 = rdf(edx.wrapping_add(0x19));
            wrf(this.wrapping_add(4), x1);
            x3 = rdf(edx.wrapping_add(0x1d));
            wrf(this.wrapping_add(8), x3);
        }
        let one = *global::<f32>(0xFE88E8);
        if ld32(edx.wrapping_add(0x15)) > 1 {
            x1 = rdf(edx.wrapping_add(0x21));
            wrf(this.wrapping_add(0x0c), x1);
            x3 = rdf(edx.wrapping_add(0x25));
        } else {
            wrf(this.wrapping_add(0x0c), x1 + one);
        }
        wrf(this.wrapping_add(0x10), x3); // join: both paths store x3 here
        if ld32(edx.wrapping_add(0x15)) > 2 {
            st32(this.wrapping_add(0x14), ld32(edx.wrapping_add(0x29)));
            st32(this.wrapping_add(0x18), ld32(edx.wrapping_add(0x2d)));
        } else {
            x1 = x1 + *global::<f32>(0xFE8A24);
            wrf(this.wrapping_add(0x18), x3);
            wrf(this.wrapping_add(0x14), x1);
        }
        ((this.wrapping_add(0x25)) as *mut u16).write_unaligned(0x0100);
        let span0 = rdf(this.wrapping_add(0x0c)) - rdf(this.wrapping_add(0x04));
        wrf(this.wrapping_add(0x1c), one / span0);
        let span1 = rdf(this.wrapping_add(0x14)) - rdf(this.wrapping_add(0x0c));
        wrf(this.wrapping_add(0x20), one / span1);
        1
    }
});
