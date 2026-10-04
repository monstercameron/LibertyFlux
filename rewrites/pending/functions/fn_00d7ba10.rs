// original: 0x00d7ba10 ui_best_pick_test
use lf_checker_rt::{callee_thiscall, export, relocated};

const UI_SHARED_OBJECT: u32 = 0x01177A80;

/// Best-first UI pick test (original 0x00D7BA10).
///
/// Reads three coordinates out of the object's data block, asks a shared
/// helper (callee 1) whether the first two select anything, and on success
/// runs a wider query (callee 2) that fills three output coordinates plus a
/// status word. Combines the coordinate deltas with three weights from the
/// object's data block into a score, stores whether the score is positive
/// through `out_sign`, and marks `out_ok`.
///
/// Returns the `out_sign` pointer on success, callee 1's answer when its low
/// byte is zero, and callee 2's answer when the status word reads 0xFFFF.
/// The scratch addresses handed to callee 2 are this function's own frame
/// slots; the contract compares their contents, not the addresses.
export!(cdecl, rw_d7ba10(obj: u32, out_ok: *mut u8, out_sign: *mut u8) -> u32 {
    unsafe {
        let data = (obj.wrapping_add(0x20) as *const u32).read();
        let f0 = (data.wrapping_add(0x30) as *const f32).read();
        let f1 = (data.wrapping_add(0x34) as *const f32).read();
        let f2 = (data.wrapping_add(0x38) as *const f32).read();
        out_ok.write(0);
        let shared = relocated(UI_SHARED_OBJECT);
        let r1: u32 = callee_thiscall!(1, u32, shared, f0.to_bits(), f1.to_bits());
        if r1 & 0xFF == 0 {
            return r1;
        }
        let mut slot_f0 = f0;
        let mut status: u32 = 0xFFFF_FFFF;
        let mut status_hi: u32 = 0xFFFF_FFFF;
        let mut spare_hi: u32 = 0;
        let mut spare_lo: u32 = 0;
        let mut picked = [0u32; 3];
        let r2: u32 = callee_thiscall!(
            2, u32, shared,
            core::ptr::addr_of_mut!(slot_f0) as u32,
            core::ptr::addr_of_mut!(status) as u32,
            core::ptr::addr_of_mut!(status_hi) as u32,
            core::ptr::addr_of_mut!(spare_hi) as u32,
            core::ptr::addr_of_mut!(spare_lo) as u32,
            0,
            0x42480000,
            0, 0, 0, 0, 0, 0,
            picked.as_mut_ptr() as u32,
            0, 0,
        );
        if status & 0xFFFF == 0xFFFF {
            return r2;
        }
        let g0 = f32::from_bits(picked[1]) - f1;
        let g3 = f32::from_bits(picked[0]) - f0;
        let g2 = f32::from_bits(picked[2]) - f2;
        let w0 = (data.wrapping_add(0) as *const f32).read();
        let w4 = (data.wrapping_add(4) as *const f32).read();
        let w8 = (data.wrapping_add(8) as *const f32).read();
        let mut score = w4 * g0;
        score += w0 * g3;
        score += w8 * g2;
        out_sign.write(if score > 0.0 { 1 } else { 0 });
        out_ok.write(1);
        out_sign as u32
    }
});
