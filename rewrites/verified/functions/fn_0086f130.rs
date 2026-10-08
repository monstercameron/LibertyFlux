/// Proof scope: one scripted callback and one call shape.
/// The writable-data snapshot is disabled; callback internals are excluded.
// original: 0x0086F130
use lf_checker_rt::{export, global};

type PrintInteger = extern "cdecl" fn(u32, u32) -> u32;

#[inline(never)]
unsafe fn forward_integer(context: u32, perturb: bool) -> u32 {
    let arguments = *(context.wrapping_add(8) as *const u32) as *const u32;
    let integer = *arguments;
    let forwarded = if perturb { integer ^ 1 } else { integer };
    let callback_address = *global::<u32>(0x0110_B718);
    let callback: PrintInteger = core::mem::transmute(callback_address);
    callback(0, forwarded)
}

export!(cdecl, rw_0086f130(context: u32) -> u32 {
    unsafe { forward_integer(context, false) }
});
