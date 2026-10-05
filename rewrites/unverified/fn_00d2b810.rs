// original: 0x00D2B810 CTaskComplexMoveFollowNavMeshRoute::vf1 (symbols)

/// Clone this task's movement parameters into a fresh subtask.
///
/// `this` is the route-following task. A singleton fetched through the
/// task system's global provides (via its first callee) the object the
/// second callee builds the subtask from; when that first answer is null
/// the clone step is skipped. The builder callee receives ten stack words:
/// three flag bits of the state word at `+0xD8` (bits 7, 5 and 4), the
/// words at `+0x18/+0x50/+0x54/+0x5c` (floats, passed as bits), the words
/// at `+0x9c/+0xe0`, and a pointer at this task's `+0x20`.
///
/// The returned subtask then receives copies of the words at `+0x58`,
/// `+0x60` and `+0x64`, thirteen single flag bits of `+0xD8`, five single
/// flag bits of `+0xDC`, and — only when the source's bit 0x40000000 is
/// set — that bit itself plus the float at `+0x68` and the half word at
/// `+0xd4`. Every other bit of the subtask's flag words is left alone.
/// The result is the subtask pointer.
///
/// The singleton global is read through an unrelocated absolute address;
/// the proof pins it to its pristine (null) value and the behaviour is
/// parametric in it (it is only passed on to the first callee).
///
/// Original: 0x00D2B810 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d2b810(this: u32) -> u32 {
    unsafe {
        /// File VA of the task-system singleton pointer (read unrelocated
        /// by the original; pristine value null).
        const SINGLETON: u32 = 0x167E2A0;
        const FLAGS: u32 = 0xD8;
        const FLAGS2: u32 = 0xDC;
        const BUILDER_THIS: u32 = 1;
        const BUILDER: u32 = 2;
        /// Single flag bits copied from `+0xD8`.
        const D8_BITS: [u32; 13] = [
            0x2000, 0x8000, 0x100000, 0x200000, 0x400000, 0x800000, 0x1000000, 0x4000000,
            0x8000000, 0x10000000, 0x20000000, 0x2000000, 0x80000000,
        ];
        /// Single flag bits copied from `+0xDC`.
        const DC_BITS: [u32; 5] = [0x1, 0x4, 0x10, 0x40, 0x20];
        /// Bit that is set (never cleared) together with the extra fields.
        const EXTRA_BIT: u32 = 0x40000000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            (a as *const u32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            (a as *mut u32).write_unaligned(v)
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            (a as *const u16).read_unaligned()
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            (a as *mut u16).write_unaligned(v)
        }
        /// Copy single bits from `s` into `d`, leaving other bits alone.
        #[inline(always)]
        fn copy_bits(s: u32, d: u32, masks: &[u32]) -> u32 {
            let mut d = d;
            let mut i = 0;
            while i < masks.len() {
                let m = masks[i];
                d = (d & !m) | (s & m);
                i += 1;
            }
            d
        }

        let singleton: u32 = *lf_checker_rt::global::<u32>(SINGLETON);
        let src: u32 = lf_checker_rt::callee_thiscall!(BUILDER_THIS, u32, singleton);
        let sub: u32;
        if src == 0 {
            sub = 0;
        } else {
            let flags = rd32(this + FLAGS);
            sub = lf_checker_rt::callee_thiscall!(
                BUILDER,
                u32,
                src,
                rd32(this + 0x18),
                this.wrapping_add(0x20),
                rd32(this + 0x50),
                rd32(this + 0x54),
                rd32(this + 0x9C),
                (flags >> 4) & 1,
                (flags >> 5) & 1,
                rd32(this + 0xE0),
                rd32(this + 0x5C),
                (flags >> 7) & 1
            );
        }
        wr32(sub + 0x58, rd32(this + 0x58));
        let s = rd32(this + FLAGS);
        let mut d = copy_bits(s, rd32(sub + FLAGS), &D8_BITS);
        wr32(sub + 0x60, rd32(this + 0x60));
        wr32(sub + 0x64, rd32(this + 0x64));
        wr32(sub + FLAGS, d);
        wr32(
            sub + FLAGS2,
            copy_bits(rd32(this + FLAGS2), rd32(sub + FLAGS2), &DC_BITS),
        );
        if s & EXTRA_BIT != 0 {
            d |= EXTRA_BIT;
            wr32(sub + FLAGS, d);
            wr32(sub + 0x68, rd32(this + 0x68));
            wr16(sub + 0xD4, rd16(this + 0xD4));
        }
        sub
    }
});
