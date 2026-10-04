// original: 0x00d8c730 audio_fixed_bytes_to_floats
/// Unpacks six signed bytes to scaled floats plus derived cross terms.
///
/// The bytes at `this[0xC..0x12]` are sign-extended, converted to float and
/// scaled, landing at `out+0/4/8/0x10/0x14/0x18` (slot `out+0xC` is left
/// untouched); three cross products of those values go to
/// `out+0x20/0x24/0x28`, and the dwords at `this+0/4/8` are copied to
/// `out+0x30/0x34/0x38`. Returns the third copied dword.
export!(thiscall, rw_00d8c730(this: *const u8, out: *mut u8) -> u32 {
    unsafe {
        let s = *global::<f32>(0x00FE_8700);
        let f0 = ((*(this.add(0xC) as *const i8) as i32) as f32) * s;
        let f1 = ((*(this.add(0xD) as *const i8) as i32) as f32) * s;
        let f2 = ((*(this.add(0xE) as *const i8) as i32) as f32) * s;
        let f3 = ((*(this.add(0xF) as *const i8) as i32) as f32) * s;
        let f4 = ((*(this.add(0x10) as *const i8) as i32) as f32) * s;
        let f5 = ((*(this.add(0x11) as *const i8) as i32) as f32) * s;
        let o = out as *mut f32;
        *o = f0;
        *o.add(1) = f1;
        *o.add(2) = f2;
        *o.add(4) = f3;
        *o.add(5) = f4;
        *o.add(6) = f5;
        *o.add(8) = f5 * f1 - f4 * f2;
        *o.add(9) = f3 * f2 - f0 * f5;
        *o.add(10) = f0 * f4 - f3 * f1;
        *(out.add(0x30) as *mut u32) = *(this as *const u32);
        *(out.add(0x34) as *mut u32) = *(this.add(4) as *const u32);
        let w2 = *(this.add(8) as *const u32);
        *(out.add(0x38) as *mut u32) = w2;
        w2
    }
});
