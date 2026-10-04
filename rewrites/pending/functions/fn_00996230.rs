// original: 0x00996230 audio_voice_update
/// Recompute a voice's spatial mix from its emitter positions.
///
/// `this` is the audio voice object (sources at +0x820..+0x844, flags at
/// +0xb31/+0xb32, mix cache at +0xa04); `arg` is the per-frame voice update
/// record (result distance at +0x50, channel id at +0x64).
///
/// Behaviour: unless every source slot is live (any zero slot takes the
/// early path that releases the voice through two calls and returns the
/// second call's result), refresh each source once, fetch the emitter
/// position through the emitter's slot-59 function, combine the scaled
/// offsets with two freshly queried reference points using single-precision
/// arithmetic, and store the resulting distance. Then run the fixed chain of
/// per-frame update calls and push the shared gain value to every source.
/// Returns the last call's result in both paths.
///
/// Merge note: the small helpers below are part of this rewrite (the merge
/// target can wrap this file in its own module). The four `f*` helpers must
/// stay `#[inline(never)]` opaque calls: inlining them lets LLVM's SLP
/// vectorizer re-pair the reduction order, which keeps values but changes
/// NaN payloads (caught by the checker at 1 fail in 1000 trials).
#[inline(always)]
fn r32(a: u32) -> u32 {
    unsafe { *(a as *const u32) }
}

#[inline(always)]
fn w32(a: u32, v: u32) {
    unsafe { *(a as *mut u32) = v };
}

#[inline(always)]
fn r8(a: u32) -> u8 {
    unsafe { *(a as *const u8) }
}

#[inline(always)]
fn w8(a: u32, v: u8) {
    unsafe { *(a as *mut u8) = v };
}

#[inline(always)]
fn rf(a: u32) -> f32 {
    f32::from_bits(r32(a))
}

#[inline(always)]
fn wf(a: u32, v: f32) {
    w32(a, v.to_bits())
}

#[inline(never)]
fn fadd(a: f32, b: f32) -> f32 {
    a + b
}

#[inline(never)]
fn fsub(a: f32, b: f32) -> f32 {
    a - b
}

#[inline(never)]
fn fmul(a: f32, b: f32) -> f32 {
    a * b
}

#[inline(never)]
fn fsqrt(a: f32) -> f32 {
    a.sqrt()
}

export!(thiscall, rw_rb66_996230(this: u32, arg: u32) -> u32 {
    // Gate: every source slot must be live.
    if r32(this.wrapping_add(0x840)) == 0
        || r32(this.wrapping_add(0x844)) == 0
        || r32(this.wrapping_add(0x828)) == 0
        || r32(this.wrapping_add(0x82c)) == 0
        || r32(this.wrapping_add(0x830)) == 0
        || r32(this.wrapping_add(0x834)) == 0
    {
        callee_thiscall!(15, u32, this, 0u32);
        return callee_thiscall!(16, u32, this, 0u32);
    }
    // One-time source refresh.
    if r8(this.wrapping_add(0xb31)) != 0 && r8(this.wrapping_add(0xb32)) == 0 {
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x828)));
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x82c)));
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x830)));
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x834)));
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x840)));
        callee_thiscall!(1, u32, r32(this.wrapping_add(0x844)));
        w8(this.wrapping_add(0xb32), 1);
    }
    // Emitter position through the emitter object's slot-59 function.
    let obj = r32(this.wrapping_add(0x820));
    let vt = r32(obj);
    let slot59: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(r32(vt.wrapping_add(0xec)) as usize) };
    let mut scratch = 0u32;
    let pos = slot59(obj, &mut scratch as *mut u32 as u32);
    // Scaled offsets against the listener head box.
    let k = f32::from_bits(r32(relocated(0x00fe8748)));
    let ox = fmul(rf(pos), k);
    let oy = fmul(rf(pos.wrapping_add(4)), k);
    let oz = fmul(rf(pos.wrapping_add(8)), k);
    let head = r32(obj.wrapping_add(0x20));
    let sx = fsub(rf(head.wrapping_add(0x30)), ox);
    let sy = fsub(rf(head.wrapping_add(0x34)), oy);
    let sz = fsub(rf(head.wrapping_add(0x38)), oz);
    // Two reference points from the shared audio service.
    let svc = relocated(0x0115def0);
    let ra = callee_thiscall!(3, u32, svc, 0u32);
    let rb = callee_thiscall!(4, u32, svc, 0u32);
    // Squared distance of the head box from reference B, summed in the
    // original lane order.
    let dx = fsub(rf(head.wrapping_add(0x30)), rf(rb.wrapping_add(0x30)));
    let dy = fsub(rf(head.wrapping_add(0x34)), rf(rb.wrapping_add(0x34)));
    let dz = fsub(rf(head.wrapping_add(0x38)), rf(rb.wrapping_add(0x38)));
    let dist_b = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
    // Residuals of the scaled offsets against the reference span.
    let qx = fsub(sx, fsub(rf(rb.wrapping_add(0x30)), rf(ra)));
    let qy = fsub(sy, fsub(rf(rb.wrapping_add(0x34)), rf(ra.wrapping_add(4))));
    let qz = fsub(sz, fsub(rf(rb.wrapping_add(0x38)), rf(ra.wrapping_add(8))));
    let err = fadd(fadd(fmul(qy, qy), fmul(qx, qx)), fmul(qz, qz));
    // Distance correction through the service; stored into the record.
    let corr = fsub(fsqrt(dist_b), fsqrt(err));
    let f: f32 = callee_thiscall!(5, f32, svc, corr.to_bits(), 0x1eu32, 0x3f800000u32);
    wf(arg.wrapping_add(0x50), f);
    // Fixed per-frame update chain.
    callee_thiscall!(6, u32, this, arg);
    if r8(this.wrapping_add(0xb32)) != 0 {
        callee_thiscall!(7, u32, this, arg);
        callee_thiscall!(8, u32, this, arg);
    }
    callee_thiscall!(9, u32, this, arg);
    callee_thiscall!(10, u32, this, arg);
    callee_thiscall!(11, u32, this, arg);
    callee_thiscall!(12, u32, this, arg);
    callee_thiscall!(13, u32, this, arg);
    w32(this.wrapping_add(0xa04), r32(arg.wrapping_add(0x64)));
    // Shared gain value to every source; return the last result.
    let gain = r32(relocated(0x01038be4));
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x828)), gain);
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x82c)), gain);
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x830)), gain);
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x834)), gain);
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x840)), gain);
    callee_thiscall!(14, u32, r32(this.wrapping_add(0x844)), gain)
});
