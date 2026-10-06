// original: 0x00D77930 construct_state_block (proposed)

/// Initialise a small state block: stamp, tag and copied fields.
///
/// `this` points to a 0x16-byte block. Writes the header stamp, mixes the
/// incoming word at `+4` with the global counter at `0x010327a0` (xor,
/// keep 14 bits, xor back, bump the counter), stores `tag` at `+8`, writes
/// the final stamp, then copies a word each from `first` and `second` to
/// `+0xc`/`+0x10` and a byte each from `b0src`/`b1src` to `+0x14`/`+0x15`.
/// Returns `this`. Thiscall: object in `ecx`, five stack words.
use lf_checker_rt::{export, global, relocated};

export!(thiscall, rw_00d77930(this: u32, tag: u32, first: u32, second: u32, b0src: u32, b1src: u32) -> u32 {
    unsafe {
        const STAMP_OPEN: u32 = 0x00e7e048;
        const STAMP_DONE: u32 = 0x00eec708;
        const COUNTER: u32 = 0x010327a0;
        const MIX_MASK: u32 = 0x3fff;
        let mix_src = ((this + 4) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(relocated(STAMP_OPEN));
        let counter = global::<u32>(COUNTER).read();
        let mix = (mix_src ^ counter) & MIX_MASK;
        ((this + 4) as *mut u32).write_unaligned(mix_src ^ mix);
        global::<u32>(COUNTER).write(counter.wrapping_add(1));
        ((this + 8) as *mut u32).write_unaligned(tag);
        (this as *mut u32).write_unaligned(relocated(STAMP_DONE));
        ((this + 0x0c) as *mut u32).write_unaligned((first as *const u32).read_unaligned());
        ((this + 0x10) as *mut u32).write_unaligned((second as *const u32).read_unaligned());
        ((this + 0x14) as *mut u8).write((b0src as *const u8).read());
        ((this + 0x15) as *mut u8).write((b1src as *const u8).read());
        this
    }
});
