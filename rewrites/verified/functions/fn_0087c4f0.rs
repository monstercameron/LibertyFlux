// original: 0x0087c4f0 crmt_buffer_assign (proposed)
/// Assign the half-open range [src, end) into this buffer object.
///
/// Lengths with len+1 <= 0x10 use the inline slots at `+0x10` (cursor base)
/// and `+0x14` (limit); longer ones allocate len+1 bytes through intercepted
/// callee 1 (stdcall/2, size passed twice) and set the base, cursor and
/// limit from the answer. The bytes move through intercepted callee 2
/// (cdecl/3, dst/src/len) and the cursor ends at base+len. Lengths are
/// unsigned throughout. The overflow path (len = 0 reached with len+1 >
/// 0x10) is arithmetically unreachable; its callee is left undeclared so
/// reaching it would fault honestly rather than pass silently. In the long
/// path with end == src no copy runs and the cursor is the base. Returns the
/// final cursor value.
///
/// Original: thiscall/2, two direct calls (plus one unreachable), no floats.
export!(thiscall, rw_0087c4f0(this: u32, src: u32, end: u32) -> u32 {
    /// Cursor slot: points at the base, ends at base+len.
    const CURSOR_OFF: u32 = 0x10;
    /// Limit slot: inline base address or base+len+1.
    const LIMIT_OFF: u32 = 0x14;
    /// Largest len+1 served by the inline slots.
    const INLINE_MAX: u32 = 0x10;
    unsafe {
        let len = end.wrapping_sub(src);
        let buf = this.wrapping_add(CURSOR_OFF);
        if len.wrapping_add(1) <= INLINE_MAX {
            ((this + LIMIT_OFF) as *mut u32).write_unaligned(buf);
            (buf as *mut u32).write_unaligned(this);
        } else {
            if len.wrapping_sub(1) > 0xFFFFFFFEu32 {
                // Provably unreachable (needs len == 0 with len+1 > 0x10);
                // calling the undeclared slot faults honestly if reached.
                callee_cdecl!(99, u32,);
            }
            let size = len.wrapping_add(1);
            let m = callee_stdcall!(1, u32, size, size);
            (this as *mut u32).write_unaligned(m);
            (buf as *mut u32).write_unaligned(m);
            ((this + LIMIT_OFF) as *mut u32)
                .write_unaligned(m.wrapping_add(1).wrapping_add(len));
        }
        let limit = ((this + LIMIT_OFF) as *const u32).read_unaligned();
        if limit != buf {
            let base = (this as *const u32).read_unaligned();
            if end != src {
                let r = callee_cdecl!(2, u32, base, src, len);
                let cur = r.wrapping_add(len);
                (buf as *mut u32).write_unaligned(cur);
                return cur;
            }
            (buf as *mut u32).write_unaligned(base);
            return base;
        }
        if len != 0 {
            callee_cdecl!(2, u32, this, src, len);
        }
        let limit2 = ((this + LIMIT_OFF) as *const u32).read_unaligned();
        let base2 = if limit2 != buf {
            (this as *const u32).read_unaligned()
        } else {
            this
        };
        let cur = base2.wrapping_add(len);
        (buf as *mut u32).write_unaligned(cur);
        cur
    }
});
