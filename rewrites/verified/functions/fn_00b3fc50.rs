// original: 0x00b3fc50 joint_pose_to_out (proposed)

/// Evaluate one indexed joint pose into a 4-word output slot.
///
/// `this` points to a pose-set object, `idx` selects the joint, `out` points
/// to four writable words. Layout read (all offsets from `this`): flag byte
/// at `+0x50`, three scale floats at `+0x40..0x48`, a 3x3 matrix with
/// translation at `+0x00..0x38` (columns at `+0x00`, `+0x10`, `+0x20`,
/// translation at `+0x30`; the words at `+0x0c`, `+0x1c`, `+0x2c` are never
/// read), a packed u16 array pointer at `+0x58`, a 16-byte-stride float
/// array pointer at `+0x5c`, and an addend triple pointer at `+0x90`.
///
/// When flag bit 0 is set, three unsigned 16-bit values at
/// `packed + idx*6` are converted to float, each multiplied by the constant
/// 2^-17, then scaled by the matching scale float and offset by the addend
/// triple. Otherwise 16 bytes are copied from `strided + idx*16`.
///
/// When flag bit 2 is set, the three floats are run through the matrix
/// (translation included) in the original's operand order, and the fourth
/// output word is set from a stack slot the original never writes: under
/// the checker's defined stack fill that value is 0, which is what this
/// rewrite stores. Bits of the flag byte other than 0 and 2 are ignored.
///
/// Original: 0x00B3FC50 (thiscall, two stack words, no return value).
lf_checker_rt::export!(thiscall, rw_00b3fc50(this: u32, idx: u32, out: u32) -> u32 {
    unsafe {
        const SCALE_X: u32 = 0x40;
        const SCALE_Y: u32 = 0x44;
        const SCALE_Z: u32 = 0x48;
        const FLAGS: u32 = 0x50;
        const PACKED: u32 = 0x58;
        const STRIDED: u32 = 0x5c;
        const ADDEND: u32 = 0x90;
        const PACKED_SCALE: f32 = f32::from_bits(0x3780_0000); // 2^-17
        const FLAG_PACKED: u8 = 0x01;
        const FLAG_MATRIX: u8 = 0x04;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let flags = rd8(this.wrapping_add(FLAGS));
        if flags & FLAG_PACKED != 0 {
            let base = rd32(this.wrapping_add(PACKED));
            let k = idx.wrapping_mul(3);
            for i in 0..3u32 {
                let raw = rd16(base.wrapping_add(k.wrapping_mul(2)).wrapping_add(i * 2));
                wrf(out.wrapping_add(i * 4), mul(raw as f32, PACKED_SCALE));
            }
            for i in 0..3u32 {
                wrf(
                    out.wrapping_add(i * 4),
                    mul(rdf(this.wrapping_add(SCALE_X + i * 4)), rdf(out.wrapping_add(i * 4))),
                );
            }
            let t = rd32(this.wrapping_add(ADDEND));
            for i in 0..3u32 {
                wrf(
                    out.wrapping_add(i * 4),
                    add(rdf(t.wrapping_add(i * 4)), rdf(out.wrapping_add(i * 4))),
                );
            }
        } else {
            let src = rd32(this.wrapping_add(STRIDED)).wrapping_add(idx.wrapping_mul(16));
            for i in 0..4u32 {
                wr32(out.wrapping_add(i * 4), rd32(src.wrapping_add(i * 4)));
            }
        }
        if flags & FLAG_MATRIX != 0 {
            let x = rdf(out);
            let y = rdf(out.wrapping_add(4));
            let z = rdf(out.wrapping_add(8));
            // Column-major multiply in the original's accumulation order:
            // (col1*y + col0*x) + col2*z, then translation.
            let nx = add(
                add(
                    add(mul(rdf(this.wrapping_add(0x10)), y), mul(rdf(this), x)),
                    mul(rdf(this.wrapping_add(0x20)), z),
                ),
                rdf(this.wrapping_add(0x30)),
            );
            let ny = add(
                add(
                    add(mul(rdf(this.wrapping_add(0x14)), y), mul(rdf(this.wrapping_add(0x04)), x)),
                    mul(rdf(this.wrapping_add(0x24)), z),
                ),
                rdf(this.wrapping_add(0x34)),
            );
            let nz = add(
                add(
                    add(mul(rdf(this.wrapping_add(0x18)), y), mul(rdf(this.wrapping_add(0x08)), x)),
                    mul(rdf(this.wrapping_add(0x28)), z),
                ),
                rdf(this.wrapping_add(0x38)),
            );
            wrf(out, nx);
            wrf(out.wrapping_add(4), ny);
            wrf(out.wrapping_add(8), nz);
            // The original copies an uninitialised stack slot here; the
            // contract defines that slot as 0 (stack_fill).
            wr32(out.wrapping_add(12), 0);
        }
        0
    }
});
