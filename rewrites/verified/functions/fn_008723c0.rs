// original: 0x008723C0 rage::crExpressionProcessor::ExpressionFilter::vf4
//! Hash the filter entries for expression evaluation: start from the bitwise
//! complement of the entry count, and for each entry whose first byte is
//! set rotate the accumulator left by 7 and fold in bytes 2, 0x0A and 0x0B.
//! A missing table hashes to 0; a non-positive count hashes to its complement.
export!(thiscall, rw_008723C0(this: *mut u8) -> u32 {
    unsafe {
        let obj = *(this.add(0x0C) as *const u32);
        if obj == 0 {
            return 0;
        }
        let count = *(obj.wrapping_add(4) as *const i32);
        let mut acc = !(count as u32);
        if count <= 0 {
            return acc;
        }
        let mut p = obj as *const u32;
        let mut n = count;
        while n != 0 {
            let entry = (*p) as *const u8;
            if *entry != 0 {
                acc = acc.rotate_left(7);
                let b0a = *entry.add(0x0A) as u32;
                let b0b = *entry.add(0x0B) as u32;
                let b02 = *entry.add(0x02) as u32;
                let mut d = b0a.wrapping_add(b0a) | b0b;
                d = d.wrapping_add(d) | b02;
                acc ^= d << 13;
            }
            p = p.add(1);
            n -= 1;
        }
        acc
    }
});
