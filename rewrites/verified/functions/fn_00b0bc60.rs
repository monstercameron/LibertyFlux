// original: 0x00b0bc60 net_slot_copy_from (proposed)
//
//! Lane r-b303 rewrite crate (checker v4).

use lf_checker_rt::{callee_thiscall, export};

// original: 0x00b0bc60 net_slot_copy_from (proposed)

/// Copy one network table slot from `src` to `dst`, field by field.
///
/// `src` (ECX) and `dst` (first stack word) point at 0x4c-byte slot records.
/// When `dst` is null nothing happens and the original returns whatever was
/// in EAX on entry (unobservable to the rewrite, so the return channel is
/// not compared). Otherwise every field is copied across except:
/// - `+0x08`, which is passed to the slot-commit callee instead; the callee
///   stores it at `dst+0x48` (observed through the stub's scripted write).
/// - `+0x28` and `+0x2c`, which are never touched.
/// - `+0x45`, whose seven low bits are merged one at a time and whose top
///   bit is then taken from the source too, so the net effect is a full
///   byte copy.
/// - `+0x46`, whose low three bits come from the source while the top five
///   bits of the destination are preserved.
///
/// The two words at `+0x34`/`+0x38` move through vector registers but no
/// arithmetic is done on them, so the copy is bitwise either way.
///
/// Original: 0x00b0bc60 (thiscall, one stack word, callee pops it).
export!(thiscall, rw_00b0bc60(src: u32, dst: u32) -> u32 {
    unsafe {
        const FLAG_FULL: u32 = 0x45;
        const FLAG_PART: u32 = 0x46;
        const PART_MASK: u8 = 0x07;
        const COMMIT_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if dst == 0 {
            return 0;
        }
        wr32(dst, rd32(src));
        wr32(dst + 4, rd32(src + 4));
        let commit_arg: u32 = rd32(src + 8);
        let _: u32 = callee_thiscall!(COMMIT_CALLEE, u32, dst, commit_arg);
        for i in 0..7u32 {
            wr32(dst + 8 + i * 4, rd32(src + 0x0c + i * 4));
        }
        wr32(dst + 0x30, rd32(src + 0x30));
        wr32(dst + 0x34, rd32(src + 0x34));
        wr32(dst + 0x38, rd32(src + 0x38));
        wr32(dst + 0x3c, rd32(src + 0x3c));
        let w40 = (src + 0x40) as *const u16;
        ((dst + 0x40) as *mut u16).write_unaligned(w40.read_unaligned());
        let w42 = (src + 0x42) as *const u16;
        ((dst + 0x42) as *mut u16).write_unaligned(w42.read_unaligned());
        ((dst + 0x44) as *mut u8).write(((src + 0x44) as *const u8).read());
        let full = ((src + FLAG_FULL) as *const u8).read();
        ((dst + FLAG_FULL) as *mut u8).write(full);
        let kept = ((dst + FLAG_PART) as *const u8).read() & !PART_MASK;
        let took = ((src + FLAG_PART) as *const u8).read() & PART_MASK;
        let merged = kept | took;
        ((dst + FLAG_PART) as *mut u8).write(merged);
        merged as u32
    }
});
