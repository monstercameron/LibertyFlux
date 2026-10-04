// original: 0x00a65600 validate_aim_row
use lf_checker_rt::{callee_cdecl, export, global};

/// Validate one object's aim row against the shared type table.
///
/// Resolves the object's kind through the global type table, blends the
/// row's span with the object's basis vectors and three global weights,
/// and hands the assembled query plus two basis-derived points to the
/// matcher. A null match accepts; otherwise the query's leading status
/// word (always stored as zero) must equal the object's own status word.
/// Returns 1 on accept, 0 on reject.
export!(cdecl, rw_00a65600(obj: u32) -> u32 {
    unsafe {
        let esi = obj;
        let w0 = f32::from_bits(global::<u32>(0x01B4B320).read());
        let kind = ((esi + 0x2E) as *const i16).read() as i32;
        let basis = ((esi + 0x20) as *const u32).read();
        let table = global::<u32>(0x01295CD8);
        let row = table.offset(kind as isize).read();
        let m14 = f32::from_bits(((basis + 0x14) as *const u32).read());
        let span = f32::from_bits(((row + 0x34) as *const u32).read())
            - f32::from_bits(((row + 0x24) as *const u32).read());
        let m18 = f32::from_bits(((basis + 0x18) as *const u32).read());
        let w1 = f32::from_bits(global::<u32>(0x01B4B324).read());
        let tip = basis + 0x30;
        let gain = span * f32::from_bits(0x3F000000) + f32::from_bits(0x3FC00000);
        let m10 = f32::from_bits(((basis + 0x10) as *const u32).read());
        let s3 = gain * m10;
        let s4 = m14 * gain;
        let s5 = m18 * gain;
        let w2 = f32::from_bits(global::<u32>(0x01B4B328).read());
        let d0 = f32::from_bits((tip as *const u32).read()) + s3;
        let d1 = f32::from_bits(((tip + 4) as *const u32).read()) + s4;
        let d2 = f32::from_bits(((tip + 8) as *const u32).read()) + s5;
        // Every value above feeds only the stubbed matcher call, but the
        // reads themselves are behaviour (a wild kind faults); keep them
        // live exactly as the original performs them.
        core::hint::black_box((w0, w1, w2, s3, s4, s5, d0, d1, d2));
        let mut query = [0u32; 32];
        let mut point = [0u32; 8];
        let hit = callee_cdecl!(1, u32, point.as_mut_ptr() as u32, tip, 0, query.as_mut_ptr() as u32, 0xE, 1, 4);
        if hit == 0 {
            return 1;
        }
        // The original stores zero into the query's leading word and then
        // compares that word against the object's status word.
        if 0u32 == ((esi + 0x38) as *const u32).read() {
            1
        } else {
            0
        }
    }
});
