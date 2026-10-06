// original: 0x00664BE0 sn_task_alloc_copy_strings (proposed)

/// Allocate a member array, copy the caller's entries into it, and stamp two
/// names on the task object.
///
/// `this` is a network task object: dword at `+0x60` points at a context whose
/// first word is an allocator object; dword at `+0x98` receives the new array,
/// dword at `+0x9c` its count, and bytes at `+0xa0` (15) and `+0xb0` (127)
/// receive two NUL-terminated names. `src` points at `n` caller entries of
/// 16 bytes each (`n` is SIGNED: zero or negative allocates and copies
/// nothing); `name` is an optional NUL-terminated string, the first stack
/// word and the fourth are not read.
///
/// Behaviour: clear the count, then ask the allocator (virtual slot `+8` of
/// the object at the context's first word, called thiscall with
/// `(n << 4, 0, 0)`) for the array. When it answers null, return the low byte
/// of the incoming `this` (the original falls through to a byte load of its
/// own saved register slot). Otherwise zero the first 10 bytes of each of
/// the `n` 16-byte slots, copy all `n` entries over them, store `n`, copy up
/// to 15 bytes of the fixed tag plus NUL, copy up to 127 bytes of `name` (or
/// the fixed default when null) plus NUL, and return the destination end
/// with its low byte set to 1.
///
/// Original: 0x00664BE0 (thiscall, five stack words; words 0 and 3 unread).
lf_checker_rt::export!(thiscall, rw_00664BE0(this: u32, _a0: u32, src: u32, n: u32, _a3: u32, name: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const ARRAY: u32 = 0x98;
        const COUNT: u32 = 0x9c;
        const TAG: u32 = 0xa0;
        const NAME: u32 = 0xb0;
        const ENTRY: u32 = 16;
        const ZEROED: u32 = 10;
        const TAG_MAX: u32 = 15;
        const NAME_MAX: u32 = 0x7f;
        const ALLOC_SLOT: u32 = 8;
        const FIXED_TAG: u32 = 0x00F1_C17E;
        const FIXED_NAME: u32 = 0x00F1_C17F;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        /// Copy at most `max` bytes up to (not including) the first NUL, then
        /// a NUL; returns the address just past the copied bytes.
        unsafe fn stamp(mut dst: u32, mut s: u32, max: u32) -> u32 {
            unsafe {
                let mut left = max;
                while left != 0 {
                    let b = rd8(s);
                    if b == 0 {
                        break;
                    }
                    wr8(dst, b);
                    dst = dst.wrapping_add(1);
                    s = s.wrapping_add(1);
                    left -= 1;
                }
                wr8(dst, 0);
                dst
            }
        }

        wr32(this.wrapping_add(COUNT), 0);
        let inner = rd32(this.wrapping_add(CTX));
        let obj = rd32(inner);
        let slot = rd32(rd32(obj).wrapping_add(ALLOC_SLOT));
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let p = alloc(obj, n << 4, 0, 0);
        wr32(this.wrapping_add(ARRAY), p);
        if p == 0 {
            return this & 0xff;
        }
        let count = n as i32;
        if count > 0 {
            let mut q = p;
            for _ in 0..count {
                for i in 0..ZEROED {
                    wr8(q.wrapping_add(i), 0);
                }
                q = q.wrapping_add(ENTRY);
            }
            let mut k = 0u32;
            for _ in 0..count {
                for i in 0..ENTRY {
                    wr8(p.wrapping_add(k).wrapping_add(i), rd8(src.wrapping_add(k).wrapping_add(i)));
                }
                k = k.wrapping_add(ENTRY);
            }
        }
        wr32(this.wrapping_add(COUNT), n);
        stamp(this.wrapping_add(TAG), lf_checker_rt::relocated(FIXED_TAG), TAG_MAX);
        let s2 = if name != 0 { name } else { lf_checker_rt::relocated(FIXED_NAME) };
        let end = if s2 != 0 {
            stamp(this.wrapping_add(NAME), s2, NAME_MAX)
        } else {
            let dst = this.wrapping_add(NAME);
            wr8(dst, 0);
            dst
        };
        (end & 0xffff_ff00) | 1
    }
});
