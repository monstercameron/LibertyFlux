// original: 0x00692450 audio_mix_pass
/// Per-voice mix pass over one channel bank (original 0x692450).
///
/// Walks the voice table of the channel bank (`this`), gathers one word per
/// voice into scratch, lets the output object observe the gathered words
/// through a virtual call, then mixes every positive sample into its voice
/// accumulator: mode 1 and mode 0 voices go through the two helper routines,
/// mode 2 voices accumulate inline. Voices flagged unavailable are silenced.
///
/// Scratch: the original reserves three `n`-word buffers with alloca; the
/// rewrite uses fixed-size stack arrays (`4 <= n <= 16` by contract).
///
/// Return quirk: the value left in EAX is the distance between the first two
/// alloca buffers (16-rounded `n * 4`) whenever the mix loop ran, else the
/// voice count. `n < 4` is out of contract: below 4 words the snapshot reads
/// past the copied words into stale stack, and alloca(0) additionally leaves
/// a scratch address in EAX; neither is reproducible from the six arguments.
export!(thiscall, rb48_fn2(this_ptr: u32, n: u32, arg1: u32, arg2: u32, _u1: u32, _u2: u32, arg3: u32) -> u32 {
    unsafe {
        let count = n as usize;
        // Three alloca(n*4) scratch buffers; the third is zeroed and never read.
        let mut gathered = [0u32; 16];
        let mut voices = [0u32; 16];
        let _spare = [0u32; 16];
        let vt = *(this_ptr as *const u32);
        let bank = *((vt.wrapping_add(0x0c)) as *const u32);
        let m = *((vt.wrapping_add(0x10)) as *const u16) as u32;
        if m == 0 {
            return m;
        }
        let mut i = 0u32;
        while i < m {
            let obj = *(((bank.wrapping_add(i.wrapping_mul(4))) as *const u32));
            // memcpy(gathered, arg3, n*4), fresh every slot (it also undoes
            // the previous slot's silencing).
            let src = core::slice::from_raw_parts(arg3 as *const u32, count);
            gathered[..count].copy_from_slice(src);
            // Gather: voices[k] is the leaf reached from key k for slot i;
            // when the leaf is flagged unavailable both words are silenced.
            let mut k = 0usize;
            while k < count {
                let key = *(((arg1.wrapping_add((k as u32).wrapping_mul(4))) as *const u32));
                let arr = *(((key.wrapping_add(0x0c)) as *const u32));
                let leaf = *(((arr.wrapping_add(i.wrapping_mul(4))) as *const u32));
                voices[k] = leaf;
                if *((leaf.wrapping_add(4)) as *const u8) & 0x10 != 0 {
                    voices[k] = 0;
                    gathered[k] = 0;
                }
                k += 1;
            }
            if arg2 != 0 {
                let b5 = *((obj.wrapping_add(5)) as *const u8) as u32;
                let w6 = *((obj.wrapping_add(6)) as *const u16) as u32;
                let vtable = *(arg2 as *const u32);
                let target = *(((vtable.wrapping_add(0x10))) as *const u32);
                let observe: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                // The answer is overwritten by the mix loop below, like the original.
                let _ = observe(arg2, b5, w6, n, gathered.as_ptr() as u32);
            }
            // Mix: every positive sample feeds its voice accumulator.
            let mut k = 0usize;
            while k < count {
                let sample = f32::from_bits(gathered[k]);
                if sample > 0.0 {
                    let entry = voices[k];
                    let dl = *((obj.wrapping_add(4)) as *const u8);
                    match dl & 0x0f {
                        1 => {
                            callee_thiscall!(2, u32, obj, entry);
                        }
                        0 => {
                            callee_thiscall!(3, u32, obj, entry);
                        }
                        2 if dl & 0x10 == 0 => {
                            let add = *((entry.wrapping_add(0x10)) as *const f32);
                            let cur = *((obj.wrapping_add(0x10)) as *const f32);
                            *((obj.wrapping_add(0x10)) as *mut f32) = cur + add * sample;
                        }
                        _ => {}
                    }
                }
                k += 1;
            }
            i += 1;
        }
        // EAX quirk: distance between the first two alloca buffers, which is
        // the 16-rounded second size (n >= 4 by contract; smaller n would
        // leave scratch-derived values here instead).
        n.wrapping_mul(4).wrapping_add(15) & !15
    }
});
