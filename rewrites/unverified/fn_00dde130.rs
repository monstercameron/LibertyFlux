// original: 0x00DDE130 scale_output_pair_by_metric
/// Fetch the inner view's current extent pair into `out`, then divide each
/// component by (live metric x configured scale). Returns `out`.
///
/// The pair comes from the inner object (+0x1E8) through its table slot
/// +0x210. Each divisor combines the object's own metric (table slot +0xB8,
/// single precision) with one of two configured integer scales selected by
/// a probe call's low byte. All arithmetic is single precision.
lf_checker_rt::export!(thiscall, rw_dde130(this: u32, out: u32) -> u32 {
    unsafe {
        let inner = *((this + 0x1E8) as *const u32);
        let inner_vtable = *(inner as *const u32);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((inner_vtable + 0x210) as *const u32) as usize);
        let pair = fetch(inner);
        let slots = out as *mut u32;
        *slots = *(pair as *const u32);
        *slots.add(1) = *((pair + 4) as *const u32);
        for i in 0..2 {
            let probe: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            let scale = if probe & 0xFF != 0 {
                *lf_checker_rt::global::<i32>(0x0105C888)
            } else {
                *lf_checker_rt::global::<i32>(0x0105C884)
            };
            let own_vtable = *(this as *const u32);
            let metric: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(*((own_vtable + 0xB8) as *const u32) as usize);
            let divisor = metric(this) * (scale as f32);
            let slot = slots.add(i) as *mut f32;
            *slot = *slot / divisor;
        }
        out
    }
});
