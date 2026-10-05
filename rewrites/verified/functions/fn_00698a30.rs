// original: 0x00698A30 bitset_builder_698A30 (proposed)

/// Builds a bit set from an array of signed words: asks a counter how many
/// bits are needed, allocates that many (rounded up to whole words) through
/// the thread-local allocator, then sets one bit per array element plus a
/// run of bits per element driven by the element's low bits.
///
/// `this` carries the shift byte at `+8`. The first stack argument points at
/// a `{words: *u32, count: u16}` pair at `+0`/`+4`. The counter (callee 1,
/// fastcall: object in ecx, `1 << shift` in edx) answers the bit count N;
/// the full answer is stored at `this+4` and `ceil(N/32)` words are
/// allocated. Allocation goes through the thread allocator reached as
/// `tls[0] -> [+8] -> vtable[+8]` (callee 2, thiscall: object, byte size
/// with 32-bit overflow saturating to all-ones, `0x10`, `0`) and the fresh
/// words are zeroed. The main loop walks the array: a running index grows by
/// each element's absolute value shifted right by the shift byte, one bit is
/// set at the running index, then a further `shift` bits are set at the
/// following indexes wherever the element's masked low bits say so (the set
/// value rotates one step ahead of its index, wrapping inside the word at
/// word ends, exactly as the original shifts it), and a negative element
/// sets one more bit. Returns `(count & ~0xFF) | 1`.
///
/// All x86 shift counts are masked to five bits, matching the original's
/// `shl`/`shr`/`rol` semantics for large shift bytes.
///
/// Original: 0x00698A30 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00698A30(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SHIFT_OFF: u32 = 8;
        const COUNTER: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn shift_byte(this: u32) -> u32 {
            unsafe { ((this + SHIFT_OFF) as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn set_bit(base: u32, index: u32, value: u32) {
            unsafe {
                let addr = base.wrapping_add((index >> 5) * 4);
                wr32(addr, rd32(addr) | value);
            }
        }

        let sh = shift_byte(this);
        let n: u32 = lf_checker_rt::callee_fastcall!(COUNTER, u32, arg0, 1u32 << (sh & 31));
        wr32(this + 4, n);
        let dwords = (n >> 5) + ((n & 0x1f != 0) as u32);

        // Thread-allocator chain: tls slot 0 -> [+8] -> vtable slot +8.
        // (The slot value itself is what the original loads from fs:[0x2c].)
        let heap_obj = rd32(lf_checker_rt::tls_slot(0) + 8);
        let vtable = rd32(heap_obj);
        let size = dwords.checked_mul(4).unwrap_or(0xFFFF_FFFF);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 8) as usize);
        let bits = alloc(heap_obj, size, 0x10, 0);
        wr32(this, bits);
        for k in 0..dwords {
            wr32(bits.wrapping_add(k * 4), 0);
        }

        let base = rd32(arg0);
        let count = ((arg0 + 4) as *const u16).read_unaligned() as u32;
        let mut index: u32 = 0;
        let mut i: u32 = 0;
        while i < count {
            let w = rd32(base.wrapping_add(i * 4));
            let mag = if (w as i32) < 0 { 0u32.wrapping_sub(w) } else { w };
            let sh2 = shift_byte(this);
            index = index.wrapping_add(mag >> (sh2 & 31));
            let at = index;
            index = index.wrapping_add(1);
            set_bit(rd32(this), at, 1u32 << (at & 31));

            let sh3 = shift_byte(this);
            let mut window = 0xFFFF_FFFFu32 >> ((32u32.wrapping_sub(sh3)) & 31);
            let mut value = 1u32.rotate_left(index);
            window &= mag;
            if sh3 != 0 {
                let mut left = sh3;
                loop {
                    value = value.rotate_left(1);
                    let at_inner = index;
                    left -= 1;
                    index = index.wrapping_add(1);
                    if window & 1 != 0 {
                        set_bit(rd32(this), at_inner, value);
                    }
                    window >>= 1;
                    if left == 0 {
                        break;
                    }
                }
            }

            let w2 = rd32(base.wrapping_add(i * 4));
            if w2 != 0 {
                let at2 = index;
                index = index.wrapping_add(1);
                if (w2 as i32) < 0 {
                    set_bit(rd32(this), at2, 1u32 << (at2 & 31));
                }
            }
            i += 1;
        }
        (count & 0xFFFF_FF00) | 1
    }
});
