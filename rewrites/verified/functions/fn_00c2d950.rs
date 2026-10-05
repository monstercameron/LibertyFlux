// original: 0x00c2d950 audfire_spatial_update (proposed name)
// Gated positional mix update for one fire-audio voice.
//
// The voice at `arg0` is processed only when the channel byte is positive, the
// emitter flag bit 21 is set, the handle lookup succeeds, and the voice index
// falls inside the live row table with its flag bits clear. A nonzero `arg1`
// additionally runs an auxiliary notification call. The mix itself dots the
// voice direction against a sampled triple, scales and biases two blend
// vectors through the handle's parameter block and an integer intensity code,
// and pushes both vectors to the row worker; when the raw dot exceeds 13.0 a
// final commit call runs. Returns nothing meaningful.
export!(thiscall, rw_00c2d950(this_ptr: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let channel = load_i8(this_ptr + 5);
        if channel <= 0 {
            return 0;
        }
        let vt = lu(arg0);
        let slot_get: u32 = lu(vt + 0xa0);
        let get: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_get as usize);
        let h1 = get(arg0, channel as i32 as u32);
        let h2 = callee_thiscall!(2, u32, h1);
        if arg1 & 0xff != 0 {
            let aux = lu(arg0 + 0x6c);
            if aux != 0 && load_u8(aux + 0x0e) != 0 {
                // Last-pushed-first: esi was pushed first, so arg0 is the voice.
                callee_thiscall!(3, u32, relocated(0x018ecfb0), arg0, this_ptr);
            }
        }
        if (lu(this_ptr + 0x10) >> 21) & 1 == 0 {
            return 0;
        }
        if h2 == 0 {
            return 0;
        }
        // The original's `(an instruction of the original)` after the movzx below is dead
        // (a zero-extended word is never negative) and is not reproduced.
        let cx = load_u16(h2 + 8) as u32;
        let tab = lu(lu(relocated(0x012b9c7c)) + 4);
        let count = load_u16(tab + 0x26) as u32;
        if cx >= count {
            return 0;
        }
        let flags = lu(tab + 0x70);
        if load_u8(flags + cx * 8 + 4) & 3 != 0 {
            return 0;
        }
        let rows = lu(tab + 0x80);
        let row = lu(rows + cx * 4);
        if row == 0 {
            return 0;
        }
        let vec = lu(arg0 + 0x20);
        let slot_sample: u32 = lu(vt + 0xec);
        let sample: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_sample as usize);
        // The original passes a pointer to uninitialized stack scratch here;
        // the callee answer is a fresh triple and the slot is never read back,
        // so the contract skips this argument and any local pointer will do.
        let mut scratch = [0u32; 4];
        let ans = sample(arg0, scratch.as_mut_ptr() as u32);
        let v10 = load_f(vec + 0x10);
        let v14 = load_f(vec + 0x14);
        let v18 = load_f(vec + 0x18);
        let a0 = load_f(ans);
        let a1 = load_f(ans + 4);
        let a2 = load_f(ans + 8);
        let mut x1 = fmul_first(v14, a1);
        let mut x0 = fmul_first(v10, a0);
        x1 = fadd_first(x1, x0);
        x0 = fmul_first(v18, a2);
        x1 = fadd_first(x1, x0);
        let dot = x1;
        let narrow = load_f(relocated(0x00ec6f98));
        let wide = load_f(relocated(0x00fe87e8));
        x0 = dot * narrow;
        let x4 = fmul_first(x0, v10);
        let x5 = fmul_first(v14, x0);
        let x6 = fmul_first(v18, x0);
        x0 = dot * wide;
        let v20 = load_f(vec + 0x20);
        let v24 = load_f(vec + 0x24);
        let v28 = load_f(vec + 0x28);
        let mut y1 = fmul_first(v20, x0);
        let mut y2 = fmul_first(v24, x0);
        let mut y3 = fmul_first(v28, x0);
        y1 = fadd_first(y1, x4);
        y2 = fadd_first(y2, x5);
        y3 = fadd_first(y3, x6);
        let eobj = lu(h2 + 4);
        let eslot: u32 = lu(lu(eobj) + 0x24);
        // f32 return type reads the x87 ST0 float result without assembly.
        let eval: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(eslot as usize);
        let f = eval(eobj);
        let half = load_f(relocated(0x00fe8830));
        let mut z0 = fmul_first(y1, f);
        let s38 = load_f(h2 + 0x38);
        let mut z6 = load_f(h2 + 0x20) * half;
        let sp20 = z0;
        z0 = fmul_first(y2, f);
        z6 = fadd_first(z6, load_f(h2 + 0x40));
        let mut z4 = load_f(h2 + 0x24) * half;
        let sp24 = z0;
        z0 = fmul_first(y3, f);
        z4 = fadd_first(z4, load_f(h2 + 0x44));
        let mut z5 = load_f(h2 + 0x28) * half;
        let sp28 = z0;
        z5 = fadd_first(z5, load_f(h2 + 0x48));
        let mut w0 = load_f(h2 + 0x30) * half;
        let mut w1 = load_f(h2 + 0x34) * half;
        let mut w2 = s38 * half;
        w0 = fadd_first(w0, z6);
        w1 = fadd_first(w1, z4);
        w2 = fadd_first(w2, z5);
        let n = callee_thiscall!(6, u32, eobj) as i32;
        // Integer intensity code mapped through small consts; the chain stays
        // finite so plain operators match bit for bit.
        let mut x3 = n as f32;
        x3 = x3 * load_f(relocated(0x00fe8684));
        x3 = x3 * load_f(relocated(0x00fe8a24));
        x3 = x3 - load_f(relocated(0x00fe88e8));
        x3 = x3 * load_f(relocated(0x00fe881c));
        let mut w0f = v10 * x3;
        let mut w1f = v14 * x3;
        w0f = fadd_first(w0f, w0);
        let mut w2f = v18 * x3;
        w1f = fadd_first(w1f, w1);
        w2f = fadd_first(w2f, w2);
        let v0 = [sp20.to_bits(), sp24.to_bits(), sp28.to_bits()];
        let v1 = [w0f.to_bits(), w1f.to_bits(), w2f.to_bits()];
        let gslot: u32 = lu(lu(row) + 0x8c);
        let push_vecs: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(gslot as usize);
        push_vecs(row, v0.as_ptr() as u32, v1.as_ptr() as u32, 0, 0x3f800000);
        if dot > load_f(relocated(0x00fe8b14)) {
            callee_thiscall!(8, u32, arg0.wrapping_add(0x210), h2.wrapping_add(0x40));
        }
        0
    }
});
