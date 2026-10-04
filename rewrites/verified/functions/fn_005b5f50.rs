// original: 0x005B5F50 span_compact_finalize
/// Slides the live records down over the removed span, finalises the trimmed
/// tail, and records the new end. Returns the destination pointer.
export!(thiscall, rw_005B5F50(obj: *mut u32, dst: *mut u32, src: *const u32, _unused: u32) -> u32 {
    unsafe {
        let end = obj.add(1).read();
        let count = ((end.wrapping_sub(src as u32)) as i32) >> 3;
        let mut d = dst;
        let mut s = src;
        if count > 0 {
            let mut k = count;
            while k > 0 {
                d.add(0).write(s.add(0).read());
                d.add(1).write(s.add(1).read());
                d = d.add(2);
                s = s.add(2);
                k -= 1;
            }
        }
        callee_cdecl!(1, u32, obj as u32);
        obj.add(1).write(d as u32);
        dst as u32
    }
});
