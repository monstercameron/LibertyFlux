// original: 0x006A7790 input_event_parse_b
/// Parse one input event record with a row-matching pass.
///
/// Same stream/this shape as [`rb200_fn2`] but a different layout: after
/// the status-gated 0x3E command (fired when the flag is CLEAR), a
/// big-endian gauge and a count byte are read; the count is announced with
/// command 0x67 and checked against the gauge (command 0x0B unless the
/// gauge equals twice the count plus six and the count is within 1..=4).
/// Each of the count rows then reads two bytes, scans the existing row
/// array at +0xC4 (bounded by +0x24) for a row whose head matches the
/// first byte (firing command 5 when none matches), stores the row pointer
/// into the inline table at +0x128, splits the second byte into nibbles on
/// the row, mirrors the row into the display object and fires command
/// 0x68. A three-byte tail is stored at +0x16C..+0x178, published with
/// command 0x69, the flag word is cleared, +0x7C is incremented, the
/// stream cursor is written back and 1 is returned; any refused refill
/// returns 0 with no write-back. Only the low return byte is significant.
lf_checker_rt::export!(thiscall, rb200_fn3(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn refill(buf: u32, this: u32) -> Option<(u32, u32)> {
        let target = *((buf.wrapping_add(0xC)) as *const u32);
        let f: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if (f(this) & 0xFF) == 0 {
            return None;
        }
        Some((
            *(buf as *const u32),
            *((buf.wrapping_add(4)) as *const u32),
        ))
    }
    #[inline(always)]
    unsafe fn cmd0(obj: u32, this: u32) {
        let target = *(obj as *const u32);
        let f: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this);
    }
    #[inline(always)]
    unsafe fn cmd1(obj: u32, this: u32) {
        let target = *((obj.wrapping_add(4)) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, 1);
    }
    macro_rules! next_byte {
        ($ptr:ident, $cnt:ident) => {{
            let b = *($ptr as *const u8);
            $ptr = $ptr.wrapping_add(1);
            $cnt = $cnt.wrapping_sub(1);
            b as u32
        }};
    }
    macro_rules! post_check {
        ($ptr:ident, $cnt:ident, $buf:ident, $this:ident) => {
            if $cnt == 0 {
                match refill($buf, $this) {
                    Some((p, c)) => {
                        $ptr = p;
                        $cnt = c;
                    }
                    None => return 0,
                }
            }
        };
    }
    macro_rules! pre_check {
        ($ptr:ident, $cnt:ident, $buf:ident, $this:ident) => {
            if $cnt == 0 {
                match refill($buf, $this) {
                    Some((p, c)) => {
                        $ptr = p;
                        $cnt = c;
                    }
                    None => return 0,
                }
            }
        };
    }
    unsafe {
        let buf = *((this.wrapping_add(0x18)) as *const u32);
        let mut ptr = *(buf as *const u32);
        let mut cnt = *((buf.wrapping_add(4)) as *const u32);
        let flag = *((this.wrapping_add(0x194)) as *const u32);
        let sub = *(this as *const u32);
        if *((flag.wrapping_add(0xD)) as *const u8) == 0 {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x3E;
            cmd0(sub, this);
        }
        pre_check!(ptr, cnt, buf, this);
        let mut gauge = next_byte!(ptr, cnt) << 8;
        post_check!(ptr, cnt, buf, this);
        gauge += next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        let n = next_byte!(ptr, cnt);
        *((sub.wrapping_add(0x14)) as *mut u32) = 0x67;
        *((sub.wrapping_add(0x18)) as *mut u32) = n;
        cmd1(sub, this);
        // No refill check after the count byte.
        if gauge != n.wrapping_mul(2).wrapping_add(6) || n < 1 || n > 4 {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x0B;
            cmd0(sub, this);
        }
        *((this.wrapping_add(0x124)) as *mut u32) = n;
        let mut rowptr = this.wrapping_add(0x128);
        let mut i = 0u32;
        while i < n {
            pre_check!(ptr, cnt, buf, this);
            let r0 = next_byte!(ptr, cnt);
            post_check!(ptr, cnt, buf, this);
            let r1 = next_byte!(ptr, cnt);
            // No refill check after the second byte.
            let bound = *((this.wrapping_add(0x24)) as *const i32);
            let arr = *((this.wrapping_add(0xC4)) as *const u32);
            let mut row = arr;
            let mut k = 0i32;
            let mut matched = false;
            if bound > 0 {
                while k < bound {
                    if r0 == *(row as *const u32) {
                        matched = true;
                        break;
                    }
                    k = k.wrapping_add(1);
                    row = row.wrapping_add(0x54);
                }
            }
            if !matched {
                *((sub.wrapping_add(0x14)) as *mut u32) = 5;
                *((sub.wrapping_add(0x18)) as *mut u32) = r0;
                cmd0(sub, this);
            }
            *(rowptr as *mut u32) = row;
            *((row.wrapping_add(0x14)) as *mut u32) = (r1 >> 4) & 0xF;
            *((row.wrapping_add(0x18)) as *mut u32) = r1 & 0xF;
            let sub2 = *(this as *const u32);
            *((sub2.wrapping_add(0x18)) as *mut u32) = r0;
            *((sub2.wrapping_add(0x1C)) as *mut u32) =
                *((row.wrapping_add(0x14)) as *const u32);
            *((sub2.wrapping_add(0x20)) as *mut u32) =
                *((row.wrapping_add(0x18)) as *const u32);
            *((sub2.wrapping_add(0x14)) as *mut u32) = 0x68;
            cmd1(sub2, this);
            rowptr = rowptr.wrapping_add(4);
            i = i.wrapping_add(1);
        }
        pre_check!(ptr, cnt, buf, this);
        *((this.wrapping_add(0x16C)) as *mut u32) = next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        *((this.wrapping_add(0x170)) as *mut u32) = next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        let t = *(ptr as *const u8) as u32;
        // No cursor advance after the last byte; the write-back compensates.
        *((this.wrapping_add(0x174)) as *mut u32) = (t >> 4) & 0xF;
        *((this.wrapping_add(0x178)) as *mut u32) = t & 0xF;
        let sub3 = *(this as *const u32);
        *((sub3.wrapping_add(0x18)) as *mut u32) =
            *((this.wrapping_add(0x16C)) as *const u32);
        *((sub3.wrapping_add(0x1C)) as *mut u32) =
            *((this.wrapping_add(0x170)) as *const u32);
        *((sub3.wrapping_add(0x20)) as *mut u32) =
            *((this.wrapping_add(0x174)) as *const u32);
        *((sub3.wrapping_add(0x24)) as *mut u32) =
            *((this.wrapping_add(0x178)) as *const u32);
        *((sub3.wrapping_add(0x14)) as *mut u32) = 0x69;
        cmd1(sub3, this);
        *((flag.wrapping_add(0x10)) as *mut u32) = 0;
        let c7c = *((this.wrapping_add(0x7C)) as *const u32);
        *((this.wrapping_add(0x7C)) as *mut u32) = c7c.wrapping_add(1);
        *(buf as *mut u32) = ptr.wrapping_add(1);
        *((buf.wrapping_add(4)) as *mut u32) = cnt.wrapping_sub(1);
        1
    }
});
