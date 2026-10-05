// original: 0x008AD480 audio_param_dispatch (proposed)

/// Dispatch a parameter handle through the 14-way parameter table.
///
/// A null `arg`, or a zero answer from the gate (`0x8aad90`, thiscall on the
/// shared table at `0x115da04` with `(arg, this)`), clears `this` and returns
/// 0. Otherwise the descriptor at `[this]` is unpacked: `[this+0x1c]` and
/// `[this+0x20]` from descriptor words `+0xd`/`+0x11`, `[this+0x25]` set when
/// descriptor byte `+5`'s low two bits equal 1, `[this+0x24]` the kind byte,
/// `[this+0x26]` cleared. The kind dispatches through a jump table (kinds
/// 1..14; kind 0 and anything above 14 return 0 with `this` untouched):
/// kinds 1 and 13 set `[this+0x26]` and return 1 (kind 1 also copies
/// descriptor word `+0x15` to `[this+4]`, kind 13 leaves it alone); kinds 6
/// and 14 clear `[this+0x26]` and return 1; the rest tail-call a kind handler
/// (thiscall on `this`, no arguments) and return its value: kinds 2,3 share
/// one, kinds 8,9,11,12 share another, kinds 4,5,7,10 each have their own.
/// The compared return channel is `al`; where the original's byte-wide
/// return leaves earlier upper bytes in `eax` (gate answer, copied word,
/// `kind - 1`), the rewrite reproduces them, except on the null-argument
/// path where the upper bytes are unknowable entry garbage. Original is
/// thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008AD480(this: u32, arg: u32) -> u32 {
    const GATE: u32 = 1;
    const H660: u32 = 2;
    const H9A0: u32 = 3;
    const H390: u32 = 4;
    const H440: u32 = 5;
    const H770: u32 = 6;
    const HAC0: u32 = 7;
    const TABLE: u32 = 0x0115_da04;
    unsafe {
        if arg == 0 {
            (this as *mut u32).write_unaligned(0);
            return 0;
        }
        let g: u32 = lf_checker_rt::callee_thiscall!(
            GATE, u32, lf_checker_rt::relocated(TABLE), arg, this);
        if g as u8 == 0 {
            (this as *mut u32).write_unaligned(0);
            return g & 0xffff_ff00;
        }
        let d = (this as *const u32).read_unaligned();
        ((this + 0x26) as *mut u8).write(0);
        ((this + 0x1c) as *mut u32)
            .write_unaligned(((d + 0xd) as *const u32).read_unaligned());
        ((this + 0x20) as *mut u32)
            .write_unaligned(((d + 0x11) as *const u32).read_unaligned());
        let b5 = ((d + 5) as *const u32).read_unaligned() as u8 & 3;
        ((this + 0x25) as *mut u8).write((b5 == 1) as u8);
        let kind = (d as *const u8).read();
        ((this + 0x24) as *mut u8).write(kind);
        match kind {
            1 => {
                let w = ((d + 0x15) as *const u32).read_unaligned();
                ((this + 4) as *mut u32).write_unaligned(w);
                ((this + 0x26) as *mut u8).write(1);
                (w & 0xffff_ff00) | 1
            }
            2 | 3 => lf_checker_rt::callee_thiscall!(H660, u32, this),
            4 => lf_checker_rt::callee_thiscall!(H9A0, u32, this),
            5 => lf_checker_rt::callee_thiscall!(H390, u32, this),
            6 | 14 => {
                ((this + 0x26) as *mut u8).write(0);
                1
            }
            7 => lf_checker_rt::callee_thiscall!(H440, u32, this),
            8 | 9 | 11 | 12 => lf_checker_rt::callee_thiscall!(H770, u32, this),
            10 => lf_checker_rt::callee_thiscall!(HAC0, u32, this),
            13 => {
                ((this + 0x26) as *mut u8).write(1);
                1
            }
            _ => (u32::from(kind).wrapping_sub(1)) & 0xffff_ff00,
        }
    }
});
