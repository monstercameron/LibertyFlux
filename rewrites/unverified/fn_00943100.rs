// original: 0x943100 SG_REMINDER_check_and_show
/// Reminder dialog gate: shows a "reminder" dialog while two polled
/// quantities still sum within a threshold.
///
/// Behaviour: if the feature flag at `FLAG` is not 1, returns the incoming
/// `eax` unchanged (entry `eax` is pinned to 0 by the contract). Otherwise
/// polls two float meters through the cdecl getter (selector ids
/// `METER_A`/`METER_B`), sums them and compares against `THRESHOLD` (12.0):
/// a sum above the threshold, or an unordered (NaN) comparison, exits
/// returning the second meter's bits (the getter also leaves its bits in
/// `eax`). Otherwise fetches the owner object (selector 0), picks one of two
/// dialog templates from the flag byte at `OWNER_TEMPLATE_FLAG`, shows it
/// through the 14-word dialog call on `DIALOG_OWNER`, and reports the shown
/// handle to `REPORT_OWNER`, returning that call's result.
///
/// Convention: cdecl, no arguments. The 14 dialog words are fixed constants
/// (a trailing -1 sentinel first, the template pointer last).

const FLAG: u32 = 0x01160EB8;
const METER_A: u32 = 0x1A3;
const METER_B: u32 = 0x105;
const THRESHOLD_BITS: u32 = 0x41400000; // 12.0
const OWNER_TEMPLATE_FLAG: u32 = 0x328C;
const TEMPLATE_WHEN_SET: u32 = 0x00E88848;
const TEMPLATE_WHEN_CLEAR: u32 = 0x00E88854;
const DIALOG_OWNER: u32 = 0x0116BFF0;
const REPORT_OWNER: u32 = 0x01033130;
const ENTRY_EAX_PIN: u32 = 0;

#[inline(always)]
fn fadd_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fge_pinned(a: f32, b: f32) -> bool {
    core::hint::black_box(a) >= core::hint::black_box(b)
}

lf_checker_rt::export!(cdecl, rw_00943100() -> u32 {
    unsafe {
        if lf_checker_rt::global::<u32>(FLAG).read() != 1 {
            return ENTRY_EAX_PIN;
        }
        let a_bits: u32 = lf_checker_rt::callee_cdecl!(1, u32, METER_A);
        let b_bits: u32 = lf_checker_rt::callee_cdecl!(5, u32, METER_B);
        let sum = fadd_pinned(f32::from_bits(a_bits), f32::from_bits(b_bits));
        let threshold = f32::from_bits(THRESHOLD_BITS);
        // Original: comiss threshold, sum / jb exit -> dialog iff ordered
        // threshold >= sum; NaN exits.
        if !fge_pinned(threshold, sum) {
            return b_bits;
        }
        let owner: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0u32);
        let flag = ((owner + OWNER_TEMPLATE_FLAG) as *const u8).read();
        let template = if flag == 0 {
            lf_checker_rt::relocated(TEMPLATE_WHEN_CLEAR)
        } else {
            lf_checker_rt::relocated(TEMPLATE_WHEN_SET)
        };
        let shown: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, lf_checker_rt::relocated(DIALOG_OWNER), template,
            1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 1u32, 0u32, 1u32, 1u32, 0u32,
            1u32, 0xFFFFFFFFu32
        );
        lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(REPORT_OWNER), shown)
    }
});
