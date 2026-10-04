// original: 0x00cf2210 CPlayStatBase::vf2
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

#[inline(always)]
fn fmt(file_va: u32) -> u32 {
    relocated(file_va)
}

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr8(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write_unaligned(v) }
}

// Sign-extended 16-bit field load (movsx).
#[inline(always)]
unsafe fn sx16(addr: u32) -> u32 {
    unsafe { (rd16(addr) as i16 as i32) as u32 }
}

// Bit-exact f32 -> f64 value conversion (cvtps2pd), returned as (lo, hi).
#[inline(always)]
fn f32_to_f64_bits(f: f32) -> (u32, u32) {
    let b = (f as f64).to_bits();
    ((b & 0xFFFF_FFFF) as u32, (b >> 32) as u32)
}

// Serializes one play-stat object into a text buffer.
//
// Clears the buffer, resolves the stat's display id through two lookup
// helpers, then formats the stat's fields into the buffer with a family of
// append helpers. The display id selects one of sixteen format shapes.
export!(thiscall, rw_cf2210(this_ptr: u32, buf: u32, aux: u32) -> u32 {
    unsafe {
        wr8(buf, 0);
        let stat_id: u32 = callee_cdecl!(1, u32,);
        let name_ptr: u32 = callee_cdecl!(2, u32, stat_id);
        callee_cdecl!(3, u32, buf, aux, fmt(0x00EDDDD0), stat_id, name_ptr);
        let tag: u32 = callee_thiscall!(4, u32, this_ptr);
        let head: u32 = callee_cdecl!(10, u32, buf, aux, fmt(0x00EDDDD8), tag);
        let idx: u32 = stat_id.wrapping_sub(1);
        match idx {
            // Two short fields.
            0 | 2 | 11 | 19 | 40 => {
                let hi = sx16(this_ptr + 0x36);
                let lo = sx16(this_ptr + 0x34);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDDE4), lo, hi)
            }
            // Named pair plus short fields, with an optional note suffix.
            1 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDDF0), b, a);
                let hi = sx16(this_ptr + 0x36);
                let lo = sx16(this_ptr + 0x34);
                let r: u32 = callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDDF8), lo, hi);
                if rd8(this_ptr + 0x38) == 0 {
                    r
                } else {
                    callee_cdecl!(10, u32, buf, aux, fmt(0x00EDDE04), this_ptr + 0x38)
                }
            }
            // Named pair only.
            3 | 4 | 5 | 10 | 16 | 17 | 18 | 22 | 30 | 35 | 36 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDE3C), b, a)
            }
            // Named pair plus short fields.
            6 | 7 | 41 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDE28), b, a);
                let hi = sx16(this_ptr + 0x36);
                let lo = sx16(this_ptr + 0x34);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDE30), lo, hi)
            }
            // Named pair plus one word field.
            23 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDE9C), b, a);
                let w = rd32(this_ptr + 0x34);
                callee_cdecl!(10, u32, buf, aux, fmt(0x00EDDEA4), w)
            }
            // Named pair plus two word fields.
            24 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDEAC), b, a);
                let w38 = rd32(this_ptr + 0x38);
                let w34 = rd32(this_ptr + 0x34);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDEB4), w34, w38)
            }
            // Named pair plus a float field printed as double.
            25 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDEC8), b, a);
                let (lo, hi) = f32_to_f64_bits(f32::from_bits(rd32(this_ptr + 0x34)));
                callee_cdecl!(12, u32, buf, aux, fmt(0x00EDDED0), 3, lo, hi)
            }
            // Named pair plus a word and a float-as-double.
            26 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDEDC), b, a);
                let (lo, hi) = f32_to_f64_bits(f32::from_bits(rd32(this_ptr + 0x38)));
                let w34 = rd32(this_ptr + 0x34);
                callee_cdecl!(13, u32, buf, aux, fmt(0x00EDDEE4), w34, 3, lo, hi)
            }
            // Id-keyed shape with a trailing tagged word.
            27 => {
                let s = rd32(this_ptr + 0x34);
                wr8(buf, 0);
                let v: u32 = callee_cdecl!(8, u32, s);
                callee_cdecl!(3, u32, buf, aux, fmt(0x00EDDEF8), s, v);
                let w = rd32(this_ptr + 0x38);
                let g: u32 = callee_thiscall!(4, u32, this_ptr);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDF00), g, w)
            }
            // Id-keyed shape with a float-as-double tail.
            28 => {
                let s = rd32(this_ptr + 0x34);
                wr8(buf, 0);
                let v: u32 = callee_cdecl!(8, u32, s);
                callee_cdecl!(3, u32, buf, aux, fmt(0x00EDDF14), s, v);
                let (lo, hi) = f32_to_f64_bits(f32::from_bits(rd32(this_ptr + 0x38)));
                let g: u32 = callee_thiscall!(4, u32, this_ptr);
                callee_cdecl!(13, u32, buf, aux, fmt(0x00EDDF1C), g, 3, lo, hi)
            }
            // Id-keyed shape with two tagged words.
            29 => {
                let s = rd32(this_ptr + 0x34);
                wr8(buf, 0);
                let v: u32 = callee_cdecl!(8, u32, s);
                callee_cdecl!(3, u32, buf, aux, fmt(0x00EDDF30), s, v);
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let g: u32 = callee_thiscall!(4, u32, this_ptr);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDF38), g, a)
            }
            // Flag-byte-selected shape with byte fields.
            31 | 32 | 37 => {
                let flag = rd8(this_ptr + 0x49);
                let sel = if flag == 1 {
                    fmt(0x00EDDE44)
                } else if flag == 2 {
                    fmt(0x00EDDE48)
                } else {
                    fmt(0x00EDDE4C)
                };
                let b4b = rd8(this_ptr + 0x4B) as u32;
                let b4a = rd8(this_ptr + 0x4A) as u32;
                let b4c = rd8(this_ptr + 0x4C) as u32;
                let b48 = rd8(this_ptr + 0x48) as u32;
                let w3c = rd32(this_ptr + 0x3C);
                let w38 = rd32(this_ptr + 0x38);
                let r: u32 = callee_cdecl!(
                    14, u32, buf, aux, fmt(0x00EDDE54),
                    w38, w3c, b48, b4c, b4a, b4b, sel
                );
                if rd8(this_ptr + 0x49) != 2 {
                    r
                } else {
                    let w44 = rd32(this_ptr + 0x44);
                    let w40 = rd32(this_ptr + 0x40);
                    callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDE8C), w40, w44)
                }
            }
            // Named pair plus a word and two short fields.
            39 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDF4C), b, a);
                let w38 = rd32(this_ptr + 0x38);
                let v36 = sx16(this_ptr + 0x36);
                let v34 = sx16(this_ptr + 0x34);
                callee_cdecl!(12, u32, buf, aux, fmt(0x00EDDF54), v34, v36, w38)
            }
            // A word plus two short fields.
            42 => {
                let v3a = sx16(this_ptr + 0x3A);
                let v38 = sx16(this_ptr + 0x38);
                let w34 = rd32(this_ptr + 0x34);
                callee_cdecl!(12, u32, buf, aux, fmt(0x00EDDF6C), w34, v38, v3a)
            }
            // One word field.
            43 | 44 => {
                let w34 = rd32(this_ptr + 0x34);
                callee_cdecl!(10, u32, buf, aux, fmt(0x00EDDF80), w34)
            }
            // Named pair plus short fields (second spelling).
            45 => {
                let a: u32 = callee_thiscall!(5, u32, this_ptr);
                let b: u32 = callee_thiscall!(6, u32, this_ptr);
                callee_cdecl!(7, u32, buf, aux, fmt(0x00EDDE14), b, a);
                let hi = sx16(this_ptr + 0x36);
                let lo = sx16(this_ptr + 0x34);
                callee_cdecl!(11, u32, buf, aux, fmt(0x00EDDE1C), lo, hi)
            }
            // Anything else keeps just the header. Note the exit value:
            // ids that dispatch through the index table leave the table's
            // value (16) in EAX, while out-of-range ids jump straight here
            // with the header call's answer still in EAX.
            _ => {
                if idx <= 0x2D {
                    16
                } else {
                    head
                }
            }
        }
    }
});
