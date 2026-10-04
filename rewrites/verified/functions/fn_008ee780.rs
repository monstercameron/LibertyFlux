// original: 0x008ee780 push_indexed_record
/// Append one 48-byte record to the global record array.
///
/// Writes the next slot (selected by the counter at 0x1176E38, which is
/// incremented) of the array at 0x1176E40: three float/plain dwords, three
/// plain dwords (one answered by a helper call), several flag bytes and
/// bitfields, and a scaled level byte. The mode argument `a8` picks two
/// behaviours at once: values 0x0E..=0x2C store a shifted mode byte and take
/// the direct scale path, anything else stores the raw low byte and clamps
/// the level input to 1.0 first (except that mode 8, although out of range,
/// still takes the direct path, matching the original's two separate tests).
/// Does nothing when the array pointer is null. Returns the array pointer.
///
/// The null-array early-out returns entry EAX in the original, which a
/// rewrite cannot observe; the contract always seeds a non-null array.
export!(stdcall, rw_008ee780(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32) -> u32 {
    unsafe {
        const IDX_VA: u32 = 0x1176E38;
        const BASE_VA: u32 = 0x1176E40;
        const DIRECT_VA: u32 = 0xE833C4;
        const ONE_VA: u32 = 0xFE88E8;
        const CLAMPED_VA: u32 = 0xFE8B20;
        const STRIDE: u32 = 48;
        let base = *global::<u32>(BASE_VA);
        if base == 0 {
            return 0;
        }
        let idx = *global::<u32>(IDX_VA);
        let rec = base.wrapping_add(idx.wrapping_mul(STRIDE));
        // Mode byte behaviour; `sub`/`cmp`/`ja` is an unsigned range test.
        let in_range = a8.wrapping_sub(0x0E) <= 0x1E;
        let mode = if in_range { 8u8 } else { a8 as u8 };
        let tag = if in_range {
            (a8 as u8).wrapping_sub(0x0D)
        } else {
            0u8
        };
        *(rec as *mut u32) = a2;
        *((rec + 4) as *mut u32) = a3;
        *((rec + 8) as *mut u32) = a4;
        *((rec + 0x14) as *mut u32) = a1;
        *((rec + 0x1C) as *mut u32) = callee_cdecl!(1, u32, a0, 0);
        *((rec + 0x18) as *mut u32) = a11;
        let w20 = (rec + 0x20) as *mut u16;
        *w20 = (*w20 & 0xFFFE) | ((a5 & 1) as u16);
        *((rec + 0x22) as *mut u8) = mode;
        *((rec + 0x23) as *mut u8) = a9 as u8;
        *((rec + 0x24) as *mut u8) = tag;
        let w26 = (rec + 0x26) as *mut u16;
        *w26 = (*w26 & 0xFFFB) | (((a13 & 1) as u16) << 2);
        *((rec + 0x28) as *mut u16) = (a14 & 0xFF) as u16;
        *w20 = (*w20 & 0xFFFD) | (((a7 & 1) as u16) << 1);
        // Bit 1 of the second flag word comes from a6 or a12 depending on
        // whether a7 is zero; both are read as bytes by the original.
        let sel = if (a7 as u8) != 0 { a6 } else { a12 };
        *w26 = (*w26 & 0xFFFD) | (((sel & 1) as u16) << 1);
        // Level byte: direct scale when the mode register holds 8 (the
        // in-range marker, or mode 8 itself), else clamp-then-scale.
        let x = f32::from_bits(a10);
        let ebx = if in_range { 8u32 } else { a8 };
        let scaled = if ebx == 8 {
            x * *global::<f32>(DIRECT_VA)
        } else {
            let one = *global::<f32>(ONE_VA);
            let c = if one > x { x } else { one };
            c * *global::<f32>(CLAMPED_VA)
        };
        // Exact cvttss2si: truncate, out-of-range/NaN yields 0x80000000.
        let t = scaled.trunc();
        let cvt = if t.is_nan() || t < -2147483648.0 || t >= 2147483648.0 {
            0x80000000u32
        } else {
            (t as i32) as u32
        };
        *((rec + 0x25) as *mut u8) = (cvt & 0xFF) as u8;
        *global::<u32>(IDX_VA) = idx.wrapping_add(1);
        base
    }
});
