// original: 0x00b18f70 ccutsceneobject_vf57 (symbols)

/// Applies the cutscene object's mode-0x22B placement update.
///
/// Thiscall of two stack words. With mode byte zero the two arguments
/// go straight to the placer (thiscall of two words). Otherwise the
/// anchor triple is read from the target block (+0x30 past the pointer
/// at +0x20) or, when that pointer is null, from +0x10 in the object;
/// the placer runs with the same two arguments, and for modes 2, 3, 5
/// and 6 that is all. Any other nonzero mode additionally runs the
/// pose writer (thiscall of the anchor triple's address and two ones).
/// Returns the last call's result, except on the mode 2/3/5/6 path where
/// the mode test has replaced the low byte of the placer's answer with
/// the mode itself.
lf_checker_rt::export!(thiscall, rw_00b18f70(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x22b;
        const TARGET_OFF: u32 = 0x20;
        const ANCHOR_OFF: u32 = 0x30;
        const LOCAL_ANCHOR: u32 = 0x10;
        let mode = ((this + MODE_OFF) as *const u8).read();
        if mode == 0 {
            return lf_checker_rt::callee_thiscall!(1, u32, this, a, b);
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        let src = if target != 0 {
            target.wrapping_add(ANCHOR_OFF)
        } else {
            this.wrapping_add(LOCAL_ANCHOR)
        };
        let answer: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a, b);
        if mode == 2 || mode == 5 || mode == 3 || mode == 6 {
            // The mode test reloads AL, so the low byte of the placer's
            // answer is replaced by the mode on this path.
            return (answer & 0xffff_ff00) | mode as u32;
        }
        let mut anchor = [0u32; 3];
        anchor[0] = (src as *const u32).read_unaligned();
        anchor[1] = ((src + 4) as *const u32).read_unaligned();
        anchor[2] = ((src + 8) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, this, anchor.as_mut_ptr() as u32, 1, 1)
    }
});
