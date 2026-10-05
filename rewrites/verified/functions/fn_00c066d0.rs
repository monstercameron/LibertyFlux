// original: 0x00c066d0 stream_group_release (proposed)

/// Release the streaming group's buffers and its trailing array.
///
/// `this` points to the group. Every set entry of the array at `ITEMS`
/// (16-bit length at `COUNT`) goes to the release helper (callee 1), then the
/// array itself is released and both fields cleared. The three optional
/// buffers at `B0`, `B1` and `B2` are released when their matching flag words
/// at `G0`, `G1` and `G2` are set (the third release re-reads the cleared
/// array field, so it always passes null). When the tail count at `TAILC` is
/// set, each of its `TAILC` slots at the buffer at `TAIL` (stride `STRIDE`)
/// goes to the teardown helper (callee 2) and that buffer is released.
/// Returns the last release answer.
///
/// Original: 0x00c066d0 (thiscall, no stack words; 1 is cdecl, 2 thiscall).
lf_checker_rt::export!(thiscall, rw_00c066d0(this: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const B0: u32 = 0x18;
        const G0: u32 = 0x1e;
        const B1: u32 = 0x10;
        const G1: u32 = 0x16;
        const G2: u32 = 0x0e;
        const TAIL: u32 = 0x00;
        const TAILC: u32 = 0x06;
        const STRIDE: u32 = 0x50;
        const FREE: u32 = 1;
        const TEARDOWN: u32 = 2;
        let mut r = 0u32;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if (count as i32) > 0 {
            let mut i = 0u32;
            while (i as i32) < (count as i32) {
                let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
                let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
                if v != 0 {
                    r = lf_checker_rt::callee_cdecl!(FREE, u32, v);
                }
                i += 1;
            }
        }
        let arr = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
        r = lf_checker_rt::callee_cdecl!(FREE, u32, arr);
        (this.wrapping_add(ITEMS) as *mut u32).write_unaligned(0);
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(0);
        if (this.wrapping_add(G0) as *const u16).read_unaligned() != 0 {
            let b = (this.wrapping_add(B0) as *const u32).read_unaligned();
            r = lf_checker_rt::callee_cdecl!(FREE, u32, b);
        }
        if (this.wrapping_add(G1) as *const u16).read_unaligned() != 0 {
            let b = (this.wrapping_add(B1) as *const u32).read_unaligned();
            r = lf_checker_rt::callee_cdecl!(FREE, u32, b);
        }
        if (this.wrapping_add(G2) as *const u16).read_unaligned() != 0 {
            let b = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            r = lf_checker_rt::callee_cdecl!(FREE, u32, b);
        }
        let tailc = (this.wrapping_add(TAILC) as *const u16).read_unaligned() as u32;
        if tailc != 0 {
            let tail = (this.wrapping_add(TAIL) as *const u32).read_unaligned();
            if (tailc as i32) > 0 {
                let mut k = 0u32;
                while k < tailc {
                    let _s: u32 = lf_checker_rt::callee_thiscall!(
                        TEARDOWN, u32, tail.wrapping_add(k.wrapping_mul(STRIDE)));
                    k += 1;
                }
            }
            r = lf_checker_rt::callee_cdecl!(FREE, u32, tail);
        }
        r
    }
});
