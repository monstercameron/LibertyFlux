// original: 0x00926B20 input_point_store
/// Store three integers and eleven float bit-patterns into an object.
///
/// `this` receives `a1` at +0, `a0` at +4 and `a13` at +8; the eleven float
/// words land at +0xC..+0x28 in order, except the last three are permuted:
/// `f8` goes to +0x30, `f9` to +0x2C and `f10` to +0x34. Floats move as raw
/// bits (`movss`), so NaN payloads are preserved bit-exactly.
export!(thiscall, rw_00926B20(obj: u32, a0: u32, a1: u32, f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32, f7: u32, f8: u32, f9: u32, f10: u32, a13: u32) -> u32 {
    unsafe {
        let o = obj as *mut u32;
        *o.add(0) = a1;
        *o.add(1) = a0;
        *o.add(2) = a13;
        *o.add(3) = f0;
        *o.add(4) = f1;
        *o.add(5) = f2;
        *o.add(6) = f3;
        *o.add(7) = f4;
        *o.add(8) = f5;
        *o.add(9) = f6;
        *o.add(10) = f7;
        // Note the swap: f8 lands at +0x30 and f9 at +0x2C.
        *o.add(12) = f8;
        *o.add(11) = f9;
        *o.add(13) = f10;
        0
    }
});
