// original: 0x00d51160 CRenderPhaseReflection::vf8
//
// Outdoor-reflection render phase setup: the longer sibling of the
// interior phase. After a preamble call it raises one global activity flag,
// configures the shared render target, and emits an extended packet
// sequence: standard cookie-tagged packets of several kinds, one gated
// builder packet (always built here, with constant selectors), packets
// finished by the two-word maker call, one initialized packet, and a small
// trailing packet. Ends in a tail call to the phase epilogue with the
// phase object in ECX. Returns the epilogue's answer, or the entry EAX
// (0 under the contract) when the selector word disables the phase.

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

const C_PRE: u32 = 8; // 0xd52420 thiscall/0: phase preamble
const C_MK: u32 = 9; // 0x8dbf20 thiscall/2: two-word packet maker
const C_POST: u32 = 10; // 0xc0fa50 cdecl/1: selector post pass
const C_TAIL: u32 = 11; // 0xd523f0 thiscall/0: phase epilogue (tail)

const TAG_E: u32 = 0x00E84C94;
const KIND_C: u32 = 0x00C106F0;
const KIND_D: u32 = 0x00C107A0;
const KIND_E: u32 = 0x009671A0;

#[inline(always)]
unsafe fn wb_at(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}

/// Cookie-tag an extended packet: standard tagging with the extended tag
/// and kind, plus a trailing mode byte.
#[inline(always)]
unsafe fn tag_packet_e(p: u32, mode: u8) {
    unsafe {
        let residue = rd32(p.wrapping_add(4));
        wr32(p, relocated(TAG_TMP));
        let cookie = (residue ^ rg32(G_COOKIE)) & 0x3FFF;
        wr32(p.wrapping_add(4), rd32(p.wrapping_add(4)) ^ cookie);
        wg32(G_COOKIE, rg32(G_COOKIE).wrapping_add(1));
        wr32(p, relocated(TAG_E));
        wr32(p.wrapping_add(8), relocated(KIND_E));
        wb_at(p.wrapping_add(0xC), mode);
    }
}

export!(thiscall, rw_d51160(this_ptr: u32) -> u32 {
    unsafe {
        let selector = rd32(this_ptr.wrapping_add(0x938));
        if selector == 0xFFFFFFFF {
            return 0; // entry EAX, fixed to 0 by the contract
        }
        callee_thiscall!(C_PRE, u32, this_ptr);
        let scratch = this_ptr.wrapping_add(0xB0);
        wb8(G_ACTIVE0, 1);
        callee_thiscall!(C_SETUP, u32, relocated(G_RENDER), scratch, F90, F0, F01, F40);

        // Packet A.
        let mut ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            tag_packet(ans, KIND_A);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Intensity: gauge default when it reports zero, else 0.0
        // (same dummy-slot idiom as the interior phase).
        let gauge = callee_thiscall!(C_GAUGE, u32, scratch);
        let intensity: u32 = if gauge & 0xFF == 0 { rg32(G_INTENSITY) } else { 0 };

        // Builder packet (unguarded here; constant selectors).
        ans = callee_cdecl!(C_ALLOC, u32, 0x18, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_BUILD, u32, ans, 0, 0, 1, intensity, 1, 0);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

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

        // Packets C and D.
        ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            tag_packet(ans, KIND_C);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);
        ans = callee_cdecl!(C_ALLOC, u32, 0xC, 0);
        if ans != 0 {
            tag_packet(ans, KIND_D);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Extended packet, mode 0.
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            tag_packet_e(ans, 0);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Maker packets (8,1) and (7,0).
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_MK, u32, ans, 8, 1);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_MK, u32, ans, 7, 0);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Selector configuration, then maker packets (8,0) and (7,1).
        callee_cdecl!(C_CFG, u32, selector, 0x131D, 8);
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_MK, u32, ans, 8, 0);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            ans = callee_thiscall!(C_MK, u32, ans, 7, 1);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Extended packet, mode 1.
        ans = callee_cdecl!(C_ALLOC, u32, 0x10, 0);
        if ans != 0 {
            tag_packet_e(ans, 1);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        // Post pass, then the small trailing packet.
        callee_cdecl!(C_POST, u32, selector);
        ans = callee_cdecl!(C_ALLOC, u32, 8, 0);
        if ans != 0 {
            tag_small_packet(ans);
        }
        ans = callee_cdecl!(C_EMIT, u32, ans);

        wb8(G_ACTIVE0, 0);
        // Tail call to the epilogue (the original jumps; the checker gives
        // the rewrite the normal stub and the original the tail variant).
        callee_thiscall!(C_TAIL, u32, this_ptr)
    }
});
