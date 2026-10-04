// original: 0x006A7490 input_event_parse_a
/// Parse one input event record from a refillable byte stream into `this`
/// and its attached display object.
///
/// `this` (ECX) owns a buffer cursor at +0x18 pointing at `{ptr, count,
/// refill}`; `dlow` (EDX) and `stk` (stack) are two tag bytes stored to
/// +0xC8/+0xC9. Eight header bytes are read (refilling through the
/// `{ptr+0xC}` callback with `this` whenever the count runs out; a zero
/// answer aborts with 0): a big-endian 16-bit gauge, a single tag byte at
/// +0xC0, two big-endian 16-bit dimensions at +0x20/+0x1C and a row count
/// at +0x24. The gauge (minus 8) and the dimensions are published to the
/// display object at `[this]`, then a fixed command sequence runs:
/// command 0x64, command 0x3A when the status flag at `[this+0x194]+0xD`
/// is set, command 0x20 unless all three of gauge-dimensions/row-count are
/// positive, and command 0x0B unless the gauge equals three times the row
/// count. A row array (`count * 0x54` bytes) is then obtained through the
/// allocator at `[this+4]` unless +0xC4 already holds one, and each row is
/// filled from four more stream bytes (value, two nibbles, value),
/// mirrored into the display object and announced with command 0x65. On
/// success the stream cursor is written back, the status flag is set and 1
/// is returned; any refused refill returns 0 with no write-back.
///
/// Only the low byte of the return value is significant.
lf_checker_rt::export!(fastcall, rb200_fn2(this: u32, dlow: u32, stk: u32) -> u32 {
    /// Refill callback through the fabricated buffer object, exactly like
    /// the original: load `[buf+0xC]` and call it with `this` (cdecl/1).
    /// Returns the reloaded (ptr, count) or None when refused (al == 0).
    #[inline(always)]
    unsafe fn refill(buf: u32, this: u32) -> Option<(u32, u32)> {
        let target = *((buf.wrapping_add(0xC)) as *const u32);
        let f: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if (f(this) & 0xFF) == 0 {
            return None;
        }
        let p = *(buf as *const u32);
        let c = *((buf.wrapping_add(4)) as *const u32);
        Some((p, c))
    }

    /// Command call through slot 0 of `[obj]` (cdecl/1) with `this`.
    #[inline(always)]
    unsafe fn cmd0(obj: u32, this: u32) {
        let target = *(obj as *const u32);
        let f: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this);
    }

    /// Command call through slot 1 of `[obj]` (cdecl/2) with (`this`, 1).
    #[inline(always)]
    unsafe fn cmd1(obj: u32, this: u32) {
        let target = *((obj.wrapping_add(4)) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, 1);
    }

    /// Row-array allocation through slot 0 of `[obj]` (cdecl/3).
    #[inline(always)]
    unsafe fn alloc_rows(obj: u32, this: u32, size: u32) -> u32 {
        let target = *(obj as *const u32);
        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, 1, size)
    }

    /// Read one stream byte, advancing the cursor. The caller owns the
    /// refill checks, placed exactly where the original has them.
    macro_rules! next_byte {
        ($ptr:ident, $cnt:ident) => {{
            let b = *($ptr as *const u8);
            $ptr = $ptr.wrapping_add(1);
            $cnt = $cnt.wrapping_sub(1);
            b as u32
        }};
    }

    /// Refill check after a read: on an exhausted count ask for more;
    /// return 0 when refused.
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

    unsafe {
        *(this.wrapping_add(0xC8) as *mut u8) = dlow as u8;
        *(this.wrapping_add(0xC9) as *mut u8) = stk as u8;
        let buf = *((this.wrapping_add(0x18)) as *const u32);
        let mut ptr = *(buf as *const u32);
        let mut cnt = *((buf.wrapping_add(4)) as *const u32);
        if cnt == 0 {
            match refill(buf, this) {
                Some((p, c)) => {
                    ptr = p;
                    cnt = c;
                }
                None => return 0,
            }
        }
        // Eight header bytes.
        let mut gauge = next_byte!(ptr, cnt) << 8;
        post_check!(ptr, cnt, buf, this);
        gauge += next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        *((this.wrapping_add(0xC0)) as *mut u32) = next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        let dim_w_hi = next_byte!(ptr, cnt);
        // The original stores the high half first, then adds the low half.
        *((this.wrapping_add(0x20)) as *mut u32) = dim_w_hi << 8;
        post_check!(ptr, cnt, buf, this);
        let dim_w = (dim_w_hi << 8) + next_byte!(ptr, cnt);
        *((this.wrapping_add(0x20)) as *mut u32) = dim_w;
        post_check!(ptr, cnt, buf, this);
        let dim_h_hi = next_byte!(ptr, cnt);
        *((this.wrapping_add(0x1C)) as *mut u32) = dim_h_hi << 8;
        post_check!(ptr, cnt, buf, this);
        let dim_h = (dim_h_hi << 8) + next_byte!(ptr, cnt);
        *((this.wrapping_add(0x1C)) as *mut u32) = dim_h;
        post_check!(ptr, cnt, buf, this);
        let rows = next_byte!(ptr, cnt);
        *((this.wrapping_add(0x24)) as *mut u32) = rows;
        // No refill check after the eighth byte (the count/test pair that
        // follows the command below observes the same values either way).
        let sub = *(this as *const u32);
        gauge = gauge.wrapping_sub(8);
        *((sub.wrapping_add(0x18)) as *mut u32) =
            *((this.wrapping_add(0x17C)) as *const u32);
        *((sub.wrapping_add(0x1C)) as *mut u32) = dim_h;
        *((sub.wrapping_add(0x20)) as *mut u32) = dim_w;
        *((sub.wrapping_add(0x24)) as *mut u32) = rows;
        *((sub.wrapping_add(0x14)) as *mut u32) = 0x64;
        cmd1(sub, this);
        let flag = *((this.wrapping_add(0x194)) as *const u32);
        if *((flag.wrapping_add(0xD)) as *const u8) != 0 {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x3A;
            cmd0(sub, this);
        }
        if !(dim_w > 0 && dim_h > 0 && rows > 0) {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x20;
            cmd0(sub, this);
        }
        if gauge != rows.wrapping_mul(3) {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x0B;
            cmd0(sub, this);
        }
        let mut arr = *((this.wrapping_add(0xC4)) as *const u32);
        if arr == 0 {
            let wobj = *((this.wrapping_add(4)) as *const u32);
            arr = alloc_rows(wobj, this, rows.wrapping_mul(0x54));
            *((this.wrapping_add(0xC4)) as *mut u32) = arr;
        }
        let mut i = 0u32;
        while i < rows {
            let row = arr.wrapping_add(i.wrapping_mul(0x54));
            *((row.wrapping_add(4)) as *mut u32) = i;
            if cnt == 0 {
                match refill(buf, this) {
                    Some((p, c)) => {
                        ptr = p;
                        cnt = c;
                    }
                    None => return 0,
                }
            }
            *(row as *mut u32) = next_byte!(ptr, cnt);
            post_check!(ptr, cnt, buf, this);
            let packed = next_byte!(ptr, cnt);
            *((row.wrapping_add(8)) as *mut u32) = (packed >> 4) & 0xF;
            *((row.wrapping_add(0xC)) as *mut u32) = packed & 0xF;
            post_check!(ptr, cnt, buf, this);
            *((row.wrapping_add(0x10)) as *mut u32) = next_byte!(ptr, cnt);
            // No refill check after the fourth byte (same note as above).
            let sub2 = *(this as *const u32);
            *((sub2.wrapping_add(0x18)) as *mut u32) = *(row as *const u32);
            *((sub2.wrapping_add(0x1C)) as *mut u32) =
                *((row.wrapping_add(8)) as *const u32);
            *((sub2.wrapping_add(0x20)) as *mut u32) =
                *((row.wrapping_add(0xC)) as *const u32);
            *((sub2.wrapping_add(0x24)) as *mut u32) =
                *((row.wrapping_add(0x10)) as *const u32);
            *((sub2.wrapping_add(0x14)) as *mut u32) = 0x65;
            cmd1(sub2, this);
            i = i.wrapping_add(1);
        }
        *((flag.wrapping_add(0xD)) as *mut u8) = 1;
        *(buf as *mut u32) = ptr;
        *((buf.wrapping_add(4)) as *mut u32) = cnt;
        1
    }
});
