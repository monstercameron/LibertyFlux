// original: 0x00addc00 forward_post_render_viewport
use lf_checker_rt::{callee_thiscall, export};

/// Forward to the shared viewport post-render step (original 0x00ADDC00).
///
/// Vtable slot 8 of the post-render viewport phase: loads the embedded
/// renderer object at byte offset 0x940 and tail-jumps to the shared
/// implementation with it, returning that call's result unchanged.
export!(thiscall, rw_00addc00(this: u32) -> u32 {
    const RENDERER_OFF: u32 = 0x940;
    let target = unsafe { (this.wrapping_add(RENDERER_OFF) as *const u32).read() };
    callee_thiscall!(1, u32, target)
});
