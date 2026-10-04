// original: 0x00B5D3B0 unnamed
/// Fire one gunshot event chain (`this`): unless the slot holds type 0x2E
/// (return), resolve the weapon info (callee 1), build a gunshot event
/// (callee 2), run the audio attach pair twice (callees 3-4), optionally
/// probe the target (callee 5) and raise a dynamic sound event (callees
/// 6-7), build the whizzed-by event (callee 8), set heard-shot flags behind
/// the mode gate (callee 11), and tear the events down (callees 9-10, 12).
/// No result; the second stack word is unused.
///
/// One call argument's upper three bytes come from a frame slot the original
/// never stores; they are reproduced as an uninitialized read, which the
/// checker defines through `stack_fill`.
///
/// Cond: `this` points to the slot object, `a0` (or null) to the target.
export!(thiscall, rw_b73_f5(this: u32, a0: u32, a1: u32, _a2: u32, a3: u32) -> u32 {
    unsafe {
        let edi18 = *(this.wrapping_add(0x18) as *const u32);
        if edi18 == 0x2E {
            return 0;
        }
        let w0: u32 = callee_cdecl!(1, u32, edi18);
        let flag12 = u32::from(
            *(w0.wrapping_add(8) as *const u32) == 3
                && *(w0.wrapping_add(0xC) as *const u32) != 0,
        );
        let w1: u32 = callee_cdecl!(1, u32, edi18);
        let v = (*(w1.wrapping_add(0x20) as *const u32) >> 8) & 0xFFFFFF01;
        let g = *(global::<i32>(0x11D6FD4));
        let flag13 = u32::from(!(g < 2) && edi18 == 0x24);
        // Argument word: the low byte is v's low byte, the upper three bytes
        // are a frame slot the original never stores (its flag bytes sit at
        // lower addresses, outside this word). Reproduced as an
        // uninitialized read, which the checker defines through stack_fill.
        let slot = core::mem::MaybeUninit::<u32>::uninit();
        let arg3val =
            (core::ptr::read_volatile(slot.as_ptr()) & !0xFF) | (v & 0xFF);
        // (The word passed as id8's second argument is the spilled eax2, not
        // uninitialized: the original saves it to its frame before id2.)
        // Frame event objects (addresses observed only, never dereferenced
        // by either side: compared out via call_regs/call_skip).
        let mut o_gun = [0u32; 64];
        let mut o_dyn = [0u32; 64];
        let mut o_whiz = [0u32; 64];
        let eax2 = a1.wrapping_add(0x30);
        let o_gun_p = o_gun.as_mut_ptr() as u32;
        let o_dyn_p = o_dyn.as_mut_ptr() as u32;
        let o_whiz_p = o_whiz.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(2, u32, o_gun_p, a0, eax2, a3, arg3val, edi18);
        // The attach calls take the object itself, the dynamic event takes
        // object+0x10 as its trailing word: same as the original's frame.
        let r3: u32 = callee_stdcall!(3, u32, o_gun_p, 0, 1);
        let _: u32 = callee_thiscall!(4, u32, r3);
        if a0 != 0 && flag13 == 0 {
            let e20 = *(a0.wrapping_add(0x20) as *const u32);
            let earg = if e20 == 0 { a0.wrapping_add(0x10) } else { e20.wrapping_add(0x30) };
            let _: u32 = callee_cdecl!(5, u32, 0x1B, earg, a0, 0, edi18, 0);
        }
        if (v & 0xFF) == 0 && a0 != 0 {
            let _: u32 = callee_thiscall!(
                6,
                u32,
                o_dyn_p,
                a0,
                0x42F00000,
                0xFFFFFFFF,
                o_dyn_p.wrapping_add(0x10)
            );
            let r3b: u32 = callee_stdcall!(3, u32, o_dyn_p, 0, 1);
            let _: u32 = callee_thiscall!(4, u32, r3b);
            let _: u32 = callee_thiscall!(7, u32, o_dyn_p);
        }
        let _: u32 = callee_thiscall!(8, u32, o_whiz_p, a0, eax2, a3, arg3val, edi18);
        let r3c: u32 = callee_stdcall!(3, u32, o_whiz_p, 0, 1);
        let _: u32 = callee_thiscall!(4, u32, r3c);
        if a0 != 0 && (*(a0.wrapping_add(0x28) as *const u32) & 0x3C0) == 0xC0 {
            let p = a0.wrapping_add(0x29C) as *mut u32;
            *p |= 0x8000000;
        } else if *global::<u32>(0x11D6FD4) == 2 {
            let a11: u32 = callee_cdecl!(11, u32,);
            if (a11 & 0xFF) != 0
                && a0 != 0
                && (*(a0.wrapping_add(0x28) as *const u32) & 0x3C0) == 0x80
            {
                let e = *(a0.wrapping_add(0xF50) as *const u32);
                if e != 0 {
                    let p = e.wrapping_add(0x29C) as *mut u32;
                    *p |= 0x8000000;
                }
            }
        }
        if flag12 == 0 {
            let _: u32 = callee_thiscall!(12, u32, relocated(0x16DCC30), 0x18, a0);
        }
        let _: u32 = callee_thiscall!(9, u32, o_whiz_p);
        let _: u32 = callee_thiscall!(10, u32, o_gun_p);
        0
    }
});
