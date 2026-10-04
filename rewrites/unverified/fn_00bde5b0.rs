// original: 0x00BDE5B0 task_airspace_probe_direct (proposed)

/// Probe whether a ped task's altitude slot clears a scripted ceiling,
/// calling the grid probe directly instead of through its ten-word wrapper.
///
/// Same contract as its sibling at 0x00BDE420: `x`, `y`, `z` are
/// single-precision coordinates and `esi` points at the task's altitude
/// word. The grid probe callee (id 1, cdecl) is called here with seven
/// stack words (`y`, `z`, `esi`, `6.0f`, `0x497423fe`, `0`, and one word of
/// uninitialised stack that reads back as the trial fill). A zero low byte
/// returns 0.
///
/// Otherwise the function builds the same five-word sample `[x, y, 0.0, x,
/// y]` (`z` is dropped the same way: the slot is reloaded from a zeroed
/// stack word) and eight-word result object, and calls the same range-list
/// callee (id 2, thiscall on the global list head) with the same eight
/// stack words. The callee stores one result word at result offset 28. A
/// zero answer returns 1; otherwise the function returns whether the
/// altitude word is ordered above the result word (`comiss` + `seta`, false
/// for NaN on either side).
///
/// The three globals at 0x1B4B320 are copied to frame scratch and never
/// re-read, so they have no observable effect. Original: 0x00BDE5B0
/// (cdecl, four stack words, byte result in `al`).
lf_checker_rt::export!(cdecl, rw_00BDE5B0(x: u32, y: u32, z: u32, esi: u32) -> u32 {
    unsafe {
        const RADIUS: u32 = 0x40c00000; // 6.0f
        const SCALE: u32 = 0x497423fe;
        const LIST_HEAD: u32 = 0x012b9c78;
        const RESULT_AT: usize = 7; // result word the callee fills, at +28

        let inside: u32 =
            lf_checker_rt::callee_cdecl!(1, u32, y, z, esi, RADIUS, SCALE, 0, 0);
        if inside & 0xFF == 0 {
            return 0;
        }
        let mut sample = [x, y, 0u32, x, y];
        let mut result = [0u32; 8];
        let list: u32 = *lf_checker_rt::global::<u32>(LIST_HEAD);
        let found: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            list,
            sample.as_mut_ptr() as u32,
            result.as_mut_ptr() as u32,
            0,
            6,
            0xFFFF_FFFF,
            7,
            1,
            0
        );
        if found == 0 {
            return 1;
        }
        let level = f32::from_bits((esi as *const u32).read_unaligned());
        let limit = f32::from_bits(result[RESULT_AT]);
        if level > limit { 1 } else { 0 }
    }
});
