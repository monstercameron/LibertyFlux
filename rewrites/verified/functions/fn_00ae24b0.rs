// original: 0x00ae24b0 ui_tagged_record_emit
/// Emit a tagged record downstream, registering it when the tag is new.
///
/// Returns the tag at once when it is nonzero and differs from the key word.
/// Otherwise the record word at `obj+8` is driven through the keyed sink
/// until the sink echoes it back (the key mixes the key word with the record,
/// or with the sign bit when the record is non-negative); a negative record
/// ends the call, otherwise the marker word at `+0xC` is set and the record is
/// handed to the float-pair sink together with the float word. Returns the
/// tag, the echoed record, or the sink's answer respectively.
export!(cdecl, rw_00ae24b0(obj: u32, f: u32, g: u32, tag: u32) -> u32 {
    unsafe {
        if tag != 0 && tag != g {
            return tag;
        }
        let bp = obj.wrapping_add(8);
        loop {
            let v = (bp as *const u32).read();
            let key = if (v as i32) < 0 { v | g } else { g | 0x80000000 };
            let r = callee_cdecl!(1, u32, bp, key, v);
            if r == v {
                if (v as i32) < 0 {
                    return r;
                }
                break;
            }
        }
        ((obj + 0xC) as *mut u32).write(0x80000000);
        callee_cdecl!(2, u32, obj, f)
    }
});
