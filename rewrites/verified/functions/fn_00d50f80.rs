// original: 0x00d50f80 CRenderPhaseInteriorReflection::vf8
//
// Interior-reflection render phase setup: if the phase's selector word is
// set, raises two global activity flags, configures the shared render
// target through the setup call, then emits a fixed sequence of render
// command packets (each freshly allocated, cookie-tagged and handed to the
// command sink). A gauge call decides which intensity value the middle
// packet builder receives, and a global mode byte gates that builder.
// Returns the sink's answer for the last packet, or the entry EAX (0 under
// the contract) when the selector word disables the phase.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Callee ids (see contract).
const C_SETUP: u32 = 1; // 0x8f8250 thiscall/5: render-target setup
const C_ALLOC: u32 = 2; // 0x8dc3a0 cdecl/2: packet allocator
const C_EMIT: u32 = 3; // 0x499e30 cdecl/1: command sink
const C_GAUGE: u32 = 4; // 0x431460 thiscall/0: intensity gauge (AL channel)
const C_BUILD: u32 = 5; // 0x8daa70 thiscall/6: gated packet builder
const C_INIT: u32 = 6; // 0x8dc290 thiscall/1: packet initializer
const C_CFG: u32 = 7; // 0xb1dee0 cdecl/3: selector configuration

// Globals (file VAs).
const G_RENDER: u32 = 0x0118D7F0; // shared render target passed in ECX
const G_ACTIVE0: u32 = 0x01720FA9; // activity flag, set then cleared
const G_ACTIVE1: u32 = 0x01720FAA; // activity flag, set then cleared
const G_MODE: u32 = 0x01720FDC; // mode byte gating the builder
const G_FD8: u32 = 0x01720FD8; // dword handed to the initializer
const G_COOKIE: u32 = 0x010327A0; // packet cookie counter
const G_INTENSITY: u32 = 0x00FE88E8; // default intensity (float bits)

// Packet tags (relocated addresses, have HIGHLOW relocs).
const TAG_TMP: u32 = 0x00E7E048;
const TAG_STD: u32 = 0x00E7E080;
const TAG_SMALL: u32 = 0x00EA76C4;
const KIND_A: u32 = 0x00432BE0;
const KIND_B: u32 = 0x009CB6A0;

// Setup-call float arguments (raw bits, no relocs).
const F90: u32 = 0x42B40000;
const F0: u32 = 0x00000000;
const F01: u32 = 0x3DCCCCCD;
const F40: u32 = 0x42200000;

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rg32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read() }
}

#[inline(always)]
unsafe fn wg32(va: u32, v: u32) {
    unsafe { global::<u32>(va).write(v) }
}

#[inline(always)]
unsafe fn rb8(va: u32) -> u8 {
    unsafe { global::<u8>(va).read() }
}

#[inline(always)]
unsafe fn wb8(va: u32, v: u8) {
    unsafe { global::<u8>(va).write(v) }
}

/// Cookie-tag one standard packet: stores the temporary tag, folds the
/// global counter into the allocator residue word, bumps the counter, then
/// stores the final tag and kind. Mirrors the original's exact store order.
#[inline(always)]
unsafe fn tag_packet(p: u32, kind: u32) {
    unsafe {
        let residue = rd32(p.wrapping_add(4));
        wr32(p, relocated(TAG_TMP));
        let cookie = (residue ^ rg32(G_COOKIE)) & 0x3FFF;
        wr32(p.wrapping_add(4), rd32(p.wrapping_add(4)) ^ cookie);
        wg32(G_COOKIE, rg32(G_COOKIE).wrapping_add(1));
        wr32(p, relocated(TAG_STD));
        wr32(p.wrapping_add(8), relocated(kind));
    }
}

/// Cookie-tag the small trailing packet: same as above but with the small
/// tag and no kind word (the 8-byte packet has no room for one).
#[inline(always)]
unsafe fn tag_small_packet(p: u32) {
    unsafe {
        let residue = rd32(p.wrapping_add(4));
        wr32(p, relocated(TAG_TMP));
        let cookie = (residue ^ rg32(G_COOKIE)) & 0x3FFF;
        wr32(p.wrapping_add(4), rd32(p.wrapping_add(4)) ^ cookie);
        wg32(G_COOKIE, rg32(G_COOKIE).wrapping_add(1));
        wr32(p, relocated(TAG_SMALL));
    }
}

export!(thiscall, rw_d50f80(this_ptr: u32) -> u32 {
    unsafe {
        let selector = rd32(this_ptr.wrapping_add(0x938));
        if selector == 0xFFFFFFFF {
            return 0; // entry EAX, fixed to 0 by the contract
        }
        let scratch = this_ptr.wrapping_add(0xB0);
        wb8(G_ACTIVE0, 1);
        wb8(G_ACTIVE1, 1);
        callee_thiscall!(C_SETUP, u32, relocated(G_RENDER), scratch, F90, F0, F01, F40);

        // Packet A.
        let mut ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            tag_packet(ans, KIND_A);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Intensity: the gauge default when it reports zero, else 0.0.
        // (The original keeps this float in its dummy push slot, which it
        // zeroes just before the setup call; the slot is below incoming ESP
        // and invisible to the comparison, so a local carries the value.)
        let gauge = callee_thiscall!(C_GAUGE, u32, scratch);
        let intensity: u32 = if gauge & 0xFF == 0 { rg32(G_INTENSITY) } else { 0 };

        // Gated builder packet.
        if rb8(G_MODE) != 0 {
            let p = callee_cdecl!(C_ALLOC, u32, 0x18, 0);
            if p != 0 {
                let mode = rb8(G_MODE) as u32;
                ans = callee_thiscall!(C_BUILD, u32, p, 0, 0, mode, intensity, mode, 0);
            } else {
                ans = 0;
            }
            ans = callee_cdecl!(C_EMIT, u32, ans);
        }

        // Packet B.
        ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            tag_packet(ans, KIND_B);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Initialized packet.
        ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_INIT, u32, ans, rg32(G_FD8));
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Selector configuration, then the small trailing packet.
        callee_cdecl!(C_CFG, u32, selector, 0x131D, 8);
        ans = callee_cdecl!(C_ALLOC, u32, 8, 0);
        if ans != 0 {
            tag_small_packet(ans);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        wb8(G_ACTIVE0, 0);
        wb8(G_ACTIVE1, 0);
        ans
    }
});
