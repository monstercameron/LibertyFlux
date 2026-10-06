// original: 0x00b008c0 probe_then_handle_each
/// Probe every element, handling the ones that decline.
///
/// thiscall `(this, arg)`: reads the element count (16-bit at `+4`) and
/// the element-pointer array at `+0`, and for each element calls its
/// vtable slot at `+0x14` as thiscall `(element)`. When the probe's low
/// answer byte is zero it additionally calls slot `+0x10` as thiscall
/// `(element, arg)`. The loop is a do-while over the UNSIGNED-nonzero
/// count with a SIGNED continue check (equivalent to unsigned for every
/// 16-bit count), like its sibling above. Returns the element count in
/// all cases: the loop epilogue reloads it over the last answer.
export!(thiscall, rw_00b008c0(this: u32, arg: u32) -> u32 {
    unsafe {
        let count = ((this + 4) as *const u16).read_unaligned() as u32;
        if count != 0 {
            let arr = (this as *const u32).read_unaligned();
            let mut i = 0u32;
            loop {
                let elem = ((arr + i.wrapping_mul(4)) as *const u32).read_unaligned();
                let vtbl = (elem as *const u32).read_unaligned();
                let probe = ((vtbl + 0x14) as *const u32).read_unaligned();
                let fp: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(probe as usize);
                let a = fp(elem);
                if (a as u8) == 0 {
                    let tgt = ((vtbl + 0x10) as *const u32).read_unaligned();
                    let fh: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    let _ = fh(elem, arg);
                }
                i = i.wrapping_add(1);
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
        count
    }
});
