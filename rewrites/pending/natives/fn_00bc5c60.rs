// original: 0x00bc5c60 GET_CAR_SPEED_VECTOR
/// Script native `GET_CAR_SPEED_VECTOR` (hash 0x112E7FB1).
///
/// Vector-returning handler. It snapshots the three words behind the second
/// script argument (a speed vector: one integer and two float bit-patterns)
/// into the call context's return area at the slot selected by the context's
/// return index, bumps that index, then forwards the vehicle handle, a
/// context-interior pointer and a coerced boolean (`arg2 != 0`) to the
/// engine.
///
/// Quirk (observed): the boolean travels in the low byte of the handler's
/// own incoming stack slot, so the pushed word's high bytes repeat the
/// context pointer; reproduced here for bit-exact outgoing-call matching.
export!(cdecl, rw_00bc5c60(ctx: *const u8) -> u32 {
    unsafe {
        let ctxu = ctx as u32;
        let ctxw = ctx as *mut u32;
        let args = (*ctxw.add(2) as *const u32) as *const u32;
        let veh = *args;
        let vec = *args.add(1) as *const u32;
        let idx = *ctxw.add(3);
        let w0 = *vec;
        let w1 = *vec.add(1);
        let w2 = *vec.add(2);
        *ctxw.add(4 + idx as usize) = vec as u32;
        let out = (ctxu.wrapping_add((idx + 2) * 16)) as *mut u32;
        *out = w0;
        *out.add(1) = w1;
        *out.add(2) = w2;
        *ctxw.add(3) = idx + 1;
        let flag = u32::from(*args.add(2) != 0);
        let quirked = (ctxu & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, veh, out as u32, quirked)
    }
});
