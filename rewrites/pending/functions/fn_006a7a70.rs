// original: 0x006A7A70 input_event_parse_c
/// Parse one input event record in budget rounds.
///
/// Same stream/this shape as [`rb200_fn2`] but the body runs in rounds
/// while a signed budget stays positive: two header bytes form the budget
/// `(b0 << 8) - 2 + b1`. Each round reads a selector byte (command 0x50),
/// sixteen payload bytes into a frame buffer (summed), publishes the two
/// halves with command 0x56 (second argument 2), fires command 8 when the
/// sum exceeds 256 or the remaining budget, reads `sum` more bytes into a
/// second 256-byte frame buffer, spends `0x11 + sum` of budget, resolves a
/// row slot from the selector (`[this + 4*(n + 0x28)]`, or `+ 0x1C` with
/// the high nibble masked when bit 4 is set; command 0x1E unless the
/// adjusted selector is within 0..4), allocates the row through
/// `[this+4]` when the slot is empty, stores the first buffer's seventeen
/// bytes and copies all 256 second-buffer bytes onto the row. The second
/// buffer's unwritten tail reads the defined zero fill (the contract sets
/// `stack_fill` 0, matching the worker's scratch reset), except bytes a
/// previous round wrote persist — the buffer lives in the frame for the
/// whole call, so each round's copy carries earlier rounds' bytes past
/// its own sum. When the budget
/// reaches 0x10 or less, command 0x0B fires unless it is exactly zero,
/// the stream cursor is written back and 1 is returned; any refused
/// refill returns 0 with no write-back. Only the low byte is significant.
lf_checker_rt::export!(thiscall, rb200_fn4(this: u32) -> u32 {
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
    unsafe fn cmd1(obj: u32, this: u32, tag: u32) {
        let target = *((obj.wrapping_add(4)) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, tag);
    }
    #[inline(always)]
    unsafe fn alloc_row(obj: u32, this: u32) -> u32 {
        let target = *(obj as *const u32);
        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, 0, 0x112)
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
        pre_check!(ptr, cnt, buf, this);
        let b0 = next_byte!(ptr, cnt);
        post_check!(ptr, cnt, buf, this);
        let b1 = next_byte!(ptr, cnt);
        // No refill check after the second byte.
        let mut acc = (b0 << 8).wrapping_sub(2).wrapping_add(b1) as i32;
        // The 256-byte buffer lives in the frame for the whole call: the
        // zero fill models the worker's defined scratch reset for bytes no
        // round ever writes, while bytes written by one round persist into
        // later rounds' copies for indices past their own sum.
        let mut blk2 = [0u8; 256];
        loop {
            if acc <= 0x10 {
                break;
            }
            pre_check!(ptr, cnt, buf, this);
            let n = next_byte!(ptr, cnt);
            // No refill check after the selector byte.
            let sub = *(this as *const u32);
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x50;
            *((sub.wrapping_add(0x18)) as *mut u32) = n;
            cmd1(sub, this, 1);
            let mut blk = [0u8; 17];
            let mut sum = 0u32;
            let mut k = 1usize;
            while k <= 16 {
                pre_check!(ptr, cnt, buf, this);
                let b = next_byte!(ptr, cnt);
                // No refill check inside the block loop.
                blk[k] = b as u8;
                sum = sum.wrapping_add(b);
                k += 1;
            }
            let mut j = 0usize;
            while j < 8 {
                *((sub.wrapping_add(0x18 + j as u32 * 4)) as *mut u32) =
                    blk[1 + j] as u32;
                j += 1;
            }
            acc = acc.wrapping_sub(0x11);
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x56;
            cmd1(sub, this, 2);
            let mut j = 0usize;
            while j < 8 {
                *((sub.wrapping_add(0x18 + j as u32 * 4)) as *mut u32) =
                    blk[9 + j] as u32;
                j += 1;
            }
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x56;
            cmd1(sub, this, 2);
            if sum > 0x100 || (sum as i32) > acc {
                *((sub.wrapping_add(0x14)) as *mut u32) = 8;
                cmd0(sub, this);
            }
            let mut k = 0u32;
            while k < sum {
                pre_check!(ptr, cnt, buf, this);
                blk2[k as usize] = next_byte!(ptr, cnt) as u8;
                k = k.wrapping_add(1);
            }
            acc = acc.wrapping_sub(sum as i32);
            let adjusted = if n & 0x10 != 0 { n.wrapping_sub(0x10) } else { n };
            let index = if n & 0x10 != 0 {
                n.wrapping_add(0x1C)
            } else {
                n.wrapping_add(0x28)
            };
            let slot = this.wrapping_add(index.wrapping_mul(4));
            if (adjusted as i32) < 0 || (adjusted as i32) >= 4 {
                *((sub.wrapping_add(0x14)) as *mut u32) = 0x1E;
                *((sub.wrapping_add(0x18)) as *mut u32) = adjusted;
                cmd0(sub, this);
            }
            let mut row = *(slot as *const u32);
            if row == 0 {
                let wobj = *((this.wrapping_add(4)) as *const u32);
                row = alloc_row(wobj, this);
                *((row.wrapping_add(0x111)) as *mut u8) = 0;
                *(slot as *mut u32) = row;
            }
            core::ptr::copy_nonoverlapping(blk.as_ptr(), row as *mut u8, 17);
            core::ptr::copy_nonoverlapping(
                blk2.as_ptr(),
                (row.wrapping_add(0x11)) as *mut u8,
                256,
            );
        }
        let sub = *(this as *const u32);
        if acc != 0 {
            *((sub.wrapping_add(0x14)) as *mut u32) = 0x0B;
            cmd0(sub, this);
        }
        *(buf as *mut u32) = ptr;
        *((buf.wrapping_add(4)) as *mut u32) = cnt;
        1
    }
});
