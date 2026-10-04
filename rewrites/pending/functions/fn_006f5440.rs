// original: 0x006f5440 build_from_packed_header
/// Build an object from a packed header and a parameter block.
///
/// `a1` points at a parameter block, `a3` at packed header bytes. Scalar
/// fields are unpacked from the header, a construction helper is invoked
/// with the inner block pointer `a1 + 4` and the unpacked values, then
/// the vtable, an 11-bit field and a flag bit are stored. Returns `this`.
rt::export!(thiscall, rw_006f5440(
    this: *mut u8,
    a1: *mut u8,
    a2: u32,
    a3: *const u8,
    a4: u32,
    a5: u32,
) -> u32 {
    unsafe {
        let b0 = *a3 as u32;
        let b1 = *a3.add(1) as u32;
        let b2 = *a3.add(2) as u32;
        let b3 = *a3.add(3) as u32;
        let raw = (b0 << 8) | b1;
        let flag = ((((b1 << 2) & 0xFF) | (b2 >> 6)) >> 7);
        let bit = (b1 >> 5) & 1;
        let scaled = (raw >> 6).wrapping_sub((bit | 2) * 2);
        let inner = a1.add(4);
        let w10 = *inner.add(0x10).cast::<u32>();
        let w14 = *inner.add(0x14).cast::<u32>();
        rt::callee_thiscall!(1, u32, this as u32, inner as u32, w10, w14, a2, scaled, a4, flag, a5);
        *this.cast::<u32>() = rt::relocated(0x00FE4AA0);
        for off in [0x4Cusize, 0x50, 0x54] {
            *this.add(off).cast::<u32>() = 0;
        }
        let field = ((b1 & 0x1F) << 11) | (b2 << 3) | (b3 >> 5);
        *this.add(0x60).cast::<u16>() = field as u16;
        *this.add(0x62) = (*this.add(0x62) & !1) | (bit as u8);
        this as u32
    }
});
