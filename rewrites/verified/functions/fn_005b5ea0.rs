// original: 0x005B5EA0 vector_broadcast_fill
/// Resizes this record vector, then broadcasts one 24-byte pattern over every
/// slot and records the end pointer. Returns this object.
export!(thiscall, rw_005B5EA0(obj: *mut u32, n: u32, src: *const u32, _unused: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32, n, obj as u32);
        let mut dst = obj.add(0).read();
        if n != 0 {
            let mut k = n;
            while k != 0 {
                if dst != 0 {
                    let d = dst as *mut u32;
                    d.add(0).write(src.add(0).read());
                    d.add(1).write(src.add(1).read());
                    d.add(2).write(src.add(2).read());
                    d.add(3).write(src.add(3).read());
                    d.add(4).write(src.add(4).read());
                    d.add(5).write(src.add(5).read());
                }
                dst = dst.wrapping_add(0x18);
                k -= 1;
            }
        }
        obj.add(1).write(dst);
        obj as u32
    }
});
