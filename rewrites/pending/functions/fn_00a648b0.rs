// original: 0x00a648b0 latch_proximity_sample
use lf_checker_rt::{callee_cdecl, export};

/// Latch one proximity sample once it repeats inside tolerance.
///
/// Compares the incoming triple against the stored one axis by axis with a
/// tight window on x/y and a wide window on z. A sample outside any window
/// replaces the stored triple and restarts the streak; a sample inside all
/// three extends the streak, and the third consecutive in-tolerance sample
/// is committed through the sink and clears the latch. The commit reloads
/// one word from a frame slot the function never stores to, which the
/// checker defines as the stack fill (zero); the latch slot is cleared
/// with it. Returns the sink's answer on commit, the stored fourth word on
/// replace, and the sample pointer while the streak is short.
export!(thiscall, rw_00a648b0(this: u32, sample: u32) -> u32 {
    unsafe {
        let esi = this;
        let fresh0 = f32::from_bits((sample as *const u32).read());
        let cur0 = f32::from_bits(((esi + 0x2B0) as *const u32).read());
        if !(0.1f32 > (fresh0 - cur0).abs()) {
            return replace_sample(esi, sample, fresh0);
        }
        let fresh1 = f32::from_bits(((sample + 4) as *const u32).read());
        let cur1 = f32::from_bits(((esi + 0x2B4) as *const u32).read());
        if !(0.1f32 > (fresh1 - cur1).abs()) {
            return replace_sample(esi, sample, fresh0);
        }
        let fresh2 = f32::from_bits(((sample + 8) as *const u32).read());
        let cur2 = f32::from_bits(((esi + 0x2B8) as *const u32).read());
        if !(2.0f32 > (fresh2 - cur2).abs()) {
            return replace_sample(esi, sample, fresh0);
        }
        let streak = (esi + 0x2C0) as *mut u32;
        let n = streak.read().wrapping_add(1);
        streak.write(n);
        if (n as i32) < 3 {
            return sample;
        }
        let answer = callee_cdecl!(1, u32, sample);
        // The original reloads [esp+0x20] here, a frame slot it never
        // stored to; under the checker's defined stack fill that word is 0.
        ((esi + 0x2B0) as *mut u32).write(0);
        ((esi + 0x2B4) as *mut u32).write(0);
        ((esi + 0x2B8) as *mut u32).write(0);
        ((esi + 0x2BC) as *mut u32).write(0);
        ((esi + 0x2C0) as *mut u32).write(0);
        answer
    }
});

/// Replace the stored sample and restart the streak at one.
#[inline(always)]
fn replace_sample(esi: u32, sample: u32, fresh0: f32) -> u32 {
    unsafe {
        let fresh1 = f32::from_bits(((sample + 4) as *const u32).read());
        let fresh2 = f32::from_bits(((sample + 8) as *const u32).read());
        ((esi + 0x2B0) as *mut u32).write(fresh0.to_bits());
        ((esi + 0x2B4) as *mut u32).write(fresh1.to_bits());
        ((esi + 0x2B8) as *mut u32).write(fresh2.to_bits());
        let tag = ((sample + 0xC) as *const u32).read();
        ((esi + 0x2BC) as *mut u32).write(tag);
        ((esi + 0x2C0) as *mut u32).write(1);
        tag
    }
}
