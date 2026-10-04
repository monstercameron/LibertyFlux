// original: 0x00d8e1e0 audio_door_range_update
/// Age the door timer and re-arm it when the listener is out of range.
///
/// Ticks the countdown at `this + 8` down by the frame step while it stays
/// positive. When the shared source is present, its virtual probe is asked
/// for the listener offset; if the offset's squared length exceeds the
/// squared radius the timer is re-armed to the far value. A still-positive
/// timer finally sets bit 1 of the status word. Returns the probe answer
/// on the full path, else zero.
lf_rs89_rt::export!(thiscall, rw_00d8e1e0(this: *mut u8) -> u32 {
    unsafe {
        let timer_at = this.wrapping_add(8) as *mut f32;
        let t0 = *timer_at;
        if t0 > 0.0 {
            *timer_at = t0 - *lf_rs89_rt::global::<f32>(0x11735BC);
        }
        let present: u32 = lf_rs89_rt::callee_cdecl!(1, u32, 0);
        let out = if present == 0 {
            0
        } else {
            let src: u32 = lf_rs89_rt::callee_cdecl!(1, u32, 0);
            let radius = *lf_rs89_rt::global::<f32>(0x1056EEC);
            let limit = radius * radius;
            let vtab = *(src as *const u32);
            let target = *(vtab.wrapping_add(0xEC) as *const u32);
            let probe: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let mut scratch: u32 = 0;
            let hit = probe(src, &mut scratch as *mut u32 as u32);
            let ox = *(hit as *const f32);
            let oy = *(hit.wrapping_add(4) as *const f32);
            let oz = *(hit.wrapping_add(8) as *const f32);
            if ox * ox + oy * oy + oz * oz > limit {
                *timer_at = *lf_rs89_rt::global::<f32>(0x1056EF0);
            }
            hit
        };
        if *(this.wrapping_add(8) as *const f32) > 0.0 {
            let status = this.wrapping_add(4) as *mut u32;
            *status |= 2;
        }
        out
    }
});
