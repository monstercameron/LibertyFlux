// original: 0x00acd970 audio_submit_voice_request
/// Voice request submission: resolve the source's levels, build the emit
/// parameter block, classify the returned id against the audible set, and —
/// unless already handed off — copy the emitted vector and hand the request
/// to the game mixer object.
///
/// `this` is the voice, `source` the sound source. Returns nothing; the
/// observable effects are the outgoing calls and the flag update.
export!(thiscall, rw_acd970(this_voice: u32, source: u32) -> u32 {
    /// Audibility gate: skip unless the source level exceeds this (0.9).
    const GATE_LIMIT: f32 = 0.9;
    /// Bias added to the resolved base level before headroom subtraction.
    const LEVEL_BIAS: f32 = 0.25;
    /// Flag bit recorded on the voice once the request is handed off.
    const HANDED_OFF: u32 = 0x4000_0000;
    /// File VA of the game-owned object receiving the finished request.
    const MIXER_OBJECT: u32 = 0x172bd20;
    /// File VA of the shared 3-float vector copied into the request block.
    const SHARED_VEC: u32 = 0x1b4b320;
    /// Request block consumed by the resolve/emit callees (their ABI):
    /// resolved levels, derived head triple, vector copies and the small
    /// tail fields the classifier result is read from.
    #[repr(C)]
    struct Request {
        out: [f32; 3],
        _reserved0: u32,
        head: [f32; 3],
        _reserved1: u32,
        send: [f32; 3],
        _reserved2: u32,
        zero: u32,
        _reserved3: [u32; 3],
        first: [f32; 3],
        _reserved4: u32,
        second: [f32; 3],
        _reserved5: u32,
        third: [f32; 3],
        _reserved6: u32,
        tail_words: [u32; 3],
        magic: u32,
        tail_flag: u8,
        _reserved7: u8,
        class_arg: u16,
    }
    unsafe {
        let inner = *((source + 0x20) as *const u32);
        let level = *((inner + 0x28) as *const f32);
        if GATE_LIMIT > level {
            return 0;
        }
        let mut req: Request = core::mem::zeroed();
        let base: f32 = callee_thiscall!(
            1,
            f32,
            this_voice,
            source,
            core::ptr::addr_of_mut!(req) as u32
        );
        req.head[0] = req.out[0];
        req.head[1] = req.out[1];
        req.head[2] = req.out[2] - (base + LEVEL_BIAS);
        let shared = global::<[f32; 3]>(SHARED_VEC);
        let v0 = (*shared)[0];
        let v1 = (*shared)[1];
        let v2 = (*shared)[2];
        req.first = [v0, v1, v2];
        req.second = [v0, v1, v2];
        req.third = [v0, v1, v2];
        req.magic = 0xffff;
        let key = *((this_voice + 0xd4) as *const u32);
        let ok: u8 = callee_cdecl!(
            2,
            u8,
            key,
            core::ptr::addr_of_mut!(req) as u32,
            core::ptr::addr_of_mut!(req.head) as u32,
            core::ptr::addr_of_mut!(req.zero) as u32,
            1,
            1,
            0
        );
        if ok == 0 {
            return 0;
        }
        let id: u32 = callee_cdecl!(3, u32, key, req.class_arg as u32);
        let audible = matches!(
            id,
            0x0
                | 0x1a1
                | 0x4b2
                | 0x4b3
                | 0x36a0
                | 0x36a1
                | 0x4b4
                | 0x4b5
                | 0x37a0
                | 0x4c7
                | 0x4c0
                | 0x4c8
                | 0x4c1
                | 0x1a7
                | 0x1a2
        );
        if !audible {
            return 0;
        }
        let flags = (this_voice + 0x164) as *mut u32;
        if (*flags >> 30) & 1 != 0 {
            return 0;
        }
        req.send = req.first;
        callee_thiscall!(
            4,
            u32,
            relocated(MIXER_OBJECT),
            this_voice,
            source,
            core::ptr::addr_of_mut!(req.send) as u32
        );
        *flags |= HANDED_OFF;
        0
    }
});
