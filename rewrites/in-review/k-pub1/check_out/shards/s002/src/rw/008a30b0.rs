// original: 0x008a30b0 rage::audMultitrackSound::vf5
/// Deleting destructor of `rage::audMultitrackSound` (vf5).
///
/// Same shape as [`rw_0089ff60`]: destructor body then conditional pool free.
export!(thiscall, rw_008a30b0(this: u32, flags: u32) -> u32 {
    unsafe {
        __lf_callee_thiscall!(1, u32, this);
        if flags & 1 != 0 && this != 0 {
            let bank = *((this as *const u8).add(0x40));
            __lf_callee_thiscall!(2, u32, relocated(SOUND_POOL), this, bank as u32);
        }
        this
    }
});

const SOUND_POOL: u32 = 0x115d8a0;

/// Convert a file VA (as seen in disassembly, image base 0x400000) to the
/// relocated address in the worker's mapping.
#[inline(always)]
pub fn relocated(file_va: u32) -> u32 {
    file_va.wrapping_sub(0x400000).wrapping_add(xbase())
}

/// Relocated base of the original image.
#[inline(always)]
pub fn xbase() -> u32 {
    unsafe { CHECKER_XBASE }
}
