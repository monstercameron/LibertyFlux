//! `lf-checker-worker`: the checker's 32-bit comparison worker.
//! Maps the original executable once, then runs many trials on stdin/stdout
//! as JSON lines. Driven by `scripts/checker/checker2.py`, whose module
//! documentation describes the protocol and the contract format.
//!
//! Usage (from the repository root):
//! `cargo build --release --target i686-pc-windows-msvc -p lf-checker-worker`
//! then run contracts with the driver; never run this binary by hand.
//! Only the i686 build ever runs: it executes original 32-bit code
//! natively. Commands: setup, trial, teardown, ping. One JSON object per
//! line on stdin, one per line on stdout; stderr is diagnostics (never JSON).
//!
//! v2 abilities: per-callee script slots, stub out-param writes, stub
//! addresses in the setup response (for driver-side vtable planting, which
//! is the indirect-call interception), E9 tail-patch sites, pointer
//! normalization with pointed-to snapshots in call keys, TLS fabrication,
//! XMM entry values, XMM0 call-arg logging with stack transport on the
//! rewrite side, defined stack fills, and below-ESP scratch reset.
//!
//! v5 abilities (each opt-in per contract, except the window guard): x87
//! entry values loaded on the original's FPU stack with a mirror for the
//! rewrite, and the x87 state check (stack top, tags, valid registers);
//! XMM2-XMM7 call-argument logging and rewrite-side transports;
//! snapshots of up to 64 words per callee at any byte offset from the
//! pointer; built-in self-test originals (`fn_selftest`) for the checker's
//! own regression; and the preferred-base window, reserved at start-up so
//! the original's unrelocated absolute accesses fault loudly instead of
//! reading worker memory, with an optional read-only shadow of the file's
//! read-only data there (`abs_shadow`). The target-independent logic lives
//! in the library (`src/lib.rs`), where its unit tests run on any host.

// The worker maps and executes original machine code, plants stubs in it,
// and catches faults in-process: raw pointers and FFI are its whole job.
#![allow(unsafe_code)]
// Integrated lane code, proven by the checker's regression suite: address
// casts, long decoders and shared match arms are inherent to the domain,
// so pedantic style lints stay off here. Correctness lints (clippy::all)
// still apply; narrow this to per-lint allows if the code is reworked.
#![allow(clippy::pedantic)]

use lf_checker_worker::{abswin, snap, vecregs, x87};
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(addr: *mut c_void, size: usize, atype: u32, prot: u32) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(proc: *mut c_void, base: *const c_void, len: usize) -> i32;
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn GetCurrentProcess() -> *mut c_void;
    fn AddVectoredExceptionHandler(
        first: u32,
        h: extern "system" fn(*mut c_void) -> i32,
    ) -> *mut c_void;
    fn LoadLibraryW(name: *const u16) -> *mut c_void;
    fn GetProcAddress(h: *mut c_void, name: *const i8) -> *mut c_void;
    fn FreeLibrary(h: *mut c_void) -> i32;
    fn VirtualFree(addr: *mut c_void, size: usize, ftype: u32) -> i32;
    fn ExitProcess(code: u32) -> !;
}

const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_RWX: u32 = 0x40;
const PAGE_NOACCESS: u32 = 0x01;
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const MEM_RELEASE: u32 = 0x8000;

// ---------------------------------------------------------------------------
// Minimal JSON value + parser (zero dependencies) and emitter helpers.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum J {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<J>),
    Obj(HashMap<String, J>),
}

impl J {
    fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::Obj(m) => m.get(k),
            _ => None,
        }
    }
    fn as_i64(&self) -> i64 {
        match self {
            J::Int(i) => *i,
            J::Bool(b) => *b as i64,
            J::Str(s) => parse_u32(s) as i64,
            _ => 0,
        }
    }
    fn as_u32(&self) -> u32 {
        self.as_i64() as u32
    }
    fn as_usize(&self) -> usize {
        self.as_i64() as usize
    }
    fn as_str(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => "",
        }
    }
    fn as_bool(&self, d: bool) -> bool {
        match self {
            J::Bool(b) => *b,
            J::Int(i) => *i != 0,
            _ => d,
        }
    }
    fn as_arr(&self) -> &[J] {
        match self {
            J::Arr(a) => a,
            _ => &[],
        }
    }
}

fn parse_u32(t: &str) -> u32 {
    let t = t.trim();
    if let Some(x) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u32::from_str_radix(x, 16).unwrap_or(0)
    } else if t.starts_with('-') {
        t.parse::<i32>().unwrap_or(0) as u32
    } else {
        t.parse::<u32>().unwrap_or(0)
    }
}

struct P<'a> {
    b: &'a [u8],
    p: usize,
}

fn parse_json(s: &str) -> Result<J, String> {
    let mut p = P {
        b: s.as_bytes(),
        p: 0,
    };
    let v = p.value()?;
    p.ws();
    if p.p != p.b.len() {
        return Err("trailing chars".to_string());
    }
    Ok(v)
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.p < self.b.len() && matches!(self.b[self.p], b' ' | b'\t' | b'\r' | b'\n') {
            self.p += 1;
        }
    }
    fn peek(&self) -> u8 {
        if self.p < self.b.len() {
            self.b[self.p]
        } else {
            0
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.peek() {
            b'{' => self.obj(),
            b'[' => self.arr(),
            b'"' => Ok(J::Str(self.string()?)),
            b't' => self.lit("true", J::Bool(true)),
            b'f' => self.lit("false", J::Bool(false)),
            b'n' => self.lit("null", J::Null),
            c if c == b'-' || c.is_ascii_digit() => self.num(),
            c => Err(format!("unexpected char {}", c)),
        }
    }
    fn lit(&mut self, s: &str, v: J) -> Result<J, String> {
        if self.b[self.p..].starts_with(s.as_bytes()) {
            self.p += s.len();
            Ok(v)
        } else {
            Err("bad literal".to_string())
        }
    }
    fn num(&mut self) -> Result<J, String> {
        let st = self.p;
        // ints, floats (floats are parsed here, not via the Int path)
        let mut is_float = false;
        while self.p < self.b.len()
            && (self.b[self.p].is_ascii_digit()
                || matches!(self.b[self.p], b'-' | b'+' | b'.' | b'e' | b'E'))
        {
            if matches!(self.b[self.p], b'.' | b'e' | b'E') {
                is_float = true;
            }
            self.p += 1;
        }
        let t = std::str::from_utf8(&self.b[st..self.p]).map_err(|_| "bad num")?;
        if is_float {
            // store floats scaled: keep as string in J::Str would lose type; use Int of bits? simplest: Int(f*1e12)
            let f: f64 = t.parse().map_err(|_| "bad float")?;
            Ok(J::Int((f * 1e12) as i64))
        } else {
            t.parse::<i64>()
                .map(J::Int)
                .map_err(|_| "bad int".to_string())
        }
    }
    fn string(&mut self) -> Result<String, String> {
        // assumes opening quote
        self.p += 1;
        let mut out = String::new();
        while self.p < self.b.len() {
            let c = self.b[self.p];
            self.p += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    if self.p >= self.b.len() {
                        break;
                    }
                    let e = self.b[self.p];
                    self.p += 1;
                    match e {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        b'u' => {
                            if self.p + 4 > self.b.len() {
                                return Err("bad \\u".to_string());
                            }
                            let h = std::str::from_utf8(&self.b[self.p..self.p + 4])
                                .map_err(|_| "bad \\u")?;
                            let cp = u32::from_str_radix(h, 16).map_err(|_| "bad \\u")?;
                            out.push(char::from_u32(cp).unwrap_or('?'));
                            self.p += 4;
                        }
                        _ => out.push(e as char),
                    }
                }
                _ => out.push(c as char),
            }
        }
        Err("unterminated string".to_string())
    }
    fn arr(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut v = Vec::new();
        loop {
            self.ws();
            if self.peek() == b']' {
                self.p += 1;
                return Ok(J::Arr(v));
            }
            v.push(self.value()?);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b']' => {
                    self.p += 1;
                    return Ok(J::Arr(v));
                }
                _ => return Err("bad array".to_string()),
            }
        }
    }
    fn obj(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut m = HashMap::new();
        loop {
            self.ws();
            if self.peek() == b'}' {
                self.p += 1;
                return Ok(J::Obj(m));
            }
            if self.peek() != b'"' {
                return Err("bad obj key".to_string());
            }
            let k = self.string()?;
            self.ws();
            if self.peek() != b':' {
                return Err("bad obj colon".to_string());
            }
            self.p += 1;
            let v = self.value()?;
            m.insert(k, v);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b'}' => {
                    self.p += 1;
                    return Ok(J::Obj(m));
                }
                _ => return Err("bad obj".to_string()),
            }
        }
    }
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

fn hx(v: u32) -> String {
    format!("\"0x{:x}\"", v)
}

fn hexbytes(b: &[u8]) -> String {
    const H: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        s.push(H[(x >> 4) as usize] as char);
        s.push(H[(x & 15) as usize] as char);
    }
    s
}

// ---------------------------------------------------------------------------
// Worker state (single trial thread at a time; main thread only joins).
// ---------------------------------------------------------------------------

const SSIZE: usize = 0x0010_0000;
const HSIZE: usize = 0x0010_0000;
const HEAP_USE: usize = 0x000F_0000; // top 64KB of heap region is a guard page
const STACK_SNAP_OFF: usize = 0xFC000;
const STACK_SNAP_LEN: usize = 0x1100;
const SCRATCH_FILL_OFF: usize = 0xF4000; // below-ESP scratch reset region
const SCRATCH_FILL_LEN: usize = 0x8000; // 32KB, ends at STACK_SNAP_OFF
const LOG_MAX: usize = 256; // default per-trial-side call-log cap
const LOG_HARD_MAX: usize = 1024; // setup rejects log_max above this
const LOG_ENTRY: usize = 272;
// Log entry v4 layout: 0:id 4:ecx 8:edx 12:ebx 16:esi 20:edi 24:nargs
// 28:args[40] 188:snap_n 192:snap[8] 224:xmm0[4] 240:xmm1[4] 256:eax
// (v2 logged args[8] only, so trailing call arguments passed uncompared;
// v3 logs and compares every argument up to LOG_MAXW. Contracts declaring
// more are rejected at setup rather than silently truncated. v4 appends the
// stub-entry eax for eax-argument callees (lane r-s111).)
// The log region always reserves LOG_HARD_MAX entries so a contract can
// raise the cap with top-level "log_max" (v4 item 6c); the stub's full
// check uses the setup-time value, byte-identical at the default 256.
const LOG_MAXW: usize = 40;
// v5: snapshot words per callee (setup refuses more). The first
// snap::SNAP_BASE_WORDS stay in the base entry (v4 layout); the rest go to
// the extension snapshot log, which shares the base log's stride.
const SNAP_MAXW: usize = snap::SNAP_CAP_WORDS;
const _: () = assert!(LOG_ENTRY == snap::LOG_ENTRY);
// v5 extension logs (scratch-stack offsets), parallel to the base log and
// at its stride, so a stub reaches them from the base entry pointer in eax
// with one constant displacement: snapshot words 8-63, and XMM2-XMM7.
const EXT_SNAP_OFF: usize = 0x66000;
const EXT_XMM_OFF: usize = 0xAA000;
const EXT_LOG_LEN: usize = LOG_HARD_MAX * LOG_ENTRY; // 0x44000 each
const _: () = assert!(EXT_SNAP_OFF + EXT_LOG_LEN <= EXT_XMM_OFF);
const _: () = assert!(EXT_XMM_OFF + EXT_LOG_LEN <= SCRATCH_FILL_OFF);
// v5 self-test originals: one page between the stub area and the log.
const SELFTEST_OFF: usize = 0x11000;
// v5 ctx layout past the XMM entry values: ctx[CTX_X87_N] is the number of
// x87 entry values the trampoline loads on this side (0 on the rewrite
// side), and ctx[CTX_X87_VALS + 3*i ..] holds ST(i) as [lo, hi, sexp]
// (an `fld tbyte` reads the low 10 bytes of each 12-byte slot).
const CTX_X87_N: usize = 177;
const CTX_X87_VALS: usize = 178;
const CTX_WORDS: usize = CTX_X87_VALS + 3 * x87::X87_MAX_ENTRIES;
// v5 x87 mirror (meta offset): count word, then ST(i) at 16 + 16*i.
const META_X87_MIRROR: usize = 1408;
const X87_MIRROR_LEN: usize = 16 + 16 * x87::X87_MAX_ENTRIES;
const WRITEW_PER_ID: usize = 16;
const SEQ_MAX: usize = 16; // max per-call answer steps per callee (v3 item 6)

#[derive(Clone, Default)]
struct Callee {
    id: u32,
    conv: String,
    nargs: usize,
    pop: u32,
    ret: String,
    stub_addr: u32,
    tail_addr: u32,                         // E9 tail-patch variant (0 when unbuilt)
    writes: Vec<(u8, usize, usize, usize, usize)>, // (kind 0=stack arg,1=ecx,2=edx; idx; wstart; nwords; dst_off)
    snap: Vec<snap::SnapSpec>, // pointed-to snapshots (v5: any byte offset, 64 words)
    // Logged and rewrite-side-transported vector registers. Index 0/1 are
    // the v2/v3 logxmm/xmm0_from_stack and logxmm1/xmm1_from_stack keys;
    // v5 adds XMM2-XMM7 (logxmm_regs, xmm_from_stack).
    xmm: vecregs::XmmCallCfg,
    preserve: bool, // v4: restore entry ecx/edx from m_save slots at stub exit
    eax_from_stack: Option<usize>, // v4: rw-side transport, load eax from stack arg
    noclean: bool, // v4: real callee pops nothing; stub pops only on the rw side
    pop_rw: u32,   // v4: rewrite-side pop for noclean stubs (nargs*4)
}

struct State {
    img: usize,
    delta: u32,
    image_size: usize,
    text_lo: usize,
    text_hi: usize,
    data_ranges: Vec<(usize, usize)>, // mapped (lo,hi) of writable data sections
    pristine: Vec<u8>,                // post-reloc snapshot of data_ranges, concatenated
    #[allow(dead_code)]
    data_off: Vec<usize>,
    s: usize, // scratch stack region
    h: usize, // scratch heap region
    meta: usize,
    m_ctx: u32,
    m_fn: u32,
    m_hostesp: u32,
    m_mxcsr: u32,
    m_tmp: u32,
    m_fault: u32, // fault flag + record (9 dwords)
    m_script_lo: u32,
    m_script_hi: u32,
    m_logidx: u32,
    m_side: u32,     // 0 = original side, 1 = rewrite side (for xmm transport)
    m_save_ecx: u32, // stub-entry spill for register out-param writes
    m_save_edx: u32,
    m_save_eax: u32, // v4: stub-entry spill for ret="preserve" callees
    m_xmm_mirror: u32, // 32 words: scripted xmm0-7 entry values, readable by rewrites
    m_x87_mirror: u32, // v5: x87 entry count + 8 x 16-byte ST(i) slots, readable by rewrites
    ext_snap_base: u32, // v5: extension log for snapshot words 8-63
    ext_xmm_base: u32,  // v5: extension log for XMM2-XMM7
    selftests: HashMap<String, u32>, // v5: built-in self-test originals
    image_base: usize,  // preferred base from the PE header
    abs: AbsWindow,     // v5: preferred-base window (guard / shadow)
    m_tls_mirror: u32, // 256 words: fabricated TLS slot values, readable by rewrites
    m_script_tab: u32, // 256 x (lo,hi) per-callee script slots
    m_writebuf: u32,   // 256 x 16 per-callee out-param write words
    m_seq_tab: u32,    // v3: 256 x 16 per-call answer steps (lo,hi)
    m_seq_len: u32,    // v3: 256 sequence lengths (trial_body fills)
    m_seq_idx: u32,    // v3: 256 per-side consumption indexes (run_side zeroes)
    log_max: u32,      // v4: setup-time call-log cap (default 256, max 1024)
    m_step: u32,       // v3: stub scratch for the clamped step index
    tls_helper: u32,   // emitted mov eax,fs:[0x2c]; ret
    log_base: u32,
    stub_base: u32,
    stub_off: usize,
    ctable: u32,
    fxbuf: u32,
    tramp: usize,
    fault_pad: usize,
    esp0: u32,
    ctx: Vec<u32>,
    before_stack: Vec<u32>,
    exe_bytes: Vec<u8>,
    segs: Vec<(usize, usize, usize, usize)>, // (vaddr,rva? no: vaddr,size,rawptr,rawsz)
    _nsec: usize,
    _sely: usize,
    _headers_len: usize,
    _opt: usize,
    dll: *mut c_void,
    exports: HashMap<String, u32>,
    callees: HashMap<u32, Callee>,
    patches: Vec<(usize, [u8; 8], usize)>, // (addr, orig bytes, len 5 or 4)
    globals: Vec<(usize, usize)>,          // declared global ranges (mapped lo, len)
    relocs: u32,
    side_rw: bool, // true while the rewrite runs (anti-cheat active)
    mapped: bool,
}

impl State {
    fn new() -> State {
        State {
            img: 0,
            delta: 0,
            image_size: 0,
            text_lo: 0,
            text_hi: 0,
            data_ranges: Vec::new(),
            pristine: Vec::new(),
            data_off: Vec::new(),
            s: 0,
            h: 0,
            meta: 0,
            m_ctx: 0,
            m_fn: 0,
            m_hostesp: 0,
            m_mxcsr: 0,
            m_tmp: 0,
            m_fault: 0,
            m_script_lo: 0,
            m_script_hi: 0,
            m_logidx: 0,
            m_side: 0,
            m_save_ecx: 0,
            m_save_edx: 0,
            m_save_eax: 0,
            m_xmm_mirror: 0,
            m_x87_mirror: 0,
            ext_snap_base: 0,
            ext_xmm_base: 0,
            selftests: HashMap::new(),
            image_base: 0,
            abs: AbsWindow::default(),
            m_tls_mirror: 0,
            m_script_tab: 0,
            m_writebuf: 0,
            m_seq_tab: 0,
            m_seq_len: 0,
            m_seq_idx: 0,
            log_max: LOG_MAX as u32,
            m_step: 0,
            tls_helper: 0,
            log_base: 0,
            stub_base: 0,
            stub_off: 0,
            ctable: 0,
            fxbuf: 0,
            tramp: 0,
            fault_pad: 0,
            esp0: 0,
            // +32: xmm0-7 entry values (v2); +25: x87 entry count and values (v5)
            ctx: vec![0u32; CTX_WORDS],
            before_stack: Vec::new(),
            exe_bytes: Vec::new(),
            segs: Vec::new(),
            _nsec: 0,
            _sely: 0,
            _headers_len: 0,
            _opt: 0,
            dll: std::ptr::null_mut(),
            exports: HashMap::new(),
            callees: HashMap::new(),
            patches: Vec::new(),
            globals: Vec::new(),
            relocs: 0,
            side_rw: false,
            mapped: false,
        }
    }
}

// v5 preferred-base window. `regions` are the reservations this worker
// holds there (made at start-up, extended at map time); `lo..hi` is the
// image's window once mapped. With a shadow, `runs` are the committed
// read-only pages (absolute, split per reservation) and `readable` tracks
// their current protection so a side switch only calls VirtualProtect when
// it changes.
#[derive(Default)]
struct AbsWindow {
    regions: Vec<(usize, usize)>,
    lo: usize,
    hi: usize,
    runs: Vec<(usize, usize)>,
    built: bool,      // shadow pages committed and filled (once per process)
    active: bool,     // the current contract declared abs_shadow
    readable: bool,   // shadow pages currently PAGE_READONLY
}

static mut ST: *mut State = std::ptr::null_mut();

fn st() -> &'static mut State {
    // SAFETY: ST is set once in main before any trial runs and never moves.
    unsafe { &mut *std::ptr::addr_of_mut!(ST).read() }
}

fn diag(msg: &str) {
    let _ = writeln!(std::io::stderr(), "worker: {}", msg);
}

// In-process fault trap with recovery: record the fault, then resume at the
// trampoline's fault landing pad (which restores the host stack and returns).
// Never kills the worker; a nested fault exits loudly.
extern "system" fn veh(p: *mut c_void) -> i32 {
    unsafe {
        let s = &mut *std::ptr::addr_of_mut!(ST).read();
        let flag = s.m_fault as *mut u32;
        if *flag != 0 {
            // fault while already handling one: cannot recover
            ExitProcess(11);
        }
        let rec = *(p as *const *const u32);
        let code = *rec.add(0);
        let nparams = *rec.add(4);
        let acc = if nparams >= 1 {
            *rec.add(5)
        } else {
            0xFFFFFFFF
        };
        let badva = if nparams >= 2 { *rec.add(6) } else { 0 };
        let ctxr = *((p as *const u32).add(1) as *const *const u32);
        let g = |off: usize| *(((ctxr as *const u8).add(off)) as *const u32);
        let eip = g(184);
        // record: flag,code,eip,access,badva,eax,ecx,edx,ebx,esi,edi,ebp,esp,eflags
        *flag.add(0) = 1;
        *flag.add(1) = code;
        *flag.add(2) = eip;
        *flag.add(3) = acc;
        *flag.add(4) = badva;
        *flag.add(5) = g(176);
        *flag.add(6) = g(172);
        *flag.add(7) = g(168);
        *flag.add(8) = g(164);
        *flag.add(9) = g(160);
        *flag.add(10) = g(156);
        *flag.add(11) = g(180);
        *flag.add(12) = g(196);
        *flag.add(13) = g(192);
        // resume on the host stack at the landing pad
        let hostesp = *(s.m_hostesp as *const u32);
        let ctxw = *((p as *const u32).add(1) as *const *mut u8);
        *((ctxw.add(184)) as *mut u32) = s.fault_pad as u32;
        *((ctxw.add(196)) as *mut u32) = hostesp;
        -1 // EXCEPTION_CONTINUE_EXECUTION
    }
}

fn u32le(v: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([v[off], v[off + 1], v[off + 2], v[off + 3]])
}
fn u16le(v: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([v[off], v[off + 1]])
}

fn map_at(addr: usize, size: usize) -> Option<usize> {
    let p = unsafe {
        VirtualAlloc(
            addr as *mut c_void,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_RWX,
        )
    };
    if p.is_null() || p as usize != addr {
        None
    } else {
        Some(addr)
    }
}

fn protect(addr: usize, size: usize, prot: u32) {
    let mut old = 0u32;
    let r = unsafe { VirtualProtect(addr as *mut c_void, size, prot, &mut old) };
    if r == 0 {
        diag(&format!("VirtualProtect {:#x} failed", addr));
    }
}

// Map the executable at a relocated base with relocations applied (runfn logic).
fn map_image(exe: &[u8]) -> Result<(), String> {
    let s = st();
    if exe.len() < 0x200 || u16le(exe, 0) != 0x5A4D {
        return Err("bad MZ".to_string());
    }
    let peoff = u32le(exe, 0x3C) as usize;
    if u32le(exe, peoff) != 0x00004550 {
        return Err("bad PE sig".to_string());
    }
    let coff = peoff + 4;
    let nsec = u16le(exe, coff + 2) as usize;
    let optsz = u16le(exe, coff + 16) as usize;
    let opt = coff + 20;
    if u16le(exe, opt) != 0x10B || optsz < 96 {
        return Err("not PE32".to_string());
    }
    let image_base = u32le(exe, opt + 28) as usize;
    let image_size = (u32le(exe, opt + 56) as usize + 0xFFF) & !0xFFF;
    let headers_len = u32le(exe, opt + 60) as usize;
    let sely = coff + 20 + optsz;

    let image_cands: [usize; 4] = [0x1000_0000, 0x4000_0000, 0x0500_0000, 0x0C00_0000];
    let stack_cands: [usize; 3] = [0x3100_0000, 0x2900_0000, 0x0900_0000];
    let heap_cands: [usize; 3] = [0x3000_0000, 0x2800_0000, 0x0A00_0000];

    let mut ibase = None;
    for &c in &image_cands {
        if let Some(b) = map_at(c, image_size) {
            ibase = Some(b);
            break;
        }
    }
    let img = ibase.ok_or("cannot map image anywhere")?;
    let delta = (img as i64 - image_base as i64) as u32;
    let mut sbase = None;
    for &c in &stack_cands {
        if let Some(b) = map_at(c, SSIZE) {
            sbase = Some(b);
            break;
        }
    }
    let sb = sbase.ok_or("cannot map scratch stack")?;
    let mut hbase = None;
    for &c in &heap_cands {
        if let Some(b) = map_at(c, HSIZE) {
            hbase = Some(b);
            break;
        }
    }
    let hb = hbase.ok_or("cannot map scratch heap")?;

    unsafe {
        std::ptr::copy_nonoverlapping(exe.as_ptr(), img as *mut u8, headers_len.min(exe.len()));
    }
    let mut segs = Vec::new();
    for i in 0..nsec {
        let sh = sely + i * 40;
        let vsize = u32le(exe, sh + 8) as usize;
        let vaddr = u32le(exe, sh + 12) as usize;
        let rawsz = u32le(exe, sh + 16) as usize;
        let rawptr = u32le(exe, sh + 20) as usize;
        if rawsz > 0 && rawptr + rawsz <= exe.len() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    exe.as_ptr().add(rawptr),
                    (img + vaddr) as *mut u8,
                    rawsz,
                );
            }
        }
        segs.push((img + vaddr, vsize.max(rawsz), rawptr, rawsz));
        let mut nm = [0u8; 8];
        nm.copy_from_slice(&exe[sh..sh + 8]);
        let _ = nm;
    }
    let rva2off = |rva: usize| -> Option<usize> {
        if rva < headers_len {
            return Some(rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            let rawptr = u32le(exe, sh + 20) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(rawptr + (rva - vaddr));
            }
        }
        None
    };
    let mut relocs = 0u32;
    let reloc_rva = u32le(exe, opt + 96 + 40) as usize;
    let reloc_size = u32le(exe, opt + 96 + 44) as usize;
    if reloc_rva != 0
        && reloc_size != 0
        && let Some(ro) = rva2off(reloc_rva)
    {
        let mut p = ro;
        let end = ro + reloc_size;
        while p + 8 <= end && p + 8 <= exe.len() {
            let page = u32le(exe, p) as usize;
            let bsz = u32le(exe, p + 4) as usize;
            if bsz < 8 {
                break;
            }
            let mut q = p + 8;
            while q + 2 <= p + bsz && q + 2 <= exe.len() {
                let e = u16le(exe, q);
                q += 2;
                if e >> 12 == 3 {
                    let at = img + page + ((e & 0xFFF) as usize);
                    unsafe {
                        let v = std::ptr::read_unaligned(at as *const u32);
                        std::ptr::write_unaligned(at as *mut u32, v.wrapping_add(delta));
                    }
                    relocs += 1;
                }
            }
            p += (bsz + 3) & !3;
            if p <= ro {
                break;
            }
        }
    }

    s.img = img;
    s.delta = delta;
    s.image_size = image_size;
    s.s = sb;
    s.h = hb;
    s.segs = segs;
    s._nsec = nsec;
    s._sely = sely;
    s._headers_len = headers_len;
    s._opt = opt;
    s.relocs = relocs;
    s.image_base = image_base;

    // .text range (for anti-cheat protection) + writable data ranges
    for i in 0..nsec {
        let sh = sely + i * 40;
        let name = &exe[sh..sh + 8];
        let vaddr = u32le(exe, sh + 12) as usize;
        let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
        let chars = u32le(exe, sh + 36);
        if name.starts_with(b".text") {
            s.text_lo = img + vaddr;
            s.text_hi = img + vaddr + vsize;
        }
        if chars & 0x80000000 != 0 {
            // writable section: track for full diff (skip the stub .rkstr? keep all)
            s.data_ranges.push((img + vaddr, img + vaddr + vsize));
        }
    }
    // pristine snapshot of writable ranges (post-reloc)
    let mut pris = Vec::new();
    let mut offs = Vec::new();
    for &(lo, hi) in &s.data_ranges {
        offs.push(pris.len());
        pris.extend_from_slice(unsafe { std::slice::from_raw_parts(lo as *const u8, hi - lo) });
    }
    s.pristine = pris;
    s.data_off = offs;

    // layout inside scratch stack region
    let meta = sb + 0xFF000;
    s.meta = meta;
    s.m_ctx = meta as u32;
    s.m_fn = (meta + 4) as u32;
    s.m_hostesp = (meta + 8) as u32;
    s.m_mxcsr = (meta + 12) as u32;
    s.m_tmp = (meta + 16) as u32;
    s.m_fault = (meta + 32) as u32;
    s.m_script_lo = (meta + 96) as u32;
    s.m_script_hi = (meta + 100) as u32;
    s.m_logidx = (meta + 104) as u32;
    s.m_side = (meta + 108) as u32;
    s.m_save_ecx = (meta + 112) as u32;
    s.m_save_edx = (meta + 116) as u32;
    s.m_step = (meta + 120) as u32;
    // v4: meta+124 was the last free word between m_step and the XMM mirror.
    s.m_save_eax = (meta + 124) as u32;
    s.m_xmm_mirror = (meta + 128) as u32;
    s.m_tls_mirror = (meta + 384) as u32;
    // v5: the x87 mirror follows the TLS mirror (256 words end at meta+1408).
    s.m_x87_mirror = (meta + META_X87_MIRROR) as u32;
    // v4 layout inside the scratch stack region (all below the snapshot window):
    // stubs 0x1000-0x11000, call log 0x12000-0x56000 (1024 x 272B max),
    // ctable 0x56000, per-callee script table 0x57000 (256 x 8B), out-param
    // write buffer 0x58000-0x5C000 (256 x 16 words), per-call answer
    // sequences 0x5C000-0x64000 (256 x 16 steps x 8B), sequence lengths
    // 0x64000 (256 dwords), per-side sequence indexes 0x65000 (256 dwords).
    s.log_base = (sb + 0x12000) as u32;
    s.stub_base = (sb + 0x1000) as u32;
    s.ctable = (sb + 0x56000) as u32;
    s.m_script_tab = (sb + 0x57000) as u32;
    s.m_writebuf = (sb + 0x58000) as u32;
    s.m_seq_tab = (sb + 0x5C000) as u32;
    s.m_seq_len = (sb + 0x64000) as u32;
    s.m_seq_idx = (sb + 0x65000) as u32;
    // v5 extension logs (see EXT_SNAP_OFF): above the sequence indexes,
    // below the scratch-fill region.
    s.ext_snap_base = (sb + EXT_SNAP_OFF) as u32;
    s.ext_xmm_base = (sb + EXT_XMM_OFF) as u32;
    s.fxbuf = (sb + 0xFD800) as u32;
    s.tramp = sb + 0xFE000;
    s.esp0 = (sb + 0xFD000) as u32;
    unsafe {
        *((s.m_mxcsr) as *mut u32) = 0x1F80;
        *((s.m_ctx) as *mut u32) = s.ctx.as_ptr() as u32;
        // guard page at the bottom of the stack region
        protect(sb, 0x1000, PAGE_NOACCESS);
        // guard at the top of the heap region
        protect(hb + HEAP_USE, HSIZE - HEAP_USE, PAGE_NOACCESS);
        // callee table: 0 = undeclared (call faults honestly)
        std::ptr::write_bytes(s.ctable as *mut u8, 0, 1024);
        std::ptr::write_bytes(s.m_script_tab as *mut u8, 0, 2048);
        std::ptr::write_bytes(s.m_writebuf as *mut u8, 0, 16384);
        std::ptr::write_bytes(s.m_seq_tab as *mut u8, 0, 32768);
        std::ptr::write_bytes(s.m_seq_len as *mut u8, 0, 1024);
        std::ptr::write_bytes(s.m_seq_idx as *mut u8, 0, 1024);
        std::ptr::write_bytes(s.m_x87_mirror as *mut u8, 0, X87_MIRROR_LEN);
    }
    build_trampoline();
    build_selftests();
    // v5: hold the image's preferred-base window (start-up reserved the
    // conventional one; this extends it to the image's real extent).
    let (wlo, whi) = abswin::window(image_base, image_size);
    abs_reserve(wlo, whi);
    s.abs.lo = wlo;
    s.abs.hi = whi;
    // emitted TLS helper: mov eax,fs:[0x2c]; ret (this thread's TEB
    // TLS-slot array pointer). The first 16 stub bytes are reserved for it
    // (see the stub_off reset in setup); a stub emitted at offset 0 once
    // overwrote it and its `ret 4` corrupted the caller frame.
    s.tls_helper = s.stub_base;
    s.stub_off = 16;
    unsafe {
        let code: [u8; 7] = [0x64, 0xA1, 0x2C, 0x00, 0x00, 0x00, 0xC3];
        std::ptr::copy_nonoverlapping(code.as_ptr(), s.tls_helper as *mut u8, 7);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, s.tls_helper as *const c_void, 7);
    }
    s.mapped = true;
    Ok(())
}

// Trampoline: runfn's setup with a fault landing pad. Layout mirrors runfn so
// the byte sequence stays reviewable against the proven version.
fn build_trampoline() {
    let s = st();
    let (t, pad_off) = trampoline_bytes();
    unsafe {
        std::ptr::copy_nonoverlapping(t.as_ptr(), s.tramp as *mut u8, t.len());
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, s.tramp as *const c_void, t.len());
    }
    s.fault_pad = s.tramp + pad_off;
}

// The trampoline's bytes and the offset of its fault landing pad (split
// from build_trampoline in v5 so the encoding is unit-testable).
fn trampoline_bytes() -> (Vec<u8>, usize) {
    let s = st();
    let (m_ctx, m_fn, m_hostesp, m_mxcsr, m_tmp, fxbuf) =
        (s.m_ctx, s.m_fn, s.m_hostesp, s.m_mxcsr, s.m_tmp, s.fxbuf);
    let mut t: Vec<u8> = Vec::with_capacity(2400);
    let u = |t: &mut Vec<u8>, v: u32| t.extend_from_slice(&v.to_le_bytes());
    t.push(0x60); // pushad
    t.push(0x9C); // pushfd
    t.push(0x89);
    t.push(0x25);
    u(&mut t, m_hostesp); // mov [m_hostesp],esp (after pushes)
    t.push(0xDB);
    t.push(0xE3); // fninit
    t.push(0x0F);
    t.push(0xAE);
    t.push(0x15);
    u(&mut t, m_mxcsr); // ldmxcsr
    t.push(0x8B);
    t.push(0x2D);
    u(&mut t, m_ctx); // mov ebp,[m_ctx]
    t.extend_from_slice(&[0x8B, 0x4D, 0x00]); // ecx
    t.extend_from_slice(&[0x8B, 0x55, 0x04]); // edx
    t.extend_from_slice(&[0x8B, 0x5D, 0x08]); // ebx
    t.extend_from_slice(&[0x8B, 0x75, 0x0C]); // esi
    t.extend_from_slice(&[0x8B, 0x7D, 0x10]); // edi
    // v2: scripted XMM entry values from ctx[145+N*4] (byte disp 580+N*16).
    // ebp still holds the ctx pointer here.
    for n in 0..8u32 {
        t.extend_from_slice(&[0x0F, 0x10, 0x85 | ((n as u8) << 3)]);
        u(&mut t, 580 + n * 16); // movups xmmN,[ebp+disp]
    }
    // v5: x87 entry values. ST(n-1) is pushed first and ST(0) last, so
    // ST(0) ends on top; ctx[CTX_X87_N] is 0 on the rewrite side and for
    // contracts without x87 entries, which loads nothing. The compare and
    // branch chain runs between pushfd/popfd on the host stack, so the
    // function still enters with the EFLAGS it had in v4.
    t.push(0x9C); // pushfd
    for k in (0..x87::X87_MAX_ENTRIES as u32).rev() {
        t.extend_from_slice(&[0x83, 0xBD]);
        u(&mut t, (CTX_X87_N * 4) as u32);
        t.push(k as u8); // cmp dword [ebp+n],k
        t.extend_from_slice(&[0x76, 0x06]); // jbe +6 (skip the fld)
        t.extend_from_slice(&[0xDB, 0xAD]);
        u(&mut t, (CTX_X87_VALS as u32 + 3 * k) * 4); // fld tbyte [ebp+slot k]
    }
    t.push(0x9D); // popfd
    t.extend_from_slice(&[0x8B, 0x45, 0x18]); // eax_in
    t.push(0xA3);
    u(&mut t, m_tmp + 8);
    t.extend_from_slice(&[0x8B, 0x45, 0x14]); // ebp_in
    t.push(0xA3);
    u(&mut t, m_tmp + 12);
    t.extend_from_slice(&[0x8B, 0x65, 0x1C]); // esp = scratch
    t.push(0xA1);
    u(&mut t, m_tmp + 12);
    t.extend_from_slice(&[0x89, 0xC5]); // ebp
    t.push(0xA1);
    u(&mut t, m_tmp + 8); // eax
    t.push(0xFF);
    t.push(0x15);
    u(&mut t, m_fn); // call [m_fn]
    // normal return path
    t.push(0x89);
    t.push(0x2D);
    u(&mut t, m_tmp); // mov [m_tmp],ebp
    t.push(0x89);
    t.push(0x25);
    u(&mut t, m_tmp + 4); // mov [m_tmp+4],esp
    t.push(0x8B);
    t.push(0x2D);
    u(&mut t, m_ctx);
    t.extend_from_slice(&[0x89, 0x45, 0x20]); // eax
    t.extend_from_slice(&[0x89, 0x4D, 0x24]); // ecx
    t.extend_from_slice(&[0x89, 0x55, 0x28]); // edx
    t.extend_from_slice(&[0x89, 0x5D, 0x2C]); // ebx
    t.extend_from_slice(&[0x89, 0x75, 0x30]); // esi
    t.extend_from_slice(&[0x89, 0x7D, 0x34]); // edi
    t.push(0xA1);
    u(&mut t, m_tmp);
    t.extend_from_slice(&[0x89, 0x45, 0x38]); // ebp
    t.push(0xA1);
    u(&mut t, m_tmp + 4);
    t.extend_from_slice(&[0x89, 0x45, 0x3C]); // esp
    t.push(0x9C);
    t.push(0x58);
    t.extend_from_slice(&[0x89, 0x45, 0x40]); // eflags
    t.push(0x0F);
    t.push(0xAE);
    t.push(0x05);
    u(&mut t, fxbuf); // fxsave
    for i in 0..128u32 {
        t.push(0xA1);
        u(&mut t, fxbuf + i * 4);
        t.push(0x89);
        t.push(0x85);
        u(&mut t, 68 + i * 4);
    }
    // v5: empty the FPU after the state is saved. With x87 entry values an
    // original may leave up to eight registers in use, and the worker's own
    // code (which uses x87 for f64 returns) must not run on a full stack.
    t.extend_from_slice(&[0xDB, 0xE3]); // fninit
    t.push(0x8B);
    t.push(0x25);
    u(&mut t, m_hostesp);
    t.push(0x9D); // popfd
    t.push(0x61); // popad
    t.push(0xC3); // ret
    // fault landing pad: VEH resumes here with esp=host (post-pushad/pushfd)
    let pad_off = t.len();
    t.extend_from_slice(&[0xDB, 0xE3]); // fninit (v5, as above)
    t.push(0x9D); // popfd
    t.push(0x61); // popad
    t.push(0xC3); // ret
    (t, pad_off)
}

// v5 built-in self-test originals, for the checker's own regression only
// (contract "function": "selftest:<name>"; the verdict marks them, and a
// self-test is never a function verification). Each is a few hand-written
// instructions exercising one channel no tracked original is known to use:
// - x87_store (cdecl, p): fstp dword [p] (ST0), fstp qword [p+4] (ST1),
//   fstp tbyte [p+12] (ST2), return p. Consumes three x87 entry values.
// - xmm_call (cdecl, a, b): xmm2 = a, xmm5 = b (movd, upper lanes zero),
//   call callee 1 through the stub table with no stack arguments, return
//   the low word of the xmm6 entry value.
// - abs_read (cdecl, addr): return the dword at addr, an absolute address
//   used as is (the shape of an unrelocated absolute operand).
fn build_selftests() {
    let s = st();
    let base = s.s + SELFTEST_OFF;
    let (code, table) = selftest_code(base, s.ctable + 4);
    unsafe {
        std::ptr::copy_nonoverlapping(code.as_ptr(), base as *mut u8, code.len());
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, base as *const c_void, code.len());
    }
    s.selftests = table;
}

// The self-test page for a page at `base`, with callee 1's stub-table slot
// at `ctab1`: the code and each routine's address.
fn selftest_code(base: usize, ctab1: u32) -> (Vec<u8>, HashMap<String, u32>) {
    let mut code: Vec<u8> = Vec::new();
    let mut table: HashMap<String, u32> = HashMap::new();
    let mut add = |name: &str, bytes: &[u8]| {
        while !code.len().is_multiple_of(16) {
            code.push(0xCC);
        }
        table.insert(name.to_string(), (base + code.len()) as u32);
        code.extend_from_slice(bytes);
    };
    add(
        "x87_store",
        &[
            0x8B, 0x4C, 0x24, 0x04, // mov ecx,[esp+4]
            0xD9, 0x19, // fstp dword [ecx]
            0xDD, 0x59, 0x04, // fstp qword [ecx+4]
            0xDB, 0x79, 0x0C, // fstp tbyte [ecx+12]
            0x89, 0xC8, // mov eax,ecx
            0xC3, // ret
        ],
    );
    let mut xc: Vec<u8> = vec![
        0x8B, 0x44, 0x24, 0x04, // mov eax,[esp+4]
        0x66, 0x0F, 0x6E, 0xD0, // movd xmm2,eax
        0x8B, 0x44, 0x24, 0x08, // mov eax,[esp+8]
        0x66, 0x0F, 0x6E, 0xE8, // movd xmm5,eax
        0xFF, 0x15, // call [ctable+4]
    ];
    xc.extend_from_slice(&ctab1.to_le_bytes());
    xc.extend_from_slice(&[
        0x66, 0x0F, 0x7E, 0xF0, // movd eax,xmm6
        0xC3, // ret
    ]);
    add("xmm_call", &xc);
    add(
        "abs_read",
        &[
            0x8B, 0x44, 0x24, 0x04, // mov eax,[esp+4]
            0x8B, 0x00, // mov eax,[eax]
            0xC3, // ret
        ],
    );
    assert!(code.len() <= 0x1000, "self-test page overflow");
    (code, table)
}

// v5: reserve `size` bytes at exactly `addr` (no commit: any access
// faults). None when something else holds part of the range.
fn reserve_at(addr: usize, size: usize) -> Option<usize> {
    let p = unsafe { VirtualAlloc(addr as *mut c_void, size, MEM_RESERVE, PAGE_NOACCESS) };
    if p.is_null() {
        return None;
    }
    if p as usize != addr {
        unsafe {
            VirtualFree(p, 0, MEM_RELEASE);
        }
        return None;
    }
    Some(addr)
}

// v5: hold [lo, hi) of the preferred-base window, as one reservation when
// the whole stretch is free, else chunk by chunk (skipping chunks the
// process already uses; the setup response reports the coverage).
fn abs_reserve(lo: usize, hi: usize) {
    let s = st();
    let missing = abswin::uncovered_chunks(lo, hi, &s.abs.regions);
    if missing.is_empty() {
        return;
    }
    if missing.len() == (hi - lo) / abswin::ABS_CHUNK && reserve_at(lo, hi - lo).is_some() {
        s.abs.regions.push((lo, hi));
    } else {
        for c in missing {
            if reserve_at(c, abswin::ABS_CHUNK).is_some() {
                s.abs.regions.push((c, c + abswin::ABS_CHUNK));
            }
        }
    }
    s.abs.regions.sort_unstable();
}

// v5: commit and fill the read-only shadow once per process: the file's
// unrelocated headers and read-only sections at the preferred base (what
// the game sees there), left PAGE_NOACCESS until a side makes it readable.
fn abs_build_shadow() -> Result<(), String> {
    let s = st();
    if s.abs.built {
        return Ok(());
    }
    let exe = &s.exe_bytes;
    let mut secs = Vec::new();
    let mut raws = Vec::new();
    for i in 0..s._nsec {
        let sh = s._sely + i * 40;
        let vaddr = u32le(exe, sh + 12) as usize;
        let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
        let writable = u32le(exe, sh + 36) & 0x80000000 != 0;
        secs.push(abswin::SectionInfo {
            vaddr,
            vsize,
            writable,
        });
        raws.push((u32le(exe, sh + 20) as usize, u32le(exe, sh + 16) as usize));
    }
    let base = s.image_base;
    let runs: Vec<(usize, usize)> = abswin::shadow_runs(s._headers_len, &secs, s.image_size)
        .iter()
        .map(|&(a, b)| (base + a, base + b))
        .collect();
    let split = abswin::split_runs(&runs, &s.abs.regions).map_err(|(a, b)| {
        format!(
            "abs_shadow: {:#x}..{:#x} of the preferred-base window is not held by the worker (another allocation sits there; restart the worker)",
            a, b
        )
    })?;
    for &(a, b) in &split {
        let p = unsafe { VirtualAlloc(a as *mut c_void, b - a, MEM_COMMIT, PAGE_READWRITE) };
        if p as usize != a {
            return Err(format!("abs_shadow: cannot commit {:#x}..{:#x}", a, b));
        }
    }
    unsafe {
        let hl = s._headers_len.min(exe.len());
        std::ptr::copy_nonoverlapping(exe.as_ptr(), base as *mut u8, hl);
        for (sec, &(rawptr, rawsz)) in secs.iter().zip(raws.iter()) {
            if sec.writable || rawsz == 0 || rawptr + rawsz > exe.len() {
                continue;
            }
            let n = rawsz.min(s.image_size.saturating_sub(sec.vaddr));
            std::ptr::copy_nonoverlapping(exe.as_ptr().add(rawptr), (base + sec.vaddr) as *mut u8, n);
        }
    }
    for &(a, b) in &split {
        protect(a, b - a, PAGE_NOACCESS);
    }
    s.abs.runs = split;
    s.abs.built = true;
    s.abs.readable = false;
    Ok(())
}

// v5: make the shadow readable (original side of an abs_shadow contract)
// or inaccessible (rewrite side, and every other contract).
fn abs_set_readable(readable: bool) {
    let s = st();
    if !s.abs.built || s.abs.readable == readable {
        return;
    }
    let prot = if readable { PAGE_READONLY } else { PAGE_NOACCESS };
    for &(a, b) in &s.abs.runs.clone() {
        protect(a, b - a, prot);
    }
    s.abs.readable = readable;
}

// Per-callsite recorder stub (machine code). Entry: esp->[ret][a0..].
// Logs (id,ecx,edx,ebx,esi,edi,nargs,args[40],snap[64],xmm0-7) to the
// call log (v5: snapshot words 8-63 and xmm2-7 in the extension logs), performs scripted out-param writes, then returns the callee's
// current per-call sequence step with the callee's cleanup discipline
// (or pop_override for tail stubs).
fn emit_stub(c: &Callee, tail_pop: Option<u32>) -> Vec<u8> {
    // tail_pop = Some(outer_pop): E9 tail-patch variant. The patched E8
    // pushed a return address (site+5) that must be discarded: the stub
    // does `add esp,4` then `ret outer_pop`, landing back at the
    // trampoline. (A plain `ret outer_pop+4` would resume at site+5.)
    let s = st();
    let mut t: Vec<u8> = Vec::new();
    let u = |t: &mut Vec<u8>, v: u32| t.extend_from_slice(&v.to_le_bytes());
    // v3: tail stubs read stack arguments one word lower. A patched E9 site
    // calls the stub with two return addresses on the stack (site+5, then
    // the trampoline return), so the caller's arg0 is at [esp+8], not
    // [esp+4] (lanes r-n100/r-n104/r-s79: v2 logged the trampoline return
    // address as arg0 for every tail call with stack arguments). This
    // assumes the tail site executes with entry ESP, which holds for
    // register-adjust + jump thunks; the contract author must confirm it.
    let arg_base: u32 = if tail_pop.is_some() { 8 } else { 4 };
    // Spill entry ecx/edx: the arg loop below clobbers ecx, and register
    // out-param writes (emitted later) need the original values.
    t.extend_from_slice(&[0x89, 0x0D]);
    u(&mut t, s.m_save_ecx); // mov [m_save_ecx],ecx
    t.extend_from_slice(&[0x89, 0x15]);
    u(&mut t, s.m_save_edx); // mov [m_save_edx],edx
    // v4 (lanes r-b79, r-s111): ret="preserve" callees must exit with the
    // entry registers intact, and eax-argument callees must log entry eax,
    // so spill eax first. Everything below clobbers it (starting with the
    // logidx load); the preserve arm restores all three.
    if c.ret == "preserve" || c.eax_from_stack.is_some() {
        t.push(0xA3);
        u(&mut t, s.m_save_eax); // mov [m_save_eax],eax
    }
    // v2 rewrite-side transport for xmm0-arg callees: on the rewrite side
    // only, load xmm0 from the declared stack arg before logging it.
    // NOTE: disp32 SIB forms (modrm 0x84/0x8C): a transport index past 30
    // would overflow disp8 (v2's latent form is fixed here too).
    // v3 added xmm1 (lanes r-b03, r-b24); v5 generalises to XMM0-XMM7 in
    // register order, byte-identical to v4 for xmm0/xmm1-only callees.
    for (reg, idx) in c
        .xmm
        .from_stack
        .iter()
        .enumerate()
        .filter_map(|(r, i)| i.map(|i| (r, i)))
    {
        t.extend_from_slice(&[0x83, 0x3D]); // cmp dword [m_side],0
        u(&mut t, s.m_side);
        t.push(0x00);
        t.extend_from_slice(&[0x74, 0x09]); // je +9 (skip the 9-byte movss)
        // movss xmm<reg>,[esp+4+idx*4]
        t.extend_from_slice(&vecregs::movss_from_esp(reg, 4 + idx as u32 * 4));
    }
    // v4 rewrite-side transport for eax-arg callees (lane r-s111): on the
    // rewrite side only, load eax from the declared stack arg. A Rust
    // rewrite cannot set eax for a call any other way; the original holds
    // the same value in eax genuinely, so comparing the logged entry eax
    // still verifies the value, only the transport differs. Base is 4 even
    // in tail stubs: transports run on the rewrite side only, which always
    // calls the normal stub via ctable (r-s101). disp32 SIB form, so any
    // declared index is encodable.
    if let Some(idx) = c.eax_from_stack {
        t.extend_from_slice(&[0x83, 0x3D]); // cmp dword [m_side],0
        u(&mut t, s.m_side);
        t.push(0x00);
        t.extend_from_slice(&[0x74, 0x07]); // je +7 (skip the 7-byte mov)
        t.extend_from_slice(&[0x8B, 0x84, 0x24]);
        u(&mut t, 4 + idx as u32 * 4); // mov eax,[esp+4+idx*4]
        // Re-spill: the logged eax is the post-transport value on the
        // rewrite side (the entry spill above predates the transport).
        t.push(0xA3);
        u(&mut t, s.m_save_eax); // mov [m_save_eax],eax
    }
    // eax = logidx; if >= cap skip logging (writes + scripted return still run).
    // v4: the cap is the setup-time log_max (default 256), so raising it
    // needs no layout change; the region always reserves LOG_HARD_MAX.
    t.push(0xA1);
    u(&mut t, s.m_logidx); // mov eax,[m_logidx]
    t.extend_from_slice(&[0x3D]);
    u(&mut t, s.log_max); // cmp eax,log_max
    // jae, not ja: with ja the entry at index 256 was written one past the
    // log, onto the callee stub table that follows it, and a correct rewrite
    // making more than 256 calls in a trial then faulted (three lanes hit this).
    t.extend_from_slice(&[0x0F, 0x83, 0x00, 0x00, 0x00, 0x00]); // jae full (rel32, patched below)
    let jae_pos = t.len() - 4;
    t.extend_from_slice(&[0x69, 0xC0]); // imul eax,eax,128 (imm32 form)
    u(&mut t, LOG_ENTRY as u32);
    t.extend_from_slice(&[0x05]);
    u(&mut t, s.log_base); // add eax,log_base
    // store id, ecx, edx, ebx, esi, edi, nargs
    t.extend_from_slice(&[0xC7, 0x00]);
    u(&mut t, c.id); // mov [eax],id
    t.extend_from_slice(&[0x89, 0x48, 0x04]); // mov [eax+4],ecx
    t.extend_from_slice(&[0x89, 0x50, 0x08]); // mov [eax+8],edx
    t.extend_from_slice(&[0x89, 0x58, 0x0C]); // mov [eax+12],ebx
    t.extend_from_slice(&[0x89, 0x70, 0x10]); // mov [eax+16],esi
    t.extend_from_slice(&[0x89, 0x78, 0x14]); // mov [eax+20],edi
    t.extend_from_slice(&[0xC7, 0x40, 0x18]);
    u(&mut t, c.nargs as u32); // mov [eax+24],nargs
    // v2 pointed-to snapshots: copy declared words through pointer args
    // into the log entry BEFORE the stack-arg loop clobbers ecx/edx.
    // v5: words start `at` bytes from the pointer (v4: always 0), and words
    // 8-63 go to the extension snapshot log (same stride, so one constant
    // displacement from eax); at 0 and 8 words this is v4's encoding.
    let ext_snap = s.ext_snap_base.wrapping_sub(s.log_base);
    let mut snap_off = 0usize;
    // v5 fix: ecx and edx are clobbered by the first entry's copy loop
    // (ecx carries the words, edx the pointer), so a later ecx/edx entry
    // reloads the stub-entry spill; v4 read the previous entry's last word
    // or pointer there. The first entry keeps v4's encoding.
    for (ei, sp) in c.snap.iter().enumerate() {
        let (kind, idx) = sp.kind_code();
        match (kind, ei) {
            (0, _) => {
                t.extend_from_slice(&[0x8B, 0x94, 0x24]);
                u(&mut t, arg_base + idx as u32 * 4); // mov edx,[esp+base+idx*4]
            }
            (1, 0) => {
                t.extend_from_slice(&[0x8B, 0xD1]); // mov edx,ecx
            }
            (1, _) => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_ecx); // mov edx,[m_save_ecx]
            }
            (_, 0) => {} // kind 2: pointer already in edx
            _ => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_edx); // mov edx,[m_save_edx]
            }
        }
        for j in 0..sp.n {
            t.extend_from_slice(&[0x8B, 0x8A]);
            u(&mut t, sp.disp(j)); // mov ecx,[edx+at+j*4]
            let dst = match snap::slot(snap_off + j) {
                snap::SnapSlot::Base(off) => off as u32,
                snap::SnapSlot::Ext(off) => ext_snap.wrapping_add(off as u32),
            };
            t.extend_from_slice(&[0x89, 0x88]);
            u(&mut t, dst); // mov [eax+dst],ecx
        }
        snap_off += sp.n;
    }
    // NOTE: disp32 form (modrm 0x80): snap_n at byte 188 exceeds the +127
    // reach of disp8, which would sign-extend into the previous entry.
    t.extend_from_slice(&[0xC7, 0x80]);
    u(&mut t, 188);
    u(&mut t, snap_off as u32); // mov [eax+188],snap_n
    // stack args: ecx scratch (saved above); [esp+base+k*4] -> [eax+28+k*4]
    let nargs = c.nargs.min(LOG_MAXW);
    for k in 0..nargs {
        t.extend_from_slice(&[0x8B, 0x8C, 0x24]);
        u(&mut t, arg_base + k as u32 * 4); // mov ecx,[esp+base+4k]
        t.extend_from_slice(&[0x89, 0x88]);
        u(&mut t, 28 + k as u32 * 4); // mov [eax+28+4k],ecx
    }
    // NOTE: disp32 forms throughout (modrm 0x80/0x88): all four vector
    // slots sit past byte 127, out of disp8 reach.
    // xmm0 at 224 (v2), xmm1 at 240 (v3, lanes r-b03, r-b24), and v5
    // XMM2-XMM7 in the extension xmm log: movlps/movhps [eax+disp],xmmN.
    let ext_xmm = s.ext_xmm_base.wrapping_sub(s.log_base);
    for reg in 0..vecregs::XMM_REGS {
        if !c.xmm.log[reg] {
            continue;
        }
        let disp = match vecregs::slot(reg) {
            vecregs::XmmSlot::Base(off) => off as u32,
            vecregs::XmmSlot::Ext(off) => ext_xmm.wrapping_add(off as u32),
        };
        t.extend_from_slice(&vecregs::log_store(reg, disp));
    }
    // v4: entry-eax logging for eax-argument callees (lane r-s111).
    // Post-transport on the rewrite side (re-spilled above), genuine on
    // the original side. Compared only when checks.call_regs selects "eax".
    // disp32 form: byte 256 is out of disp8 reach.
    if c.eax_from_stack.is_some() {
        t.extend_from_slice(&[0x8B, 0x0D]);
        u(&mut t, s.m_save_eax); // mov ecx,[m_save_eax]
        t.extend_from_slice(&[0x89, 0x88]);
        u(&mut t, 256); // mov [eax+256],ecx
    }
    // logidx++
    t.push(0xFF);
    t.push(0x05);
    u(&mut t, s.m_logidx); // inc [m_logidx]
    // full: scripted out-param writes + return value
    let full_pos = t.len();
    let rel = (full_pos - (jae_pos + 4)) as u32;
    t[jae_pos..jae_pos + 4].copy_from_slice(&rel.to_le_bytes());
    // v3 per-call answer sequences (lane r-b04): this call consumes step
    // idx = min([seqidx], len-1) of the callee's sequence and advances the
    // per-side index, so a callee polled in a loop can answer token, token,
    // ..., NULL. Contracts without "seq" get [(script lo,hi)] with length
    // 1 (filled by trial_body), which is exactly the v2 behavior. eax is
    // preserved across this: the `al` channel below keeps v2's residue
    // semantics bit-identically.
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_seq_idx + c.id * 4); // mov ecx,[seqidx]
    t.extend_from_slice(&[0x3B, 0x0D]);
    u(&mut t, s.m_seq_len + c.id * 4); // cmp ecx,[seqlen]
    t.extend_from_slice(&[0x72, 0x07]); // jb have (skip 7: mov+dec)
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_seq_len + c.id * 4); // mov ecx,[seqlen]
    t.push(0x49); // dec ecx (len >= 1 always, so len-1 >= 0)
    // have: ecx = clamped step
    t.push(0xFF);
    t.push(0x05);
    u(&mut t, s.m_seq_idx + c.id * 4); // inc [seqidx]
    t.extend_from_slice(&[0x89, 0x0D]);
    u(&mut t, s.m_step); // mov [m_step],ecx (writes below clobber ecx)
    // v2 out-param writes: words from the per-callee write buffer stored
    // through the pointer found at the declared stack arg or register.
    for (kind, idx, wstart, n, dst) in &c.writes {
        match *kind {
            1 => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_ecx); // mov edx,[m_save_ecx]
            }
            2 => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_edx); // mov edx,[m_save_edx]
            }
            _ => {
                t.extend_from_slice(&[0x8B, 0x94, 0x24]);
                u(&mut t, arg_base + *idx as u32 * 4); // mov edx,[esp+base+arg*4]
            }
        }
        for j in 0..*n {
            t.extend_from_slice(&[0x8B, 0x0D]);
            u(&mut t, s.m_writebuf + c.id * 64 + (*wstart + j) as u32 * 4);
            t.extend_from_slice(&[0x89, 0x8A]); // mov ecx,[wbuf]; mov [edx+dst+j*4],ecx
            u(&mut t, (*dst as u32).wrapping_add((j * 4) as u32));
        }
    }
    // edx = this call's (lo,hi) step address: seq_tab + id*128 + step*8.
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_step); // mov ecx,[m_step]
    t.extend_from_slice(&[0x8B, 0xD1]); // mov edx,ecx
    t.extend_from_slice(&[0xC1, 0xE2, 0x03]); // shl edx,3
    t.extend_from_slice(&[0x81, 0xC2]);
    u(&mut t, s.m_seq_tab + c.id * 128); // add edx,seqbase
    match c.ret.as_str() {
        "u64" => {
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
            t.extend_from_slice(&[0x83, 0xC2, 0x04]); // add edx,4
            t.extend_from_slice(&[0x8B, 0x12]); // mov edx,[edx]
        }
        "al" => {
            t.extend_from_slice(&[0x25]);
            u(&mut t, 0xFFFFFF00); // and eax,0xffffff00
            // or in the script lo byte via edx scratch (clobberable)
            t.extend_from_slice(&[0x8B, 0x12]); // mov edx,[edx]
            t.extend_from_slice(&[0x81, 0xE2]);
            u(&mut t, 0xFF); // and edx,0xff
            t.extend_from_slice(&[0x09, 0xD0]); // or eax,edx
        }
        "f32xmm0" => {
            t.extend_from_slice(&[0xF3, 0x0F, 0x10, 0x02]); // movss xmm0,[edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx] too
        }
        "f64xmm0" => {
            t.extend_from_slice(&[0x0F, 0x12, 0x02]); // movlps xmm0,[edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx] too
        }
        "f32st0" => {
            t.extend_from_slice(&[0xD9, 0x02]); // fld dword [edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
        "f64st0" => {
            t.extend_from_slice(&[0xDD, 0x02]); // fld qword [edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
        // v4 (lane r-b79): no scripted answer; restore the entry registers
        // the logging above clobbered. For callees the original calls after
        // its return value is already set, such as the CRT security-cookie
        // check, which genuinely preserves eax (and the scratch registers).
        // Any script/seq declared for the callee still advances but its value
        // is ignored.
        "preserve" => {
            t.push(0xA1);
            u(&mut t, s.m_save_eax); // mov eax,[m_save_eax]
            t.extend_from_slice(&[0x8B, 0x0D]);
            u(&mut t, s.m_save_ecx); // mov ecx,[m_save_ecx]
            t.extend_from_slice(&[0x8B, 0x15]);
            u(&mut t, s.m_save_edx); // mov edx,[m_save_edx]
        }
        _ => {
            // u32 default: eax = step lo
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
    }
    // v4 (lane r-s172): optional scratch-register preservation for callees
    // with a scripted answer. The stub spills entry ecx/edx but never
    // restores them, leaving sequence scratch in place; real MSVC callees
    // (e.g. a base constructor) effectively preserve ecx, and callers built
    // on that fault under the stock stub on every trial. edx is NOT restored
    // for u64 returns, where it carries the high word. Default off: existing
    // contracts emit byte-identical stubs.
    if c.preserve {
        t.extend_from_slice(&[0x8B, 0x0D]);
        u(&mut t, s.m_save_ecx); // mov ecx,[m_save_ecx]
        if c.ret != "u64" {
            t.extend_from_slice(&[0x8B, 0x15]);
            u(&mut t, s.m_save_edx); // mov edx,[m_save_edx]
        }
    }
    match tail_pop {
        Some(outer) => {
            // discard the E8 return address, return to the trampoline
            t.extend_from_slice(&[0x83, 0xC4, 0x04]); // add esp,4
            if outer > 0 {
                t.push(0xC2);
                t.push((outer & 0xFF) as u8);
                t.push((outer >> 8) as u8); // ret outer_pop
            } else {
                t.push(0xC3); // ret
            }
        }
        None => {
            // v4 (lane r-s94): "noclean" models callees that take register
            // args (thiscall/fastcall logging) but pop nothing (cdecl
            // cleanup). The stub then returns side-conditionally: plain ret
            // on the original side (the original cleans its own stack) and
            // ret N on the rewrite side (Rust cannot emit caller-cleanup
            // with ECX live, so the stub balances the rewrite's stack).
            // Observed calls are identical on both sides.
            if c.noclean && c.conv.as_str() != "cdecl" {
                t.extend_from_slice(&[0x83, 0x3D]); // cmp dword [m_side],0
                u(&mut t, s.m_side);
                t.push(0x00);
                t.extend_from_slice(&[0x74, 0x03]); // je +3 (skip ret N)
                t.push(0xC2);
                t.push((c.pop_rw & 0xFF) as u8);
                t.push((c.pop_rw >> 8) as u8); // ret N (rewrite side)
                t.push(0xC3); // ret (original side)
            } else if c.pop > 0 {
                t.push(0xC2);
                t.push((c.pop & 0xFF) as u8);
                t.push((c.pop >> 8) as u8); // ret pop
            } else {
                t.push(0xC3); // ret
            }
        }
    }
    t
}

// Read this thread's TEB TLS-slot array pointer via the emitted helper.
fn tls_slots() -> u32 {
    let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(st().tls_helper as usize) };
    f()
}

// Patch an E9 tail-jump site to a recorder stub call. The stub is a tail
// variant that returns directly to the trampoline (see cmd_setup).
fn patch_e9(site_mapped: usize, stub: u32) -> Result<[u8; 8], String> {
    unsafe {
        let op = *(site_mapped as *const u8);
        if op != 0xE9 {
            return Err(format!("site {:#x} is not E9 (op={:#x})", site_mapped, op));
        }
        let mut orig = [0u8; 8];
        std::ptr::copy_nonoverlapping(site_mapped as *const u8, orig.as_mut_ptr(), 5);
        let rel = stub.wrapping_sub((site_mapped + 5) as u32);
        *(site_mapped as *mut u8) = 0xE8;
        std::ptr::write_unaligned((site_mapped + 1) as *mut u32, rel);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, site_mapped as *const c_void, 5);
        Ok(orig)
    }
}

// Patch an E8 call site to a recorder stub. Returns error if not E8.
fn patch_e8(site_mapped: usize, stub: u32) -> Result<[u8; 8], String> {
    unsafe {
        let op = *(site_mapped as *const u8);
        if op != 0xE8 {
            return Err(format!("site {:#x} is not E8 (op={:#x})", site_mapped, op));
        }
        let mut orig = [0u8; 8];
        std::ptr::copy_nonoverlapping(site_mapped as *const u8, orig.as_mut_ptr(), 5);
        let rel = stub.wrapping_sub((site_mapped + 5) as u32);
        *(site_mapped as *mut u8) = 0xE8;
        std::ptr::write_unaligned((site_mapped + 1) as *mut u32, rel);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, site_mapped as *const c_void, 5);
        Ok(orig)
    }
}

// Minimal import directory walk: find the IAT slot for dll!name.
fn find_iat(dll_want: &str, name_want: &str) -> Option<usize> {
    let s = st();
    let exe = &s.exe_bytes;
    let opt = s._opt;
    let idt_rva = u32le(exe, opt + 96 + 8) as usize;
    let idt_sz = u32le(exe, opt + 96 + 12) as usize;
    if idt_rva == 0 {
        return None;
    }
    let rva2mapped = |rva: usize| -> Option<usize> {
        // map via section table recompute
        let nsec = s._nsec;
        let sely = s._sely;
        let headers_len = s._headers_len;
        if rva < headers_len {
            return Some(s.img + rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(s.img + rva);
            }
        }
        None
    };
    let rva2bytes = |rva: usize| -> Option<usize> {
        // offset into file bytes
        let nsec = s._nsec;
        let sely = s._sely;
        let headers_len = s._headers_len;
        if rva < headers_len {
            return Some(rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            let rawptr = u32le(exe, sh + 20) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(rawptr + (rva - vaddr));
            }
        }
        None
    };
    let n = idt_sz / 20;
    for i in 0..n {
        let d = rva2bytes(idt_rva + i * 20)?;
        let ilt = u32le(exe, d) as usize;
        let name_rva = u32le(exe, d + 12) as usize;
        let iat = u32le(exe, d + 16) as usize;
        if name_rva == 0 {
            break;
        }
        let no = rva2bytes(name_rva)?;
        let mut dn = Vec::new();
        let mut q = no;
        while q < exe.len() && exe[q] != 0 {
            dn.push(exe[q]);
            q += 1;
        }
        let dll = String::from_utf8_lossy(&dn).to_ascii_uppercase();
        if dll != dll_want.to_ascii_uppercase() {
            continue;
        }
        // walk ILT/IAT in parallel
        let mut k = 0usize;
        loop {
            let lto = rva2bytes(ilt + k * 4)?;
            if lto + 4 > exe.len() {
                return None;
            }
            let thunk = u32le(exe, lto);
            if thunk == 0 {
                return None;
            }
            if thunk & 0x80000000 == 0 {
                let hn = rva2bytes((thunk & 0x7FFFFFFF) as usize)?;
                let mut nm = Vec::new();
                let mut q = hn + 2;
                while q < exe.len() && exe[q] != 0 {
                    nm.push(exe[q]);
                    q += 1;
                }
                if String::from_utf8_lossy(&nm) == name_want {
                    return rva2mapped(iat + k * 4);
                }
            }
            k += 1;
            if k > 4096 {
                return None;
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Trial execution.
// ---------------------------------------------------------------------------

// v3: deterministic per-word fill with full avalanche (murmur3 fmix32 over
// the mixed key). Every bit of (seed, trial, range, word) affects every
// output bit, so fills genuinely vary between trials, between ranges and
// between words. This replaces the v1/v2 per-trial xorshift stream seeded
// `(trial ^ seed*G ^ C)|1`, whose `|1` erased the bit consecutive trials
// differed in, making trial pairs share bit-identical first words.
fn fill_word(seed: u32, trial: u32, range: u32, word: u32) -> u32 {
    let mut h = seed
        ^ trial.wrapping_mul(0x85EBCA6B)
        ^ range.wrapping_mul(0xC2B28077)
        ^ word.wrapping_mul(0x27D4EB2F);
    h ^= 16;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B28077);
    h ^= h >> 16;
    h
}

fn heap_pattern(i: u32, trial: u32, seed: u32) -> u32 {
    let mut h = i.wrapping_mul(0x9E3779B1) ^ 0xC6C6C6C6;
    h ^= trial.wrapping_mul(0x85EBCA6B).wrapping_add(0x3C3C3C3C);
    h ^= seed.wrapping_mul(0xC2B28077);
    if h == 0 {
        h = 0xA5A5A5A5;
    }
    h
}

fn fnv1a(mut h: u64, off: usize, val: u32) -> u64 {
    for b in (off as u64)
        .to_le_bytes()
        .iter()
        .chain(val.to_le_bytes().iter())
    {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001B3);
    }
    h
}

type CallRec = (
    u32,
    u32,
    u32,
    Vec<u32>,
    Vec<u32>,
    [u32; 4],
    [u32; 4],
    u32,                  // v4: stub-entry eax (post-transport on the rw side)
    Vec<(u8, [u32; 4])>, // v5: logged XMM2-XMM7 as (register, words)
);

#[derive(Clone, Default)]
struct Obs {
    status: String, // ok | fault | cheat
    regs: [u32; 8], // eax ecx edx ebx esi edi ebp esp
    eflags: u32,
    _mxcsr: u32,
    x87: Option<x87::X87State>, // v5: x87 state on a normal return
    st0: String,
    xmm0: String,
    esp_delta: i64,
    heap_n: u32,
    heap_writes: Vec<(usize, u32)>,
    heap_hash: u64,
    heap_chash: u64, // v4: hash over NaN-canonicalized writes (diagnostic only)
    stack_n: u32,
    stack_writes: Vec<(usize, u32)>,
    stack_hash: u64,
    stack_chash: u64, // v4: hash over NaN-canonicalized writes (diagnostic only)
    globals_writes: Vec<(u32, u32)>, // (rva, val)
    undeclared: Vec<(u32, u32)>,
    undeclared_n: u32,
    calls: Vec<CallRec>, // (id, ecx, edx, args, snap, xmm0, xmm1, eax, xmm2-7)
    log_attempted: u32,  // v4: stub entries this side (sum of per-callee seqidx)
    log_logged: u32,     // v4: records actually logged (min(logidx, log_max))
    fault: String,       // "" or "code=.. eip=.. ..."
    fault_code: u32,
    fault_eip: u32,
    fault_badva: u32,
}

fn obs_json(o: &Obs) -> String {
    let jw = |v: &Vec<(usize, u32)>| -> String {
        v.iter()
            .map(|(x, y)| format!("[{},\"0x{:x}\"]", x, y))
            .collect::<Vec<_>>()
            .join(",")
    };
    let jg = |v: &Vec<(u32, u32)>| -> String {
        v.iter()
            .map(|(x, y)| format!("[\"0x{:x}\",\"0x{:x}\"]", x, y))
            .collect::<Vec<_>>()
            .join(",")
    };
    let jc = o
        .calls
        .iter()
        .map(|(id, cx, dx, a, snap, x0, x1, ax, xe)| {
            // v5: logged XMM2-XMM7 appear only when a callee logs them, so
            // v4 responses are unchanged.
            let xmore = xe
                .iter()
                .map(|(r, w)| {
                    format!(
                        ",\"xmm{}\":[{}]",
                        r,
                        w.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(",")
                    )
                })
                .collect::<String>();
            format!(
                "{{\"id\":{},\"ecx\":{},\"edx\":{},\"eax\":{},\"args\":[{}],\"snap\":[{}],\"xmm0\":[{}],\"xmm1\":[{}]{}}}",
                id,
                hx(*cx),
                hx(*dx),
                hx(*ax),
                a.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                snap.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                x0.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                x1.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                xmore
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    // v5: the x87 state (stack top, tag byte, valid registers in stack
    // order) appears only on a normal return.
    let x87j = match &o.x87 {
        Some(x) => format!(
            ",\"x87\":{{\"top\":{},\"tags\":{},\"st\":[{}]}}",
            x.top,
            x.tags,
            (0..8)
                .filter(|&i| x.valid(i))
                .map(|i| format!("\"{}\"", x.regs[i].hex()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        None => String::new(),
    };
    format!(
        "{{\"status\":\"{}\",\"eax\":{},\"ecx\":{},\"edx\":{},\"ebx\":{},\"esi\":{},\"edi\":{},\"ebp\":{},\"esp\":{},\"eflags\":{},\"st0\":\"{}\",\"xmm0\":\"{}\",\"esp_delta\":{},\"log_attempted\":{},\"log_logged\":{},\"heap_n\":{},\"heap_writes\":[{}],\"heap_hash\":\"0x{:x}\",\"stack_n\":{},\"stack_writes\":[{}],\"stack_hash\":\"0x{:x}\",\"globals_writes\":[{}],\"undeclared\":[{}],\"undeclared_n\":{},\"calls\":[{}],\"fault\":\"{}\",\"fault_code\":{},\"fault_eip\":{},\"fault_badva\":{}{}}}",
        o.status,
        hx(o.regs[0]),
        hx(o.regs[1]),
        hx(o.regs[2]),
        hx(o.regs[3]),
        hx(o.regs[4]),
        hx(o.regs[5]),
        hx(o.regs[6]),
        hx(o.regs[7]),
        hx(o.eflags),
        o.st0,
        o.xmm0,
        o.esp_delta,
        o.log_attempted,
        o.log_logged,
        o.heap_n,
        jw(&o.heap_writes),
        o.heap_hash,
        o.stack_n,
        jw(&o.stack_writes),
        o.stack_hash,
        jg(&o.globals_writes),
        jg(&o.undeclared),
        o.undeclared_n,
        jc,
        esc(&o.fault),
        hx(o.fault_code),
        hx(o.fault_eip),
        hx(o.fault_badva),
        x87j
    )
}

// Snapshot data ranges back to pristine, then apply a globals fill.
fn restore_data() {
    let s = st();
    let mut off = 0usize;
    for &(lo, hi) in &s.data_ranges.clone() {
        let len = hi - lo;
        unsafe {
            std::ptr::copy_nonoverlapping(s.pristine.as_ptr().add(off), lo as *mut u8, len);
        }
        off += len;
    }
}

fn in_declared(mapped: usize) -> bool {
    let s = st();
    for &(lo, len) in &s.globals {
        if lo <= mapped && mapped < lo + len {
            return true;
        }
    }
    false
}

// The trial parameters are a flat physical interface (registers, stack,
// heap, globals, TLS, XMM); bundling them would only hide the protocol.
#[allow(clippy::too_many_arguments)]
fn run_side(
    fn_addr: u32,
    regs: &[u32; 7], // eax ecx edx ebx esi edi ebp
    stack_args: &[u32],
    heapsegs: &[(usize, Vec<u32>)],
    globals_fill: &[(usize, Vec<u32>)], // mapped addr + words
    trial: u32,
    seed: u32,
    tls: &[(u32, u32)], // (slot, value) fabricated TLS slots
    xmm: &[u32; 32],    // xmm0-7 entry values
    x87_in: &[x87::F80], // v5: x87 entry values, ST(0) first
    stack_fill: Option<u32>,
    is_rw: bool,
    fulldata: bool,
) -> Obs {
    let s = st();
    let h = s.h;
    let sb = s.s;
    let esp0 = s.esp0;
    // 1. fill heap with trial pattern
    unsafe {
        let hp = h as *mut u32;
        for i in 0..(HEAP_USE / 4) {
            *hp.add(i) = heap_pattern(i as u32, trial, seed);
        }
        // stack snapshot window: trial pattern, or the contract's defined fill
        let sp = (sb + 0xFC000) as *mut u32;
        for i in 0..(STACK_SNAP_LEN / 4) {
            *sp.add(i) = match stack_fill {
                Some(v) => v,
                None => (i as u32).wrapping_mul(0x85EBCA6B) ^ 0x3C3C3C3C ^ trial,
            };
        }
        // v2: reset the below-ESP scratch (alloca-style regions) identically
        // on both sides, so stub-shaped buffers do not read cross-side garbage.
        let bp = (sb + SCRATCH_FILL_OFF) as *mut u32;
        for i in 0..(SCRATCH_FILL_LEN / 4) {
            *bp.add(i) = match stack_fill {
                Some(v) => v,
                None => {
                    let ai = ((SCRATCH_FILL_OFF / 4) + i) as u32;
                    ai.wrapping_mul(0x85EBCA6B) ^ 0x3C3C3C3C ^ trial
                }
            };
        }
        // heap segments
        for (off, words) in heapsegs {
            for (k, w) in words.iter().enumerate() {
                *((h + off + k * 4) as *mut u32) = *w;
            }
        }
        // stack args at esp0
        let ap = esp0 as *mut u32;
        for (i, &a) in stack_args.iter().enumerate() {
            *ap.add(i) = a;
        }
        // snapshot stack window BEFORE
        std::ptr::copy_nonoverlapping(
            (sb + STACK_SNAP_OFF) as *const u32,
            s.before_stack.as_mut_ptr(),
            STACK_SNAP_LEN / 4,
        );
        // fault flag + log + side marker (script tables are filled once
        // per trial in trial_body, shared by both sides)
        std::ptr::write_bytes(s.m_fault as *mut u8, 0, 56);
        *((s.m_logidx) as *mut u32) = 0;
        // v3: each side consumes per-call sequences from step 0.
        std::ptr::write_bytes(s.m_seq_idx as *mut u8, 0, 1024);
        *((s.m_side) as *mut u32) = if is_rw { 1 } else { 0 };
        *((s.m_fn) as *mut u32) = fn_addr;
        // ctx inputs: ecx edx ebx esi edi ebp eax esp0
        s.ctx[0] = regs[1];
        s.ctx[1] = regs[2];
        s.ctx[2] = regs[3];
        s.ctx[3] = regs[4];
        s.ctx[4] = regs[5];
        s.ctx[5] = regs[6];
        s.ctx[6] = regs[0];
        s.ctx[7] = esp0;
        // v2: scripted XMM entry values (+ a mirror the rewrite can read,
        // since Rust cannot observe incoming vector registers any other way)
        for (i, &w) in xmm.iter().enumerate() {
            s.ctx[145 + i] = w;
            *((s.m_xmm_mirror + i as u32 * 4) as *mut u32) = w;
        }
        // v5: x87 entry values. The original gets them on its FPU stack
        // (the trampoline loads ctx[CTX_X87_N] of them); the rewrite starts
        // with an empty stack and reads the mirror, which holds the same
        // values on both sides.
        s.ctx[CTX_X87_N] = if is_rw { 0 } else { x87_in.len() as u32 };
        std::ptr::write_bytes(s.m_x87_mirror as *mut u8, 0, X87_MIRROR_LEN);
        *(s.m_x87_mirror as *mut u32) = x87_in.len() as u32;
        for (i, v) in x87_in.iter().enumerate() {
            let b = v.to_bytes();
            std::ptr::copy_nonoverlapping(
                b.as_ptr(),
                (s.m_x87_mirror as usize + 16 + 16 * i) as *mut u8,
                b.len(),
            );
            let w = [
                v.man as u32,
                (v.man >> 32) as u32,
                u32::from(v.sexp),
            ];
            s.ctx[CTX_X87_VALS + 3 * i..CTX_X87_VALS + 3 * i + 3].copy_from_slice(&w);
        }
        *((s.m_ctx) as *mut u32) = s.ctx.as_ptr() as u32;
    }
    // v2: fabricated TLS slots on this trial thread + a mirror the
    // rewrite can read, since Rust cannot read FS. The mirror is
    // worker-shared, so it is zeroed on every side. The TEB slots are
    // saved and restored around each side: low slots may belong to a
    // loaded module (thread teardown reads them; leaving a heap pointer
    // behind killed the worker).
    unsafe {
        std::ptr::write_bytes(s.m_tls_mirror as *mut u8, 0, 1024);
    }
    // NOTE: faults before the trampoline's hostesp save (this plant
    // included) resume with esp=0 and die by nested fault; keep this
    // sequence to mapped-memory operations only (TEB slots + mirror are).
    let mut tls_save: Vec<(u32, u32)> = Vec::new();
    if !tls.is_empty() {
        let slots = tls_slots();
        for (slot, val) in tls {
            unsafe {
                let p = (slots + slot * 4) as *mut u32;
                tls_save.push((*slot, *p));
                *p = *val;
                if *slot < 256 {
                    *((s.m_tls_mirror + slot * 4) as *mut u32) = *val;
                }
            }
        }
    }
    // 2. data ranges: pristine + declared fill
    let need_data = fulldata || !globals_fill.is_empty() || !st().globals.is_empty();
    if need_data {
        restore_data();
    }
    // v2: fills outside the writable sections (.rdata hook slots) are
    // captured and restored around the call so they never leak across trials.
    let mut rda_restores: Vec<(usize, u32)> = Vec::new();
    for (addr, words) in globals_fill {
        for (k, w) in words.iter().enumerate() {
            let a = addr + k * 4;
            let in_writable = st().data_ranges.iter().any(|(lo, hi)| *lo <= a && a < *hi);
            unsafe {
                if !in_writable && !rda_restores.iter().any(|(x, _)| *x == a) {
                    rda_restores.push((a, *(a as *const u32)));
                }
                *(a as *mut u32) = *w;
            }
        }
    }
    // snapshot declared ranges AFTER fill (diffs are callee-only)
    let mut gbefore: Vec<(usize, Vec<u32>)> = Vec::new();
    for &(lo, len) in &st().globals {
        let mut v = Vec::with_capacity(len / 4);
        for k in 0..(len / 4) {
            v.push(unsafe { *((lo + k * 4) as *const u32) });
        }
        gbefore.push((lo, v));
    }
    // full-data snapshot after fill for undeclared-write discovery
    let mut dbefore: Vec<u8> = Vec::new();
    if fulldata {
        for &(lo, hi) in &st().data_ranges {
            dbefore
                .extend_from_slice(unsafe { std::slice::from_raw_parts(lo as *const u8, hi - lo) });
        }
    }
    // v5: the read-only shadow is readable only while the original of an
    // abs_shadow contract runs (no-op unless the protection changes).
    abs_set_readable(s.abs.active && !is_rw);
    // 3. anti-cheat: revoke .text while the rewrite runs
    s.side_rw = is_rw;
    if is_rw {
        protect(s.text_lo, s.text_hi - s.text_lo, PAGE_NOACCESS);
    }
    // 4. call
    let tramp_fn: extern "C" fn() = unsafe { std::mem::transmute(st().tramp) };
    tramp_fn();
    for (slot, old) in &tls_save {
        unsafe {
            *((tls_slots() + slot * 4) as *mut u32) = *old;
        }
    }
    if is_rw {
        let s = st();
        protect(s.text_lo, s.text_hi - s.text_lo, PAGE_RWX);
        s.side_rw = false;
    }
    for (a, v) in &rda_restores {
        unsafe {
            *(*a as *mut u32) = *v;
        }
    }
    // 5. collect
    let s = st();
    let mut o = Obs::default();
    let faulted = unsafe { *((s.m_fault) as *const u32) != 0 };
    if faulted {
        let f = unsafe { std::slice::from_raw_parts(s.m_fault as *const u32, 14) };
        o.fault_code = f[1];
        o.fault_eip = f[2];
        let acc = f[3];
        o.fault_badva = f[4];
        o.regs = [f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12]];
        o.eflags = f[13];
        o.esp_delta = (o.regs[7] as i64) - (esp0 as i64);
        o.fault = format!(
            "code=0x{:x} eip=0x{:x} access={} badva=0x{:x}",
            o.fault_code, o.fault_eip, acc, o.fault_badva
        );
        // cheat? rewrite touching original code pages
        let in_text = |a: u32| (s.text_lo as u32) <= a && a < (s.text_hi as u32);
        if is_rw && (in_text(o.fault_eip) || in_text(o.fault_badva)) {
            o.status = "cheat".to_string();
        } else {
            o.status = "fault".to_string();
        }
        // v5: a fault in the preferred-base window is an unrelocated
        // absolute access; say so (diagnostic text only, never compared).
        if s.abs.lo < s.abs.hi
            && let Some(note) =
                abswin::fault_note(o.fault_badva as usize, s.abs.lo, s.abs.hi, is_rw, s.abs.active)
        {
            o.fault.push(' ');
            o.fault.push_str(&note);
        }
        o.st0 = String::new();
        o.xmm0 = String::new();
    } else {
        o.status = "ok".to_string();
        o.regs = [
            s.ctx[8], s.ctx[9], s.ctx[10], s.ctx[11], s.ctx[12], s.ctx[13], s.ctx[14], s.ctx[15],
        ];
        o.eflags = s.ctx[16];
        o.esp_delta = (o.regs[7] as i64) - (esp0 as i64);
        let fx = &s.ctx[17..17 + 128];
        let fxb = unsafe { std::slice::from_raw_parts(fx.as_ptr() as *const u8, 512) };
        o.st0 = hexbytes(&fxb[32..42]);
        o.xmm0 = hexbytes(&fxb[160..176]);
        o._mxcsr = u32::from_le_bytes([fxb[24], fxb[25], fxb[26], fxb[27]]);
        o.x87 = x87::X87State::from_fxsave(fxb);
    }
    // heap diff
    let mut hh = 0xcbf29ce484222325u64;
    let mut hc = 0xcbf29ce484222325u64; // v4: canonical-NaN hash (diagnostic only)
    unsafe {
        let hp = h as *const u32;
        for i in 0..(HEAP_USE / 4) {
            // expected = pattern, unless a segment overrode it
            let mut expect = heap_pattern(i as u32, trial, seed);
            let byte = i * 4;
            for (off, words) in heapsegs {
                if *off <= byte && byte < off + words.len() * 4 {
                    expect = words[(byte - off) / 4];
                    break;
                }
            }
            let got = *hp.add(i);
            if got != expect {
                o.heap_n += 1;
                hh = fnv1a(hh, byte, got);
                hc = fnv1a(hc, byte, canon_f32(got));
                if o.heap_writes.len() < 64 {
                    o.heap_writes.push((byte, got));
                }
            }
        }
    }
    o.heap_hash = hh;
    o.heap_chash = hc;
    // stack diff
    let mut sh = 0xcbf29ce484222325u64;
    let mut sc = 0xcbf29ce484222325u64; // v4: canonical-NaN hash (diagnostic only)
    let esp_after = o.regs[7] as usize;
    unsafe {
        let sp = (sb + STACK_SNAP_OFF) as *const u32;
        let sb0 = sb + STACK_SNAP_OFF;
        for i in 0..s.before_stack.len() {
            let addr = sb0 + i * 4;
            if (addr as u32) < esp0 {
                continue; // below incoming ESP: callee's own frame, not observable
            }
            if !faulted && addr == esp_after.wrapping_sub(4) {
                continue; // trampoline's own pushfd slot
            }
            // args area was written before snapshot, so diffs are callee-only;
            // but skip the exact arg slots only if unchanged... no: include all diffs
            let got = *sp.add(i);
            if got != s.before_stack[i] {
                // ignore slots that are exactly our own pre-placed stack args
                let rel = addr as u32 as i64 - esp0 as i64;
                if rel >= 0 && (rel as usize) < stack_args.len() * 4 {
                    let want = stack_args[rel as usize / 4];
                    if got == want {
                        continue;
                    }
                }
                o.stack_n += 1;
                sh = fnv1a(sh, STACK_SNAP_OFF + i * 4, got);
                sc = fnv1a(sc, STACK_SNAP_OFF + i * 4, canon_f32(got));
                if o.stack_writes.len() < 64 {
                    o.stack_writes.push((STACK_SNAP_OFF + i * 4, got));
                }
            }
        }
    }
    o.stack_hash = sh;
    o.stack_chash = sc;
    // declared globals diff + full-data undeclared discovery
    for (gi, &(lo, len)) in st().globals.clone().iter().enumerate() {
        for k in 0..(len / 4) {
            let got = unsafe { *((lo + k * 4) as *const u32) };
            if got != gbefore[gi].1[k] {
                o.globals_writes.push(((lo - st().img) as u32, got));
            }
        }
    }
    if fulldata {
        let s = st();
        let img = s.img;
        let mut off = 0usize;
        for &(lo, hi) in &s.data_ranges.clone() {
            let len = hi - lo;
            let now = unsafe { std::slice::from_raw_parts(lo as *const u8, len) };
            let bef = &dbefore[off..off + len];
            // word-wise compare
            let nw = len / 4;
            for k in 0..nw {
                let a = u32::from_le_bytes([
                    now[k * 4],
                    now[k * 4 + 1],
                    now[k * 4 + 2],
                    now[k * 4 + 3],
                ]);
                let b = u32::from_le_bytes([
                    bef[k * 4],
                    bef[k * 4 + 1],
                    bef[k * 4 + 2],
                    bef[k * 4 + 3],
                ]);
                if a != b && !in_declared(lo + k * 4) {
                    o.undeclared_n += 1;
                    if o.undeclared.len() < 64 {
                        o.undeclared.push(((lo + k * 4 - img) as u32, a));
                    }
                }
            }
            off += len;
        }
    }
    // call log
    {
        let s = st();
        let cap = s.log_max;
        let n = unsafe { *((s.m_logidx) as *const u32) }.min(cap);
        o.log_logged = n;
        // v4: attempted calls = sum of the per-callee sequence indexes. The
        // stub advances one index per entry (logged or dropped) and run_side
        // zeroes the table per side, so attempted > logged means the log
        // filled and calls went uncompared: the calls check must fail.
        let mut attempted = 0u32;
        unsafe {
            let tab = s.m_seq_idx as *const u32;
            for i in 0..256 {
                attempted = attempted.wrapping_add(*tab.add(i));
            }
        }
        o.log_attempted = attempted;
        for i in 0..n {
            let e = (s.log_base + i * LOG_ENTRY as u32) as *const u32;
            unsafe {
                let id = *e;
                let cx = *e.add(1);
                let dx = *e.add(2);
                let nargs = (*e.add(6)).min(LOG_MAXW as u32);
                let mut args = Vec::new();
                for k in 0..nargs {
                    args.push(*e.add(7 + k as usize));
                }
                let snapn = (*e.add(47)).min(SNAP_MAXW as u32);
                // v5: words 8 and up come from the extension snapshot log.
                let es = (s.ext_snap_base + i * LOG_ENTRY as u32) as *const u32;
                let mut snap = Vec::new();
                for k in 0..snapn as usize {
                    snap.push(match snap::slot(k) {
                        snap::SnapSlot::Base(off) => *e.add(off / 4),
                        snap::SnapSlot::Ext(off) => *es.add(off / 4),
                    });
                }
                let mut x0 = [0u32; 4];
                for (k, slot) in x0.iter_mut().enumerate() {
                    *slot = *e.add(56 + k);
                }
                let mut x1 = [0u32; 4];
                for (k, slot) in x1.iter_mut().enumerate() {
                    *slot = *e.add(60 + k);
                }
                // v4: entry eax at byte 256, meaningful only for callees
                // with eax_from_stack (other stubs never store it, so the
                // slot would show a stale record: report 0 instead).
                let ax = match s.callees.get(&id) {
                    Some(c) if c.eax_from_stack.is_some() => *e.add(64),
                    _ => 0,
                };
                // v5: XMM2-XMM7 the callee logs, from the extension xmm log.
                let ex = (s.ext_xmm_base + i * LOG_ENTRY as u32) as *const u32;
                let mut xe = Vec::new();
                if let Some(c) = s.callees.get(&id) {
                    for reg in c.xmm.ext_logged() {
                        if let vecregs::XmmSlot::Ext(off) = vecregs::slot(reg) {
                            let mut w = [0u32; 4];
                            for (k, slot) in w.iter_mut().enumerate() {
                                *slot = *ex.add(off / 4 + k);
                            }
                            xe.push((reg as u8, w));
                        }
                    }
                }
                o.calls.push((id, cx, dx, args, snap, x0, x1, ax, xe));
            }
        }
    }
    o
}

// Compare two observations per the checks object. Returns (pass, checks_json, first_mismatch).
fn compare(a: &Obs, b: &Obs, checks: &J) -> (bool, String, String) {
    let (pass, json, first) = compare_inner(a, b, checks);
    if pass {
        return (true, json, first);
    }
    // v4 NaN diagnostic (lane r-b91's case, NOT their fix: all-NaN-equal is
    // unsound and stays out). If the trial fails but passes with every NaN
    // canonicalized (sign/payload erased), the only difference is NaN bits
    // and the verdict says so, pointing at operand order. The trial still
    // fails; this is text only, never a pass.
    let ca = canon_nan_obs(a);
    let cb = canon_nan_obs(b);
    if compare_inner(&ca, &cb, checks).0 {
        let diag = "[nan-diagnostic: sides differ ONLY in NaN sign/payload bits; check operand order in the rewrite]";
        let first = if first.is_empty() {
            diag.to_string()
        } else {
            format!("{} {}", first, diag)
        };
        return (false, json, first);
    }
    (false, json, first)
}

fn is_f32_nan(v: u32) -> bool {
    (v & 0x7F800000) == 0x7F800000 && (v & 0x007FFFFF) != 0
}

fn canon_f32(v: u32) -> u32 {
    if is_f32_nan(v) { 0x7FC00000 } else { v }
}

// Canonicalize every float-comparable word for the NaN diagnostic recheck:
// call args, snapshots and call vector words (f32 lanes), the st0/xmm0
// return channels, pointed-to globals, and the heap/stack hashes (via the
// canonical hashes run_side maintains over the full write stream, since the
// kept write prefixes are truncated and cannot be re-hashed).
fn canon_nan_obs(o: &Obs) -> Obs {
    let mut c = o.clone();
    for rec in c.calls.iter_mut() {
        for w in rec.3.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.4.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.5.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.6.iter_mut() {
            *w = canon_f32(*w);
        }
        rec.7 = canon_f32(rec.7);
        for (_, w4) in rec.8.iter_mut() {
            for w in w4.iter_mut() {
                *w = canon_f32(*w);
            }
        }
    }
    c.st0 = canon_st0_hex(&c.st0);
    c.xmm0 = canon_xmm0_hex(&c.xmm0);
    for (_, v) in c.globals_writes.iter_mut() {
        *v = canon_f32(*v);
    }
    c.heap_hash = c.heap_chash;
    c.stack_hash = c.stack_chash;
    c
}

fn unhex(h: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let b = h.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        let hi = (b[i] as char).to_digit(16).unwrap_or(0);
        let lo = (b[i + 1] as char).to_digit(16).unwrap_or(0);
        v.push((hi * 16 + lo) as u8);
        i += 2;
    }
    v
}

fn canon_st0_hex(h: &str) -> String {
    let b = unhex(h);
    if b.len() < 10 {
        return h.to_string();
    }
    let man = u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]);
    let ex = u16::from_le_bytes([b[8], b[9]]);
    if (ex & 0x7FFF) == 0x7FFF && (man & 0x7FFFFFFFFFFFFFFF) != 0 {
        "00000000000000c0ff7f".to_string() // +qNaN, zero payload
    } else {
        h.to_string()
    }
}

fn canon_xmm0_hex(h: &str) -> String {
    let b = unhex(h);
    if b.len() < 16 {
        return h.to_string();
    }
    let mut b = b;
    let w0 = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    if is_f32_nan(w0) {
        b[0..4].copy_from_slice(&0x7FC00000u32.to_le_bytes());
    }
    let d0 = u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]);
    if (d0 & 0x7FF0000000000000) == 0x7FF0000000000000
        && (d0 & 0x000FFFFFFFFFFFFF) != 0
    {
        b[0..8].copy_from_slice(&0x7FF8000000000000u64.to_le_bytes());
    }
    hexbytes(&b)
}

fn compare_inner(a: &Obs, b: &Obs, checks: &J) -> (bool, String, String) {
    let mut parts: Vec<String> = Vec::new();
    let mut first = String::new();
    let mut ok_all = true;
    let mut chk = |name: &str, pass: bool, detail: String| {
        if !pass {
            ok_all = false;
            if first.is_empty() {
                first = format!("{}: {}", name, detail);
            }
        }
        parts.push(format!(
            "{{\"name\":\"{}\",\"passed\":{},\"detail\":\"{}\"}}",
            name,
            if pass { "true" } else { "false" },
            esc(&detail)
        ));
    };
    let want = |k: &str, d: bool| checks.get(k).map(|v| v.as_bool(d)).unwrap_or(d);
    // termination
    if want("fault", true) {
        let same = a.status == b.status || (a.status == "ok" && b.status == "ok");
        // fault codes must match too when both fault
        let pass = if a.status == "fault" && b.status == "fault" {
            a.fault_code == b.fault_code
        } else {
            same
        };
        chk(
            "termination",
            pass,
            format!("orig={} rw={} {}", a.status, b.status, b.fault),
        );
    }
    // cheat is always a failure for the rewrite side
    if b.status == "cheat" {
        chk(
            "no_cheat",
            false,
            format!("rewrite touched original code: {}", b.fault),
        );
    } else {
        chk("no_cheat", true, String::new());
    }
    if a.status != "ok" || b.status != "ok" {
        // fault-vs-fault with same code passes termination; nothing else to compare
        return (ok_all, format!("[{}]", parts.join(",")), first);
    }
    // return value
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    match ret {
        "none" => chk("ret", true, String::new()),
        "al" => {
            let p = (a.regs[0] & 0xFF) == (b.regs[0] & 0xFF);
            chk(
                "ret",
                p,
                format!(
                    "al orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFF,
                    b.regs[0] & 0xFF
                ),
            );
        }
        "ax" => {
            let p = (a.regs[0] & 0xFFFF) == (b.regs[0] & 0xFFFF);
            chk(
                "ret",
                p,
                format!(
                    "ax orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFFFF,
                    b.regs[0] & 0xFFFF
                ),
            );
        }
        "edx_eax" => {
            let p = a.regs[0] == b.regs[0] && a.regs[2] == b.regs[2];
            chk(
                "ret",
                p,
                format!(
                    "edx:eax orig=0x{:x}:0x{:x} rw=0x{:x}:0x{:x}",
                    a.regs[2], a.regs[0], b.regs[2], b.regs[0]
                ),
            );
        }
        "st0" => {
            let (p, d, _exact) = fp_cmp(&a.st0, &b.st0, 10, checks);
            chk("ret", p, format!("st0 {}", d));
        }
        "xmm0" => {
            let (p, d, _exact) = fp_cmp(&a.xmm0, &b.xmm0, 16, checks);
            chk("ret", p, format!("xmm0 {}", d));
        }
        _ => {
            let p = a.regs[0] == b.regs[0];
            chk(
                "ret",
                p,
                format!("eax orig=0x{:x} rw=0x{:x}", a.regs[0], b.regs[0]),
            );
        }
    }
    if want("esp", true) {
        let p = a.esp_delta == b.esp_delta;
        chk(
            "esp",
            p,
            format!("delta orig={} rw={}", a.esp_delta, b.esp_delta),
        );
    }
    if want("heap", true) {
        let p = a.heap_n == b.heap_n && a.heap_hash == b.heap_hash;
        let mut d = format!("n orig={} rw={}", a.heap_n, b.heap_n);
        if !p {
            d.push_str(&format!(
                " orig_first={:?} rw_first={:?}",
                a.heap_writes.iter().take(4).collect::<Vec<_>>(),
                b.heap_writes.iter().take(4).collect::<Vec<_>>()
            ));
        }
        chk("heap", p, d);
    }
    if want("stack", true) {
        let p = a.stack_n == b.stack_n && a.stack_hash == b.stack_hash;
        chk("stack", p, format!("n orig={} rw={}", a.stack_n, b.stack_n));
    }
    if want("globals", true) {
        let p = a.globals_writes == b.globals_writes;
        chk(
            "globals",
            p,
            format!(
                "orig={:?} rw={:?}",
                lim(&a.globals_writes, 4),
                lim(&b.globals_writes, 4)
            ),
        );
    }
    if want("calls", true) {
        // Only the callee's real inputs participate: id + declared reg args
        // (cdecl/stdcall: none; thiscall: ecx; fastcall: ecx+edx; override
        // via checks.call_regs {id:[regs]}). Scratch regs at the call site
        // differ legitimately between original and rewrite codegen.
        // v4 fail-closed validations run before the key comparison: a zero
        // mask, an eax reg without its transport, or a filled call log all
        // mean calls went uncompared, so the check fails loudly.
        if let Some(z) = find_zero_mask(checks) {
            chk("calls", false, z);
        } else if let Some(e) = validate_eax_regs(checks) {
            chk("calls", false, e);
        } else if let Some(e) = validate_eax_transport(checks) {
            chk("calls", false, e);
        } else if a.log_attempted > a.log_logged || b.log_attempted > b.log_logged {
            chk(
                "calls",
                false,
                format!(
                    "call log truncated: orig attempted {} logged {}; rw attempted {} logged {} (cap {}; raise top-level log_max, max {})",
                    a.log_attempted,
                    a.log_logged,
                    b.log_attempted,
                    b.log_logged,
                    st().log_max,
                    LOG_HARD_MAX
                ),
            );
        } else {
            let ka: Vec<String> = a.calls.iter().map(|c| callkey(c, checks)).collect();
            let kb: Vec<String> = b.calls.iter().map(|c| callkey(c, checks)).collect();
            let p = ka == kb;
            chk(
                "calls",
                p,
                format!("orig={:?} rw={:?}", lims(&ka, 3), lims(&kb, 3)),
            );
        }
    }
    if want("undeclared", true) {
        let p = a.undeclared_n == 0 && b.undeclared_n == 0;
        chk(
            "undeclared",
            p,
            format!(
                "orig_n={} rw_n={} rw_first={:?}",
                a.undeclared_n,
                b.undeclared_n,
                lim(&b.undeclared, 4)
            ),
        );
    }
    // v5: x87 state (stack top, tag byte, valid registers). On when the
    // contract asks for it or the trial loads x87 entry values (parse_trial
    // sets the flag then); absent otherwise, so v4 check lists are unchanged.
    if want("x87_state", false) {
        match (&a.x87, &b.x87) {
            (Some(x), Some(y)) => match x87::compare_x87(x, y) {
                Ok(()) => chk("x87", true, String::new()),
                Err(d) => chk("x87", false, d),
            },
            _ => chk("x87", false, "x87 state not captured".to_string()),
        }
    }
    (ok_all, format!("[{}]", parts.join(",")), first)
}

// v2 pointer normalization: an address inside one of the worker's fixed
// windows compares as a region-relative offset, so a pointer to the
// function's own frame, the trial heap or the image matches when it refers
// to the same place on both sides even though raw addresses differ.
fn norm_ptr(v: u32) -> String {
    let s = st();
    let h = s.h as u32;
    if h <= v && v < h.wrapping_add(HEAP_USE as u32) {
        return format!("H+{:x}", v - h);
    }
    let sb = s.s as u32;
    if sb <= v && v < sb.wrapping_add(SSIZE as u32) {
        if v >= s.esp0 {
            return format!("S+{:x}", v - s.esp0);
        } else {
            return format!("S-{:x}", s.esp0 - v);
        }
    }
    let img = s.img as u32;
    if img <= v && v < img.wrapping_add(s.image_size as u32) {
        return format!("I+{:x}", v - img);
    }
    format!("0x{:x}", v)
}

// v4: a zero mask compares nothing (every value masks to 0), so it is
// rejected instead of comparing. Returns the failure detail, if any.
fn find_zero_mask(checks: &J) -> Option<String> {
    let m = checks.get("call_mask")?;
    let obj = match m {
        J::Obj(o) => o,
        _ => return None,
    };
    for (id, per) in obj {
        let per = match per {
            J::Obj(o) => o,
            _ => continue,
        };
        for (idx, mask) in per {
            if mask.as_u32() == 0 {
                return Some(format!(
                    "call_mask {}.{} is zero (rejected: a zero mask compares nothing)",
                    id, idx
                ));
            }
        }
    }
    None
}

// v4: "eax" in checks.call_regs is only meaningful with the callee's
// eax_from_stack transport (a rewrite cannot set eax any other way, and
// stubs without it never log eax). Without it the reg would compare
// stale zeros on both sides and pass vacuously: reject instead.
fn validate_eax_regs(checks: &J) -> Option<String> {
    let m = checks.get("call_regs")?;
    let obj = match m {
        J::Obj(o) => o,
        _ => return None,
    };
    for (id, regs) in obj {
        let wants_eax = regs.as_arr().iter().any(|r| r.as_str() == "eax");
        if !wants_eax {
            continue;
        }
        let cid = id.parse::<u32>().unwrap_or(u32::MAX);
        let ok = st()
            .callees
            .get(&cid)
            .map(|c| c.eax_from_stack.is_some())
            .unwrap_or(false);
        if !ok {
            return Some(format!(
                "call_regs {} selects eax without eax_from_stack transport (rejected: eax would be uncompared)",
                id
            ));
        }
    }
    None
}

// v4 (coordinator review): the reverse must hold as well. A callee with the
// eax transport has its stack arguments left out of the call key, so unless
// call_regs selects "eax" for it the transported argument would be compared
// nowhere. Reject instead of passing with it uncompared.
fn validate_eax_transport(checks: &J) -> Option<String> {
    let mut ids: Vec<u32> = st()
        .callees
        .iter()
        .filter(|(_, c)| c.eax_from_stack.is_some())
        .map(|(id, _)| *id)
        .collect();
    ids.sort();
    for id in ids {
        let selected = checks
            .get("call_regs")
            .and_then(|m| m.get(id.to_string().as_str()))
            .map(|regs| regs.as_arr().iter().any(|r| r.as_str() == "eax"))
            .unwrap_or(false);
        if !selected {
            return Some(format!(
                "callee {} has eax_from_stack but call_regs does not select eax (rejected: the transported argument would be uncompared)",
                id
            ));
        }
    }
    None
}

fn callkey(c: &CallRec, checks: &J) -> String {
    let id = c.0.to_string();
    let cal = st().callees.get(&c.0).cloned().unwrap_or_default();
    let regs: Vec<String> = match checks.get("call_regs").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|r| r.as_str().to_string()).collect(),
        None => match cal.conv.as_str() {
            "thiscall" => vec!["ecx".to_string()],
            "fastcall" => vec!["ecx".to_string(), "edx".to_string()],
            _ => Vec::new(),
        },
    };
    let mut k = format!("id={}", c.0);
    for r in &regs {
        match r.as_str() {
            "ecx" => k.push_str(&format!(" ecx={}", norm_ptr(c.1))),
            "edx" => k.push_str(&format!(" edx={}", norm_ptr(c.2))),
            "eax" => k.push_str(&format!(" eax={}", norm_ptr(c.7))), // v4 (r-s111)
            _ => {}
        }
    }
    // v2: stack args are normalized too; volatile indexes can be dropped
    // with checks.call_skip {id:[idx,...]} (folded from lane q-16). Args of
    // a transport callee are skipped: the original side pushes nothing.
    let skip: Vec<usize> = match checks.get("call_skip").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|x| x.as_usize()).collect(),
        None => Vec::new(),
    };
    if !cal.xmm.any_transport() && cal.eax_from_stack.is_none() {
        // v4 per-argument masks (lanes r-b39, r-n117, r-n86; alias
        // checks.call_low8 from lane r-n118): checks.call_mask
        // {id:{idx:mask}} compares (value & mask) as raw hex instead of the
        // normalized pointer. Use: the original pushes a one-byte value as
        // a full word whose upper bytes are caller leftovers (not
        // behaviour); the mask keeps the meaningful bytes compared exactly,
        // unlike call_skip which drops the arg. A full (all-ones) mask is
        // the default comparison. Zero masks never reach here (rejected
        // above). The alias maps each listed index to 0xFF; an explicit
        // call_mask entry wins over the alias for the same argument.
        let mut masks: HashMap<usize, u32> = HashMap::new();
        if let Some(v) = checks.get("call_low8").and_then(|m| m.get(id.as_str())) {
            for x in v.as_arr() {
                masks.insert(x.as_usize(), 0xFF);
            }
        }
        if let Some(J::Obj(o)) = checks.get("call_mask").and_then(|m| m.get(id.as_str())) {
            for (idx, m) in o {
                if let Ok(i) = idx.parse::<usize>() {
                    masks.insert(i, m.as_u32());
                }
            }
        }
        let args: Vec<String> = c
            .3
            .iter()
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .map(|(i, &a)| match masks.get(&i) {
                Some(0xFFFF_FFFF) | None => norm_ptr(a),
                Some(m) => format!("0x{:x}", a & m),
            })
            .collect();
        k.push_str(&format!(" args={:?}", args));
    } else {
        k.push_str(" args=transport");
    }
    // Pointed-to snapshots compare by value (raw words: contents, not addresses).
    if !c.4.is_empty() {
        k.push_str(&format!(" snap={:?}", c.4));
    }
    if cal.xmm.log[0] {
        k.push_str(&format!(" xmm0={:?}", c.5));
    }
    if cal.xmm.log[1] {
        k.push_str(&format!(" xmm1={:?}", c.6));
    }
    // v5: XMM2-XMM7, in register order.
    for (r, w) in &c.8 {
        k.push_str(&format!(" xmm{}={:?}", r, w));
    }
    k
}

fn lims(v: &[String], n: usize) -> Vec<String> {
    v.iter().take(n).cloned().collect()
}

fn lim(v: &[(u32, u32)], n: usize) -> Vec<String> {
    v.iter()
        .take(n)
        .map(|(a, b)| format!("0x{:x}=0x{:x}", a, b))
        .collect()
}

// FP compare: exact bits first; else f32/f64 abs+rel tolerance. Returns (pass, detail, exact).
fn fp_cmp(ah: &str, bh: &str, nbytes: usize, checks: &J) -> (bool, String, bool) {
    if ah == bh {
        return (true, "exact".to_string(), true);
    }
    let unhex = |h: &str| -> Vec<u8> {
        let mut v = Vec::new();
        let b = h.as_bytes();
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap_or(0);
            let lo = (b[i + 1] as char).to_digit(16).unwrap_or(0);
            v.push((hi * 16 + lo) as u8);
            i += 2;
        }
        v
    };
    let ab = unhex(ah);
    let bb = unhex(bh);
    let mut detail = format!("orig={} rw={} ", ah, bh);
    if ab.len() < nbytes || bb.len() < nbytes {
        return (false, detail + "short", false);
    }
    // Bitwise difference with zero tolerance is a failure, full stop.
    // (IEEE == would call -0.0 and +0.0 equal; the checker must not.)
    let tol = checks.get("fp_tol").map(|v| v.as_str()).unwrap_or("");
    let (tabs, trel) = if tol.is_empty() {
        (0.0f64, 0.0f64)
    } else {
        let mut sp = tol.split(',');
        let a = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let r = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        (a, r)
    };
    if tabs == 0.0 && trel == 0.0 {
        return (false, detail + "bitwise differ, zero tolerance", false);
    }
    // f32 lane (low 4 bytes little-endian) and f64 lane (low 8)
    let a32 = f32::from_le_bytes([ab[0], ab[1], ab[2], ab[3]]);
    let b32 = f32::from_le_bytes([bb[0], bb[1], bb[2], bb[3]]);
    let (abs, rel) = (tabs, trel);
    let close32 = (a32 == b32)
        || (!a32.is_nan()
            && !b32.is_nan()
            && ((a32 - b32).abs() as f64 <= abs + rel * (b32.abs() as f64)));
    if nbytes == 16
        && checks
            .get("fp64")
            .map(|v| v.as_bool(false))
            .unwrap_or(false)
    {
        let a64 = f64::from_le_bytes([ab[0], ab[1], ab[2], ab[3], ab[4], ab[5], ab[6], ab[7]]);
        let b64 = f64::from_le_bytes([bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], bb[6], bb[7]]);
        let close64 = (a64 == b64)
            || (!a64.is_nan() && !b64.is_nan() && ((a64 - b64).abs() <= abs + rel * b64.abs()));
        detail.push_str(&format!("f64 {} vs {}", a64, b64));
        return (close64, detail, false);
    }
    // x87 st0: 80-bit extended; compare via f64 conversion of the 64-bit mantissa+exponent
    if nbytes == 10 {
        let cvt = |x: &[u8]| -> f64 {
            let man = u64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
            let ex = u16::from_le_bytes([x[8], x[9]]);
            let sign = if ex & 0x8000 != 0 { -1.0 } else { 1.0 };
            let e = (ex & 0x7FFF) as i32;
            if e == 0 && man == 0 {
                return 0.0 * sign;
            }
            if e == 0x7FFF {
                return if man & 0x7FFFFFFFFFFFFFFF == 0 {
                    sign * f64::INFINITY
                } else {
                    f64::NAN
                };
            }
            sign * (man as f64) * 2f64.powi(e - 16383 - 63)
        };
        let a = cvt(&ab);
        let b = cvt(&bb);
        let close =
            (a == b) || (!a.is_nan() && !b.is_nan() && ((a - b).abs() <= abs + rel * b.abs()));
        detail.push_str(&format!("st0 {} vs {}", a, b));
        return (close, detail, false);
    }
    detail.push_str(&format!("f32 {} vs {}", a32, b32));
    (close32, detail, false)
}

// ---------------------------------------------------------------------------
// Commands.
// ---------------------------------------------------------------------------

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// v5: a callee's snapshot entries [{kind, idx, n, at}]. A missing kind is
// "arg" (as in v4); an unknown kind, a non-integer or out-of-range `at`,
// or more than SNAP_MAXW words in total is a setup error (v4 silently
// treated unknown kinds as "arg" and had no `at`).
fn parse_snap(c: &J, id: u32) -> Result<Vec<snap::SnapSpec>, String> {
    let mut out = Vec::new();
    if let Some(ss) = c.get("snap") {
        for sn in ss.as_arr() {
            let kind = match sn.get("kind") {
                None => "arg",
                Some(J::Str(k)) => k.as_str(),
                Some(_) => return Err(format!("callee {} snap kind must be a string", id)),
            };
            let idx = sn.get("idx").map(|v| v.as_usize()).unwrap_or(0);
            let n = sn.get("n").map(|v| v.as_usize()).unwrap_or(0);
            let at = match sn.get("at") {
                None => 0,
                Some(J::Int(i)) => *i,
                Some(_) => return Err(format!("callee {} snap at must be an integer", id)),
            };
            out.push(snap::SnapSpec::new(kind, idx, n, at).map_err(|e| format!("callee {}: {}", id, e))?);
        }
    }
    snap::total_words(id, &out)?;
    Ok(out)
}

// v5: a callee's vector-register options: the v2/v3 keys (logxmm,
// xmm0_from_stack, logxmm1, xmm1_from_stack, unchanged) and the v5 keys
// logxmm_regs [n, ...] and xmm_from_stack {"n": idx} for XMM0-XMM7.
fn parse_xmm(c: &J, id: u32, nargs: usize) -> Result<vecregs::XmmCallCfg, String> {
    let mut keys = vecregs::XmmCallKeys {
        logxmm: c.get("logxmm").map(|v| v.as_bool(false)).unwrap_or(false),
        logxmm1: c.get("logxmm1").map(|v| v.as_bool(false)).unwrap_or(false),
        xmm0_from_stack: c.get("xmm0_from_stack").map(|v| v.as_usize()),
        xmm1_from_stack: c.get("xmm1_from_stack").map(|v| v.as_usize()),
        ..Default::default()
    };
    if let Some(v) = c.get("logxmm_regs") {
        let J::Arr(a) = v else {
            return Err(format!("callee {} logxmm_regs must be a list", id));
        };
        for r in a {
            let J::Int(n) = r else {
                return Err(format!("callee {} logxmm_regs entries must be integers", id));
            };
            keys.logxmm_regs.push(usize::try_from(*n).unwrap_or(usize::MAX));
        }
    }
    if let Some(v) = c.get("xmm_from_stack") {
        let J::Obj(m) = v else {
            return Err(format!("callee {} xmm_from_stack must be an object {{reg: idx}}", id));
        };
        let mut pairs = Vec::new();
        for (k, idx) in m {
            let reg = k
                .parse::<usize>()
                .map_err(|_| format!("callee {} xmm_from_stack key {:?} is not a register number", id, k))?;
            let J::Int(i) = idx else {
                return Err(format!("callee {} xmm_from_stack index must be an integer", id));
            };
            pairs.push((reg, usize::try_from(*i).unwrap_or(usize::MAX)));
        }
        pairs.sort_unstable();
        keys.xmm_from_stack = pairs;
    }
    vecregs::XmmCallCfg::merge(id, &keys, nargs)
}

fn cmd_setup(q: &J) -> String {
    let s = st();
    if !s.mapped {
        let exe_path = q.get("exe").map(|v| v.as_str()).unwrap_or("");
        if exe_path.is_empty() {
            return "{\"ok\":false,\"error\":\"no exe\"}".to_string();
        }
        let bytes = std::fs::read(exe_path).unwrap_or_else(|_| Vec::new());
        if bytes.is_empty() {
            return "{\"ok\":false,\"error\":\"cannot read exe\"}".to_string();
        }
        s.before_stack = vec![0u32; STACK_SNAP_LEN / 4];
        s.exe_bytes = bytes.clone();
        if let Err(e) = map_image(&bytes) {
            return format!("{{\"ok\":false,\"error\":\"{}\"}}", esc(&e));
        }
    }
    // v4: per-setup call-log cap (default 256). The region always
    // reserves LOG_HARD_MAX entries, so raising needs no layout change.
    let log_max = q.get("log_max").map(|v| v.as_u32()).unwrap_or(LOG_MAX as u32);
    if log_max == 0 || log_max as usize > LOG_HARD_MAX {
        return format!(
            "{{\"ok\":false,\"error\":\"log_max {} out of range 1..{}\"}}",
            log_max, LOG_HARD_MAX
        );
    }
    s.log_max = log_max;
    // v5: the read-only shadow at the preferred base, for this contract
    // only (built once per process; an incompletely held window is a
    // setup error, never a partial shadow).
    let want_shadow = q.get("abs_shadow").map(|v| v.as_bool(false)).unwrap_or(false);
    if want_shadow && let Err(e) = abs_build_shadow() {
        return format!("{{\"ok\":false,\"error\":\"{}\"}}", esc(&e));
    }
    s.abs.active = want_shadow;
    let uses_x87 = q.get("x87").map(|v| v.as_bool(false)).unwrap_or(false);
    // callees (stub space is reused per setup, but the first 16 bytes
    // hold the emitted TLS/ESP helpers and are never reused)
    s.callees.clear();
    s.stub_off = 16;
    if let Some(cs) = q.get("callees") {
        for c in cs.as_arr() {
            let id = c.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let conv = c
                .get("conv")
                .map(|v| v.as_str())
                .unwrap_or("cdecl")
                .to_string();
            let nargs = c.get("nargs").map(|v| v.as_usize()).unwrap_or(0);
            if nargs > LOG_MAXW {
                return format!(
                    "{{\"ok\":false,\"error\":\"callee {} nargs {} exceeds log window {}\"}}",
                    id, nargs, LOG_MAXW
                );
            }
            let ret = c
                .get("ret")
                .map(|v| v.as_str())
                .unwrap_or("u32")
                .to_string();
            // (Setup-time scripts are vestigial: per-trial answers arrive
            // with each trial request, so the setup list is not stored.)
            // v3: callee-cleaned stubs pop the full argument count. (v2 capped
            // the cleanup at 8 words like the log window, under-popping for
            // nargs > 8 and corrupting the caller's frame: lane r-b01's fix.)
            // v4 (lane r-s94): "noclean" callees pop nothing on the original
            // side; the stub pops pop_rw only on the rewrite side.
            let noclean = c.get("noclean").map(|v| v.as_bool(false)).unwrap_or(false);
            let pop_rw = (nargs as u32) * 4;
            let mut pop = match conv.as_str() {
                "cdecl" => 0,
                _ => (nargs as u32) * 4, // stdcall/thiscall/fastcall/custom pop
            };
            if noclean {
                pop = 0;
            }
            // v2: out-param writes [{arg|reg, at, n}], snapshots, xmm logging.
            let mut writes = Vec::new();
            if let Some(ws) = c.get("writes") {
                for w in ws.as_arr() {
                    let (kind, idx) = if let Some(r) = w.get("reg") {
                        match r.as_str() {
                            "edx" => (2u8, 0usize),
                            _ => (1u8, 0usize),
                        }
                    } else {
                        (0u8, w.get("arg").map(|v| v.as_usize()).unwrap_or(0))
                    };
                    let at = w.get("at").map(|v| v.as_usize()).unwrap_or(0);
                    let n = w.get("n").map(|v| v.as_usize()).unwrap_or(0);
                    if at + n > WRITEW_PER_ID {
                        return format!(
                            "{{\"ok\":false,\"error\":\"callee {} writes overflow\"}}",
                            id
                        );
                    }
                    // v4 (lane r-b109): optional destination byte offset
                    // added to the pointer (default 0 = v3 behavior exactly).
                    let dst = w.get("dst").map(|v| v.as_usize()).unwrap_or(0);
                    writes.push((kind, idx, at, n, dst));
                }
            }
            // v5: snapshots (64 words, any offset) and XMM0-XMM7 options.
            let (snap, xmm) = match parse_snap(c, id).and_then(|sn| Ok((sn, parse_xmm(c, id, nargs)?))) {
                Ok(v) => v,
                Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", esc(&e)),
            };
            let preserve = c.get("preserve").map(|v| v.as_bool(false)).unwrap_or(false);
            let eax_from_stack = c.get("eax_from_stack").map(|v| v.as_usize());
            let cal = Callee {
                id,
                conv,
                nargs,
                pop,
                ret,
                stub_addr: 0,
                tail_addr: 0,
                writes,
                snap,
                xmm,
                preserve,
                eax_from_stack,
                noclean,
                pop_rw,
            };
            let bytes = emit_stub(&cal, None);
            let addr = s.stub_base + s.stub_off as u32;
            if s.stub_off + bytes.len() > 0x10000 {
                return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                let proc = GetCurrentProcess();
                FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                *((s.ctable + id * 4) as *mut u32) = addr;
            }
            s.stub_off += (bytes.len() + 15) & !15;
            let mut cal = cal;
            cal.stub_addr = addr;
            s.callees.insert(id, cal);
        }
    }
    // E8 patches
    let mut errors: Vec<String> = Vec::new();
    if let Some(ps) = q.get("patches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let site = s.img + site_rva as usize;
            match patch_e8(site, stub) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // v2 E9 tail-jump patches: the tail stub logs like a normal stub
    // but returns straight to the trampoline (outer_pop + the E8 slot).
    let outer_pop = q.get("outer_pop").map(|v| v.as_u32()).unwrap_or(0);
    if let Some(ps) = q.get("tailpatches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let cal = match s.callees.get(&id) {
                Some(c) => c.clone(),
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let tail = if cal.tail_addr != 0 {
                cal.tail_addr
            } else {
                let bytes = emit_stub(&cal, Some(outer_pop));
                let addr = s.stub_base + s.stub_off as u32;
                if s.stub_off + bytes.len() > 0x10000 {
                    return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                    let proc = GetCurrentProcess();
                    FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                }
                s.stub_off += (bytes.len() + 15) & !15;
                if let Some(c) = s.callees.get_mut(&id) {
                    c.tail_addr = addr;
                }
                addr
            };
            let site = s.img + site_rva as usize;
            match patch_e9(site, tail) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // IAT patches
    let mut iat_done: Vec<String> = Vec::new();
    if let Some(ps) = q.get("iat") {
        for p in ps.as_arr() {
            let dll = p.get("dll").map(|v| v.as_str()).unwrap_or("");
            let name = p.get("name").map(|v| v.as_str()).unwrap_or("");
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            match find_iat(dll, name) {
                Some(slot) => unsafe {
                    let mut orig = [0u8; 8];
                    std::ptr::copy_nonoverlapping(slot as *const u8, orig.as_mut_ptr(), 4);
                    *(slot as *mut u32) = stub;
                    s.patches.push((slot, orig, 4));
                    iat_done.push(format!("{}!{}", dll, name));
                },
                None => errors.push(format!("iat {}!{} not found", dll, name)),
            }
        }
    }
    // declared globals
    s.globals.clear();
    if let Some(gs) = q.get("globals") {
        for g in gs.as_arr() {
            let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
            let size = g.get("size").map(|v| v.as_usize()).unwrap_or(0);
            s.globals.push((s.img + rva as usize, size));
        }
    }
    // DLL
    let mut dll_base = String::from("null");
    let mut expmap = String::new();
    if let Some(d) = q.get("dll").map(|v| v.as_str()).filter(|x| !x.is_empty()) {
        if !s.dll.is_null() {
            unsafe {
                FreeLibrary(s.dll);
            }
            s.dll = std::ptr::null_mut();
        }
        s.exports.clear();
        let w = wide(d);
        let h = unsafe { LoadLibraryW(w.as_ptr()) };
        if h.is_null() {
            errors.push(format!("LoadLibrary failed: {}", d));
        } else {
            s.dll = h;
            dll_base = format!("\"0x{:x}\"", h as usize);
            // patch CHECKER_XBASE + CHECKER_CTABLE + CHECKER_XMM data exports
            // (CHECKER_XMM is v2-only; older lane DLLs lack it and report an
            // error string here, which is harmless unless the contract uses it)
            for (sym, val) in [
                ("CHECKER_XBASE\0", s.img as u32),
                ("CHECKER_CTABLE\0", s.ctable),
                ("CHECKER_XMM\0", s.m_xmm_mirror),
                ("CHECKER_TLS\0", s.m_tls_mirror),
            ] {
                let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                if !p.is_null() {
                    unsafe {
                        *(p as *mut u32) = val;
                    }
                } else {
                    errors.push(format!("dll export {} missing", sym.trim_end_matches('\0')));
                }
            }
            // v5: the x87 mirror. Older rewrite DLLs lack the export; that is
            // an error only for a contract that declares x87 entry values
            // (its rewrite could not read them).
            let px = unsafe { GetProcAddress(h, c"CHECKER_X87".as_ptr()) };
            if !px.is_null() {
                unsafe {
                    *(px as *mut u32) = s.m_x87_mirror;
                }
            } else if uses_x87 {
                return "{\"ok\":false,\"error\":\"the contract declares x87 entry values but the rewrite DLL lacks CHECKER_X87 (rebuild it against the current lf-checker-rt)\"}".to_string();
            }
            let mut parts = Vec::new();
            if let Some(es) = q.get("exports") {
                for e in es.as_arr() {
                    let name = e.as_str().to_string();
                    let sym = format!("{}\0", name);
                    let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                    if p.is_null() {
                        errors.push(format!("export {} missing", name));
                    } else {
                        s.exports.insert(name.clone(), p as u32);
                        parts.push(format!("\"{}\":\"0x{:x}\"", name, p as usize));
                    }
                }
            }
            expmap = parts.join(",");
        }
    }
    // v2: stub addresses for driver-side planting (vtables, data slots).
    let mut stubs: Vec<(&u32, &Callee)> = s.callees.iter().collect();
    stubs.sort_by_key(|(id, _)| *id);
    let stubmap = stubs
        .iter()
        .map(|(id, c)| format!("\"{}\":\"0x{:x}\"", id, c.stub_addr))
        .collect::<Vec<_>>()
        .join(",");
    // v5: preferred-base window coverage and the self-test originals.
    let total = (s.abs.hi - s.abs.lo) / abswin::ABS_CHUNK;
    let held = total - abswin::uncovered_chunks(s.abs.lo, s.abs.hi, &s.abs.regions).len();
    let mut names: Vec<&String> = s.selftests.keys().collect();
    names.sort();
    let v5 = format!(
        "\"abs_window\":{{\"lo\":{},\"hi\":{},\"held_chunks\":{},\"total_chunks\":{},\"shadow\":{}}},\"selftests\":[{}]",
        hx(s.abs.lo as u32),
        hx(s.abs.hi as u32),
        held,
        total,
        s.abs.active,
        names.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(",")
    );
    format!(
        "{{\"ok\":true,\"img_base\":{},\"delta\":{},\"relocs\":{},\"heap\":{},\"stack\":{},\"dll_base\":{},\"text_lo\":{},\"text_hi\":{},\"exports\":{{{}}},\"stub_addrs\":{{{}}},\"iat_patched\":[{}],{},\"errors\":[{}]}}",
        hx(s.img as u32),
        hx(s.delta),
        s.relocs,
        hx(s.h as u32),
        hx(s.s as u32),
        dll_base,
        hx(s.text_lo as u32),
        hx(s.text_hi as u32),
        expmap,
        stubmap,
        iat_done
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(","),
        v5,
        errors
            .iter()
            .map(|x| format!("\"{}\"", esc(x)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn cmd_teardown() -> String {
    let s = st();
    for (addr, orig, len) in s.patches.clone() {
        unsafe {
            std::ptr::copy_nonoverlapping(orig.as_ptr(), addr as *mut u8, len);
        }
    }
    s.patches.clear();
    unsafe {
        std::ptr::write_bytes(s.ctable as *mut u8, 0, 1024);
    }
    s.callees.clear();
    s.globals.clear();
    if !s.dll.is_null() {
        unsafe {
            FreeLibrary(s.dll);
        }
        s.dll = std::ptr::null_mut();
    }
    s.exports.clear();
    restore_data();
    "{\"ok\":true}".to_string()
}

// One callee's per-trial answers: (id, script lo, script hi, out-param
// write words, per-call sequence steps).
type ScriptVal = (u32, u32, u32, Vec<u32>, Vec<(u32, u32)>);

#[derive(Clone)]
struct TrialReq {
    fn_addr: u32,
    rw_addr: u32,
    regs: [u32; 7],
    stack: Vec<u32>,
    heapsegs: Vec<(usize, Vec<u32>)>,
    globals_fill: Vec<(usize, Vec<u32>)>,
    trial: u32,
    seed: u32,
    fulldata: bool,
    tls: Vec<(u32, u32)>, // (slot, value) fabricated TLS slots
    xmm: [u32; 32],       // xmm0-7 entry values
    x87: Vec<x87::F80>,   // v5: x87 entry values, ST(0) first
    stack_fill: Option<u32>,
}

fn parse_trial(q: &J) -> Result<(TrialReq, J), String> {
    let s = st();
    // v5: "fn_selftest" names a built-in self-test original instead of an
    // image RVA (the checker's own regression; see build_selftests).
    let fn_addr = if let Some(name) = q.get("fn_selftest") {
        *s.selftests
            .get(name.as_str())
            .ok_or_else(|| format!("unknown selftest {}", name.as_str()))?
    } else {
        let fn_rva = q.get("fn_rva").map(|v| v.as_u32()).unwrap_or(0);
        (s.img + fn_rva as usize) as u32
    };
    let rw_addr = if let Some(e) = q.get("export") {
        *s.exports
            .get(e.as_str())
            .ok_or_else(|| format!("unknown export {}", e.as_str()))?
    } else if let Some(a) = q.get("rw_rva") {
        (s.img + a.as_u32() as usize) as u32
    } else {
        return Err("trial needs export or rw_rva".to_string());
    };
    let regs_v = q
        .get("regs")
        .map(|v| v.as_arr().to_vec())
        .unwrap_or_default();
    if regs_v.len() != 7 {
        return Err("regs must have 7 entries".to_string());
    }
    let mut regs = [0u32; 7];
    for i in 0..7 {
        regs[i] = regs_v[i].as_u32();
    }
    let stack: Vec<u32> = q
        .get("stack")
        .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
        .unwrap_or_default();
    let mut heapsegs = Vec::new();
    if let Some(hs) = q.get("heapsegs") {
        for hseg in hs.as_arr() {
            let off = hseg.get("off").map(|v| v.as_usize()).unwrap_or(0);
            let words: Vec<u32> = hseg
                .get("words")
                .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                .unwrap_or_default();
            if off + words.len() * 4 > HEAP_USE {
                return Err("heapseg out of range".to_string());
            }
            heapsegs.push((off, words));
        }
    }
    let mut globals_fill = Vec::new();
    if let Some(gf) = q.get("globals_fill") {
        if gf.as_str() == "random" {
            // deterministic per-trial fill of declared ranges
            let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
            let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
            for (ri, &(lo, len)) in s.globals.iter().enumerate() {
                let mut words = Vec::new();
                for wi in 0..(len / 4) {
                    words.push(fill_word(seed, trial, ri as u32, wi as u32));
                }
                globals_fill.push((lo, words));
            }
        } else if let Some(arr) = match gf {
            J::Arr(a) => Some(a),
            _ => None,
        } {
            for g in arr {
                let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
                let words: Vec<u32> = g
                    .get("words")
                    .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                    .unwrap_or_default();
                globals_fill.push((s.img + rva as usize, words));
            }
        }
        // "pristine" or missing: no fill
    }
    let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
    let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
    let mut checks = q.get("checks").cloned().unwrap_or(J::Obj(HashMap::new()));
    // v5: x87 entry values [[lo, hi, sexp], ...], ST(0) first. They switch
    // the x87 state check on (it verifies the original consumed them); an
    // explicit "x87_state": false beside them is refused, never honoured.
    let x87_words: Vec<Vec<u32>> = q
        .get("x87")
        .map(|v| {
            v.as_arr()
                .iter()
                .map(|e| e.as_arr().iter().map(|w| w.as_u32()).collect())
                .collect()
        })
        .unwrap_or_default();
    let x87_in = x87::parse_entries(&x87_words)?;
    if !x87_in.is_empty() {
        if matches!(checks.get("x87_state"), Some(J::Bool(false))) {
            return Err("x87 entry values need the x87 state check (x87_state false refused)".to_string());
        }
        if let J::Obj(m) = &mut checks {
            m.insert("x87_state".to_string(), J::Bool(true));
        }
    }
    let fulldata = checks
        .get("fulldata")
        .map(|v| v.as_bool(true))
        .unwrap_or(true);
    // v2: fabricated TLS slots [{slot|slot_rva, value}]
    let mut tls = Vec::new();
    if let Some(ts) = q.get("tls") {
        for te in ts.as_arr() {
            let slot = if let Some(rva) = te.get("slot_rva") {
                let addr = s.img + rva.as_u32() as usize;
                unsafe { *((addr & !3) as *const u32) } // slot index word
            } else {
                te.get("slot").map(|v| v.as_u32()).unwrap_or(0)
            };
            let val = te.get("value").map(|v| v.as_u32()).unwrap_or(0);
            if slot < 64 {
                tls.push((slot, val));
            } else {
                return Err("tls slot >= 64 needs expansion slots (unsupported)".to_string());
            }
        }
    }
    // v2: XMM entry values (sparse {reg:[4 dwords]} or dense [32])
    let mut xmm = [0u32; 32];
    if let Some(x) = q.get("xmm") {
        match x {
            J::Arr(a) => {
                for (i, v) in a.iter().take(32).enumerate() {
                    xmm[i] = v.as_u32();
                }
            }
            J::Obj(m) => {
                for (k, v) in m {
                    if let Ok(r) = k.parse::<usize>()
                        && r < 8
                    {
                        for (i, w) in v.as_arr().iter().take(4).enumerate() {
                            xmm[r * 4 + i] = w.as_u32();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // v2: defined uninitialized-stack fill (integer or "pattern")
    let stack_fill = match q.get("stack_fill") {
        Some(J::Int(i)) => Some(*i as u32),
        Some(J::Str(st)) if st.as_str() != "pattern" => Some(parse_u32(st)),
        _ => None,
    };
    Ok((
        TrialReq {
            fn_addr,
            rw_addr,
            regs,
            stack,
            heapsegs,
            globals_fill,
            trial,
            seed,
            fulldata,
            tls,
            xmm,
            x87: x87_in,
            stack_fill,
        },
        checks,
    ))
}

// The trial body runs on a dedicated thread so a hang can be cut off.
fn trial_body(
    req: TrialReq,
    checks: J,
    script_vals: Vec<ScriptVal>,
) -> String {
    // v2: per-callee script slots + out-param write words, filled once per
    // trial and shared by both sides. v3: per-call answer sequences; a
    // callee without "seq" gets [(script lo,hi)] with length 1, exactly v2.
    unsafe {
        std::ptr::write_bytes(st().m_script_tab as *mut u8, 0, 2048);
        std::ptr::write_bytes(st().m_writebuf as *mut u8, 0, 16384);
        std::ptr::write_bytes(st().m_seq_tab as *mut u8, 0, 32768);
        std::ptr::write_bytes(st().m_seq_len as *mut u8, 0, 1024);
        for (id, lo, hi, w, seq) in &script_vals {
            if *id < 256 {
                *((st().m_script_tab + id * 8) as *mut u32) = *lo;
                *((st().m_script_tab + id * 8 + 4) as *mut u32) = *hi;
                for (j, word) in w.iter().take(WRITEW_PER_ID).enumerate() {
                    *((st().m_writebuf + id * 64 + j as u32 * 4) as *mut u32) = *word;
                }
                let owned: Vec<(u32, u32)> = if seq.is_empty() {
                    vec![(*lo, *hi)]
                } else {
                    seq[..seq.len().min(SEQ_MAX)].to_vec()
                };
                for (k, (slo, shi)) in owned.iter().enumerate() {
                    *((st().m_seq_tab + id * 128 + k as u32 * 8) as *mut u32) = *slo;
                    *((st().m_seq_tab + id * 128 + k as u32 * 8 + 4) as *mut u32) = *shi;
                }
                *((st().m_seq_len + id * 4) as *mut u32) = owned.len() as u32;
            }
        }
    }
    let mut f0 = 0i64;
    let mut f1 = 0i64;
    let mut fq = 0i64;
    unsafe {
        QueryPerformanceFrequency(&mut fq);
        QueryPerformanceCounter(&mut f0);
    }
    let orig = run_side(
        req.fn_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        &req.x87,
        req.stack_fill,
        false,
        req.fulldata,
    );
    let rw = run_side(
        req.rw_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        &req.x87,
        req.stack_fill,
        true,
        req.fulldata,
    );
    unsafe {
        QueryPerformanceCounter(&mut f1);
    }
    let us = if fq > 0 {
        ((f1 - f0) as f64 * 1e6 / fq as f64) as u64
    } else {
        0
    };
    let (pass, checks_json, first) = compare(&orig, &rw, &checks);
    // exact-fp flag for the driver's exact-match counts
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    let exact = match ret {
        "st0" => orig.st0 == rw.st0,
        "xmm0" => orig.xmm0 == rw.xmm0,
        _ => true,
    };
    format!(
        "{{\"ok\":true,\"pass\":{},\"checks\":{},\"first_mismatch\":\"{}\",\"fp_exact\":{},\"trial_us\":{},\"orig\":{},\"rw\":{}}}",
        if pass { "true" } else { "false" },
        checks_json,
        esc(&first),
        if exact { "true" } else { "false" },
        us,
        obs_json(&orig),
        obs_json(&rw)
    )
}

fn main() {
    unsafe {
        let boxed = Box::new(State::new());
        ST = Box::into_raw(boxed);
        AddVectoredExceptionHandler(1, veh);
    }
    // v5: hold the conventional preferred-base window before anything large
    // is allocated (the executable is read later), so unrelocated absolute
    // accesses by the original fault instead of reading worker memory.
    abs_reserve(abswin::EARLY_LO, abswin::EARLY_LO + abswin::EARLY_LEN);
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut out = std::io::stdout();
    // ready banner (driver waits for this)
    writeln!(out, "{{\"ready\":true,\"checker_version\":\"checker4\"}}").unwrap();
    out.flush().unwrap();
    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let q = match parse_json(&line) {
            Ok(J::Obj(m)) => J::Obj(m),
            Ok(_) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"not an object\"}}").unwrap();
                out.flush().unwrap();
                continue;
            }
            Err(e) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"json: {}\"}}", esc(&e)).unwrap();
                out.flush().unwrap();
                continue;
            }
        };
        let cmd = q.get("cmd").map(|v| v.as_str()).unwrap_or("");
        match cmd {
            "ping" => {
                writeln!(out, "{{\"ok\":true,\"mapped\":{}}}", st().mapped).unwrap();
                out.flush().unwrap();
            }
            "setup" => {
                let r = cmd_setup(&q);
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "teardown" => {
                let r = cmd_teardown();
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "trial" => {
                let timeout = q.get("timeout_ms").map(|v| v.as_u32()).unwrap_or(10000);
                // explicit scripted values (driver-resolved per callee scripts)
                let sv: Vec<ScriptVal> = q
                    .get("script_vals")
                    .map(|v| {
                        v.as_arr()
                            .iter()
                            .map(|e| {
                                (
                                    e.get("id").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("lo").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("hi").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("w")
                                        .map(|w| w.as_arr().iter().map(|x| x.as_u32()).collect())
                                        .unwrap_or_default(),
                                    // v3: per-call answer steps [[lo,hi],...]
                                    e.get("seq")
                                        .map(|s| {
                                            s.as_arr()
                                                .iter()
                                                .map(|st| match st {
                                                    J::Arr(a) => (
                                                        a.first().map(|x| x.as_u32()).unwrap_or(0),
                                                        a.get(1).map(|x| x.as_u32()).unwrap_or(0),
                                                    ),
                                                    x => (x.as_u32(), 0),
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let parsed = parse_trial(&q);
                match parsed {
                    Err(e) => {
                        writeln!(out, "{{\"ok\":false,\"error\":\"{}\"}}", esc(&e)).unwrap();
                        out.flush().unwrap();
                    }
                    Ok((req, checks)) => {
                        let (tx, rx) = mpsc::channel();
                        std::thread::Builder::new()
                            .name("trial".to_string())
                            .stack_size(8 * 1024 * 1024)
                            .spawn(move || {
                                let r = trial_body(req, checks, sv);
                                let _ = tx.send(r);
                            })
                            .unwrap();
                        match rx.recv_timeout(Duration::from_millis(timeout as u64)) {
                            Ok(r) => {
                                writeln!(out, "{}", r).unwrap();
                                out.flush().unwrap();
                            }
                            Err(_) => {
                                // hang: the trial thread is abandoned (tainted worker).
                                // Report, then exit so the driver starts a fresh worker.
                                writeln!(
                                    out,
                                    "{{\"ok\":true,\"pass\":false,\"hang\":true,\"checks\":[],\"first_mismatch\":\"watchdog timeout\",\"fp_exact\":false,\"trial_us\":{}}}",
                                    (timeout as u64) * 1000
                                )
                                .unwrap();
                                out.flush().unwrap();
                                diag("watchdog: trial timed out, exiting");
                                unsafe {
                                    ExitProcess(0);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                writeln!(out, "{{\"ok\":false,\"error\":\"unknown cmd\"}}").unwrap();
                out.flush().unwrap();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests of the parts that run no foreign code: the request parser, the
// fill and hash helpers, the stub and trampoline encoders, the comparison
// and its verdict text. They build only for the i686 Windows target (this
// binary links kernel32); the target-independent v5 logic is also tested in
// the library, where `cargo test -p lf-checker-worker --lib` runs anywhere.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // The worker keeps one global State; tests that touch it run one at a
    // time on a fresh, never-mapped State with plausible window addresses.
    static LOCK: Mutex<()> = Mutex::new(());

    fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut s = Box::new(State::new());
        s.h = 0x3000_0000;
        s.s = 0x3100_0000;
        s.img = 0x1000_0000;
        s.image_size = 0x0200_0000;
        s.esp0 = 0x310F_D000;
        s.log_base = 0x3101_2000;
        s.ext_snap_base = 0x3106_6000;
        s.ext_xmm_base = 0x310A_A000;
        s.m_ctx = 0x310F_F000;
        s.m_fn = 0x310F_F004;
        s.m_hostesp = 0x310F_F008;
        s.m_mxcsr = 0x310F_F00C;
        s.m_tmp = 0x310F_F010;
        s.fxbuf = 0x310F_D800;
        s.m_logidx = 0x310F_F068;
        s.m_side = 0x310F_F06C;
        s.m_save_ecx = 0x310F_F070;
        s.m_save_edx = 0x310F_F074;
        s.m_step = 0x310F_F078;
        s.m_save_eax = 0x310F_F07C;
        s.m_seq_tab = 0x3105_C000;
        s.m_seq_len = 0x3106_4000;
        s.m_seq_idx = 0x3106_5000;
        s.m_writebuf = 0x3105_8000;
        s.ctable = 0x3105_6000;
        let p = Box::into_raw(s);
        unsafe {
            ST = p;
        }
        let r = f(unsafe { &mut *p });
        unsafe {
            ST = std::ptr::null_mut();
            drop(Box::from_raw(p));
        }
        r
    }

    fn j(s: &str) -> J {
        parse_json(s).expect("test JSON parses")
    }

    fn contains(hay: &[u8], needle: &[u8]) -> bool {
        hay.windows(needle.len()).any(|w| w == needle)
    }

    fn ok_obs() -> Obs {
        Obs {
            status: "ok".to_string(),
            ..Obs::default()
        }
    }

    fn checks_named(json: &str) -> Vec<(String, bool)> {
        // The checks list compare() returns, as (name, passed) pairs.
        let v = j(json);
        v.as_arr()
            .iter()
            .map(|c| {
                (
                    c.get("name").map(|x| x.as_str().to_string()).unwrap_or_default(),
                    c.get("passed").map(|x| x.as_bool(false)).unwrap_or(false),
                )
            })
            .collect()
    }

    fn callee(id: u32) -> Callee {
        Callee {
            id,
            conv: "cdecl".to_string(),
            nargs: 2,
            ret: "u32".to_string(),
            ..Callee::default()
        }
    }

    // --- request parsing -------------------------------------------------

    #[test]
    fn json_scalars_and_nesting() {
        let v = j(r#"{"a":1,"b":-2,"c":true,"d":null,"e":"x","f":[1,[2]],"g":{"h":"0x10"}}"#);
        assert_eq!(v.get("a").unwrap().as_i64(), 1);
        assert_eq!(v.get("b").unwrap().as_i64(), -2);
        assert!(v.get("c").unwrap().as_bool(false));
        assert!(matches!(v.get("d"), Some(J::Null)));
        assert_eq!(v.get("e").unwrap().as_str(), "x");
        assert_eq!(v.get("f").unwrap().as_arr()[1].as_arr()[0].as_i64(), 2);
        // Hex strings read as numbers where a number is expected.
        assert_eq!(v.get("g").unwrap().get("h").unwrap().as_u32(), 16);
        assert!(v.get("missing").is_none());
    }

    #[test]
    fn json_strings_and_errors() {
        let v = j(r#"{"s":"a\"b\\c\nA"}"#);
        assert_eq!(v.get("s").unwrap().as_str(), "a\"b\\c\nA");
        assert!(parse_json("{\"a\":1} x").is_err());
        assert!(parse_json("{\"a\" 1}").is_err());
        assert!(parse_json("[1,2").is_err());
        assert!(parse_json("\"open").is_err());
        assert!(parse_json("{1:2}").is_err());
    }

    #[test]
    fn parse_u32_forms() {
        assert_eq!(parse_u32("0x1F"), 0x1F);
        assert_eq!(parse_u32("0X1f"), 0x1F);
        assert_eq!(parse_u32("-1"), 0xFFFF_FFFF);
        assert_eq!(parse_u32(" 42 "), 42);
        assert_eq!(parse_u32("junk"), 0);
    }

    #[test]
    fn emitters_escape_and_hex() {
        assert_eq!(esc("a\"b\\\n\t\u{1}"), "a\\\"b\\\\\\n\\t\\u0001");
        assert_eq!(hx(0xAB), "\"0xab\"");
        assert_eq!(hexbytes(&[0x00, 0x7F, 0xFF]), "007fff");
        assert_eq!(unhex("007fff"), vec![0x00, 0x7F, 0xFF]);
    }

    #[test]
    fn parse_trial_x87_entries_turn_the_state_check_on() {
        with_state(|s| {
            s.exports.insert("rw".to_string(), 0x1234);
            let q = j(r#"{"fn_rva":16,"export":"rw","regs":[0,0,0,0,0,0,0],
                "x87":[[0,2147483648,16383],[0,0,0]],"checks":{"ret":"eax"}}"#);
            let (req, checks) = parse_trial(&q).unwrap();
            assert_eq!(req.fn_addr, 0x1000_0010);
            assert_eq!(req.rw_addr, 0x1234);
            assert_eq!(req.x87.len(), 2);
            assert_eq!(req.x87[0].sexp, 0x3FFF);
            assert!(checks.get("x87_state").unwrap().as_bool(false));
        });
    }

    #[test]
    fn parse_trial_refuses_x87_without_state_check_and_bad_entries() {
        with_state(|s| {
            s.exports.insert("rw".to_string(), 1);
            let off = j(r#"{"export":"rw","regs":[0,0,0,0,0,0,0],"x87":[[0,0,0]],
                "checks":{"x87_state":false}}"#);
            assert!(parse_trial(&off).is_err());
            let nine = format!(
                r#"{{"export":"rw","regs":[0,0,0,0,0,0,0],"x87":[{}]}}"#,
                ["[0,0,0]"; 9].join(",")
            );
            assert!(parse_trial(&j(&nine)).is_err());
            let wide = j(r#"{"export":"rw","regs":[0,0,0,0,0,0,0],"x87":[[0,0,65536]]}"#);
            assert!(parse_trial(&wide).is_err());
            // No x87 key: no entries, the check stays off (v4 behaviour).
            let none = j(r#"{"export":"rw","regs":[0,0,0,0,0,0,0],"checks":{}}"#);
            let (req, checks) = parse_trial(&none).unwrap();
            assert!(req.x87.is_empty());
            assert!(checks.get("x87_state").is_none());
        });
    }

    #[test]
    fn parse_trial_selftest_and_basic_errors() {
        with_state(|s| {
            s.exports.insert("rw".to_string(), 1);
            s.selftests.insert("x87_store".to_string(), 0x3101_1000);
            let q = j(r#"{"fn_selftest":"x87_store","export":"rw","regs":[0,0,0,0,0,0,0]}"#);
            assert_eq!(parse_trial(&q).unwrap().0.fn_addr, 0x3101_1000);
            let bad = j(r#"{"fn_selftest":"nope","export":"rw","regs":[0,0,0,0,0,0,0]}"#);
            assert!(parse_trial(&bad).is_err());
            assert!(parse_trial(&j(r#"{"export":"rw","regs":[0,0,0]}"#)).is_err());
            assert!(parse_trial(&j(r#"{"export":"missing","regs":[0,0,0,0,0,0,0]}"#)).is_err());
            assert!(parse_trial(&j(r#"{"regs":[0,0,0,0,0,0,0]}"#)).is_err());
        });
    }

    #[test]
    fn parse_trial_xmm_tls_and_stack_fill() {
        with_state(|s| {
            s.exports.insert("rw".to_string(), 1);
            let q = j(r#"{"export":"rw","regs":[1,2,3,4,5,6,7],"stack":[9],
                "xmm":{"7":[1,2,3,4],"9":[5]},"tls":[{"slot":3,"value":8}],
                "stack_fill":0,"heapsegs":[{"off":16,"words":[1,2]}]}"#);
            let (req, _) = parse_trial(&q).unwrap();
            assert_eq!(req.regs, [1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(req.stack, vec![9]);
            assert_eq!(&req.xmm[28..32], &[1, 2, 3, 4]);
            assert!(req.xmm[..28].iter().all(|&w| w == 0)); // xmm9 ignored
            assert_eq!(req.tls, vec![(3, 8)]);
            assert_eq!(req.stack_fill, Some(0));
            assert_eq!(req.heapsegs, vec![(16, vec![1, 2])]);
            let far = j(r#"{"export":"rw","regs":[0,0,0,0,0,0,0],"tls":[{"slot":64,"value":1}]}"#);
            assert!(parse_trial(&far).is_err());
            let out = j(r#"{"export":"rw","regs":[0,0,0,0,0,0,0],"heapsegs":[{"off":983036,"words":[1,2]}]}"#);
            assert!(parse_trial(&out).is_err());
        });
    }

    #[test]
    fn parse_snap_v5_fields_and_caps() {
        let c = j(r#"{"snap":[{"kind":"ecx","n":16,"at":472},{"idx":1,"n":4,"at":-16}]}"#);
        let sn = parse_snap(&c, 1).unwrap();
        assert_eq!(sn.len(), 2);
        assert_eq!(sn[0].at, 472);
        assert_eq!(sn[1].kind_code(), (0, 1)); // missing kind is "arg"
        assert_eq!(sn[1].disp(0), (-16i32) as u32);
        let over = j(r#"{"snap":[{"kind":"ecx","n":40},{"kind":"edx","n":25}]}"#);
        assert!(parse_snap(&over, 2).unwrap_err().contains("cap"));
        assert!(parse_snap(&j(r#"{"snap":[{"kind":"esi","n":1}]}"#), 1).is_err());
        assert!(parse_snap(&j(r#"{"snap":[{"kind":"ecx","n":1,"at":"4"}]}"#), 1).is_err());
        assert!(parse_snap(&j(r#"{"snap":[{"kind":"ecx","n":1,"at":70000}]}"#), 1).is_err());
        assert!(parse_snap(&j(r#"{}"#), 1).unwrap().is_empty());
    }

    #[test]
    fn parse_xmm_v5_fields() {
        let c = j(r#"{"logxmm":true,"logxmm_regs":[2,5],"xmm_from_stack":{"5":1,"2":0}}"#);
        let x = parse_xmm(&c, 1, 2).unwrap();
        assert!(x.log[0] && x.log[2] && x.log[5] && !x.log[1]);
        assert_eq!(x.from_stack[2], Some(0));
        assert_eq!(x.from_stack[5], Some(1));
        assert!(parse_xmm(&j(r#"{"logxmm_regs":"2"}"#), 1, 2).is_err());
        assert!(parse_xmm(&j(r#"{"logxmm_regs":[2],"xmm_from_stack":{"x":0}}"#), 1, 2).is_err());
        assert!(parse_xmm(&j(r#"{"xmm_from_stack":{"3":0}}"#), 1, 2).is_err());
        // Legacy keys alone keep their v4 meaning.
        let l = parse_xmm(&j(r#"{"logxmm1":true,"xmm1_from_stack":0}"#), 1, 1).unwrap();
        assert!(l.log[1] && l.from_stack[1] == Some(0) && l.ext_logged().is_empty());
    }

    // --- fill and hash helpers -----------------------------------------

    #[test]
    fn fills_are_deterministic_and_vary() {
        assert_eq!(fill_word(1, 2, 3, 4), fill_word(1, 2, 3, 4));
        assert_ne!(fill_word(1, 2, 3, 4), fill_word(1, 3, 3, 4));
        assert_ne!(fill_word(1, 2, 3, 4), fill_word(1, 2, 3, 5));
        for i in 0..1000 {
            assert_ne!(heap_pattern(i, 7, 9), 0);
        }
        assert_ne!(heap_pattern(5, 1, 9), heap_pattern(5, 2, 9));
    }

    #[test]
    fn fnv_hash_depends_on_offset_and_value() {
        let h0 = 0xcbf2_9ce4_8422_2325u64;
        assert_eq!(fnv1a(h0, 4, 7), fnv1a(h0, 4, 7));
        assert_ne!(fnv1a(h0, 4, 7), fnv1a(h0, 8, 7));
        assert_ne!(fnv1a(h0, 4, 7), fnv1a(h0, 4, 8));
        // Order matters: the write stream is hashed in address order.
        assert_ne!(fnv1a(fnv1a(h0, 0, 1), 4, 2), fnv1a(fnv1a(h0, 4, 2), 0, 1));
    }

    #[test]
    fn nan_canonicalisation_is_diagnostic_only() {
        assert!(is_f32_nan(0x7FC0_0001) && is_f32_nan(0xFF80_0001));
        assert!(!is_f32_nan(0x7F80_0000));
        assert_eq!(canon_f32(0xFFC0_1234), 0x7FC0_0000);
        assert_eq!(canon_f32(0x3F80_0000), 0x3F80_0000);
        assert_eq!(canon_st0_hex("010000000000c0ffff7f"), "00000000000000c0ff7f");
        assert_eq!(canon_st0_hex("0000000000000080ff3f"), "0000000000000080ff3f");
        let x = canon_xmm0_hex("0100c0ff000000000000000000000000");
        assert_eq!(&x[..8], "0000c07f");
    }

    #[test]
    fn fp_compare_is_bit_exact_by_default() {
        let none = j("{}");
        assert!(fp_cmp("0000803f", "0000803f", 4, &none).0);
        // +0 and -0 differ.
        assert!(!fp_cmp("00000000", "00000080", 4, &none).0);
        // Two NaNs with different payloads differ.
        assert!(!fp_cmp("0100c07f", "0200c07f", 4, &none).0);
        let zero = j(r#"{"fp_tol":"0,0"}"#);
        assert!(!fp_cmp("0000803f", "0100803f", 4, &zero).0);
        // A tolerance is honoured only when declared.
        let tol = j(r#"{"fp_tol":"0.001,0"}"#);
        assert!(fp_cmp("0000803f", "0100803f", 4, &tol).0);
        assert!(!fp_cmp("short", "0000803f", 4, &tol).0);
    }

    #[test]
    fn zero_masks_and_eax_regs_are_refused() {
        assert!(find_zero_mask(&j(r#"{"call_mask":{"1":{"0":0}}}"#)).is_some());
        assert!(find_zero_mask(&j(r#"{"call_mask":{"1":{"0":"0xff"}}}"#)).is_none());
        assert!(find_zero_mask(&j("{}")).is_none());
        with_state(|s| {
            s.callees.insert(1, callee(1));
            let e = validate_eax_regs(&j(r#"{"call_regs":{"1":["eax"]}}"#));
            assert!(e.is_some());
            let mut c2 = callee(2);
            c2.eax_from_stack = Some(0);
            s.callees.insert(2, c2);
            assert!(validate_eax_regs(&j(r#"{"call_regs":{"2":["eax"]}}"#)).is_none());
            assert!(validate_eax_transport(&j("{}")).is_some());
        });
    }

    // --- encoders --------------------------------------------------------

    #[test]
    fn stub_keeps_v4_encodings_for_legacy_options() {
        with_state(|_| {
            let mut c = callee(1);
            c.xmm = vecregs::XmmCallCfg::merge(
                1,
                &vecregs::XmmCallKeys {
                    logxmm: true,
                    xmm0_from_stack: Some(1),
                    ..Default::default()
                },
                2,
            )
            .unwrap();
            c.snap = vec![snap::SnapSpec::new("arg", 0, 2, 0).unwrap()];
            let b = emit_stub(&c, None);
            // v4 xmm0 transport and logging, byte for byte.
            assert!(contains(&b, &[0xF3, 0x0F, 0x10, 0x84, 0x24, 8, 0, 0, 0]));
            assert!(contains(
                &b,
                &[0x0F, 0x13, 0x80, 0xE0, 0, 0, 0, 0x0F, 0x17, 0x80, 0xE8, 0, 0, 0]
            ));
            // v4 snapshot copies: [edx+0] -> [eax+192], [edx+4] -> [eax+196].
            assert!(contains(
                &b,
                &[0x8B, 0x8A, 0, 0, 0, 0, 0x89, 0x88, 0xC0, 0, 0, 0, 0x8B, 0x8A, 4, 0, 0, 0, 0x89, 0x88, 0xC4, 0, 0, 0]
            ));
            assert!(!contains(&b, &[0x0F, 0x13, 0x90])); // no xmm2 logging
        });
    }

    #[test]
    fn stub_places_wide_and_offset_snapshots() {
        with_state(|s| {
            let ext = s.ext_snap_base - s.log_base;
            let mut c = callee(1);
            c.snap = vec![
                snap::SnapSpec::new("ecx", 0, 9, 0x1D8).unwrap(),
                snap::SnapSpec::new("ecx", 0, 1, -8).unwrap(),
            ];
            let b = emit_stub(&c, None);
            // Word 8 of the first entry: [edx+0x1F8] -> extension word 0.
            let mut w8 = vec![0x8B, 0x8A];
            w8.extend_from_slice(&0x1F8u32.to_le_bytes());
            w8.extend_from_slice(&[0x89, 0x88]);
            w8.extend_from_slice(&ext.to_le_bytes());
            assert!(contains(&b, &w8));
            // The second ECX entry reloads the spilled ECX (v5 fix), then
            // reads [edx-8] into extension word 1.
            let mut reload = vec![0x8B, 0x15];
            reload.extend_from_slice(&s.m_save_ecx.to_le_bytes());
            assert!(contains(&b, &reload));
            let mut neg = vec![0x8B, 0x8A];
            neg.extend_from_slice(&0xFFFF_FFF8u32.to_le_bytes());
            neg.extend_from_slice(&[0x89, 0x88]);
            neg.extend_from_slice(&(ext + 4).to_le_bytes());
            assert!(contains(&b, &neg));
            // The first entry keeps v4's mov edx,ecx.
            assert!(contains(&b, &[0x8B, 0xD1]));
        });
    }

    #[test]
    fn stub_logs_and_transports_xmm2_to_xmm7() {
        with_state(|s| {
            let ext = s.ext_xmm_base - s.log_base;
            let mut c = callee(1);
            c.xmm = vecregs::XmmCallCfg::merge(
                1,
                &vecregs::XmmCallKeys {
                    logxmm_regs: vec![2, 7],
                    xmm_from_stack: vec![(2, 0)],
                    ..Default::default()
                },
                2,
            )
            .unwrap();
            let b = emit_stub(&c, None);
            assert!(contains(&b, &vecregs::movss_from_esp(2, 4)));
            assert!(contains(&b, &vecregs::log_store(2, ext)));
            assert!(contains(&b, &vecregs::log_store(7, ext + 80)));
            assert!(!contains(&b, &vecregs::movss_from_esp(7, 4)));
        });
    }

    #[test]
    fn trampoline_loads_x87_entries_deepest_first_with_flags_saved() {
        with_state(|_| {
            let (t, pad) = trampoline_bytes();
            // cmp dword [ebp+708],7 ; jbe +6 ; fld tbyte [ebp+712+84] first.
            let mut first = vec![0x9C, 0x83, 0xBD];
            first.extend_from_slice(&708u32.to_le_bytes());
            first.extend_from_slice(&[7, 0x76, 0x06, 0xDB, 0xAD]);
            first.extend_from_slice(&(712u32 + 84).to_le_bytes());
            assert!(contains(&t, &first));
            // ... and ST(0) last, then popfd.
            let mut last = vec![0x83, 0xBD];
            last.extend_from_slice(&708u32.to_le_bytes());
            last.extend_from_slice(&[0, 0x76, 0x06, 0xDB, 0xAD]);
            last.extend_from_slice(&712u32.to_le_bytes());
            last.push(0x9D);
            assert!(contains(&t, &last));
            // The landing pad empties the FPU, restores flags and registers.
            assert_eq!(&t[pad..], &[0xDB, 0xE3, 0x9D, 0x61, 0xC3]);
            assert_eq!(CTX_X87_N * 4, 708);
            assert!(CTX_WORDS <= State::new().ctx.len());
        });
    }

    #[test]
    fn selftest_page_layout() {
        let (code, table) = selftest_code(0x3101_1000, 0x3105_6004);
        assert!(code.len() <= 0x1000);
        let at = |n: &str| (table[n] - 0x3101_1000) as usize;
        assert_eq!(at("x87_store"), 0);
        assert_eq!(
            &code[..15],
            &[0x8B, 0x4C, 0x24, 0x04, 0xD9, 0x19, 0xDD, 0x59, 0x04, 0xDB, 0x79, 0x0C, 0x89, 0xC8, 0xC3]
        );
        assert_eq!(at("xmm_call") % 16, 0);
        let x = &code[at("xmm_call")..];
        assert_eq!(&x[16..22], &[0xFF, 0x15, 0x04, 0x60, 0x05, 0x31]);
        assert_eq!(&code[at("abs_read")..at("abs_read") + 7], &[0x8B, 0x44, 0x24, 0x04, 0x8B, 0x00, 0xC3]);
    }

    // --- comparison and verdict text ----------------------------------

    #[test]
    fn identical_observations_pass_every_default_check() {
        with_state(|_| {
            let (pass, json, first) = compare(&ok_obs(), &ok_obs(), &j("{}"));
            assert!(pass, "{first}");
            let names: Vec<String> = checks_named(&json).into_iter().map(|c| c.0).collect();
            assert_eq!(
                names,
                vec!["termination", "no_cheat", "ret", "esp", "heap", "stack", "globals", "calls", "undeclared"]
            );
        });
    }

    #[test]
    fn return_channels_compare_their_bits() {
        with_state(|_| {
            let a = ok_obs();
            let mut b = ok_obs();
            b.regs[0] = 0x100;
            assert!(!compare(&a, &b, &j("{}")).0);
            // al compares the low byte only; ret none compares nothing.
            assert!(compare(&a, &b, &j(r#"{"ret":"al"}"#)).0);
            assert!(compare(&a, &b, &j(r#"{"ret":"none"}"#)).0);
            let mut c = ok_obs();
            c.regs[2] = 1;
            assert!(!compare(&a, &c, &j(r#"{"ret":"edx_eax"}"#)).0);
            let mut s0 = ok_obs();
            s0.st0 = "0000000000000080ff3f".to_string();
            let mut s1 = ok_obs();
            s1.st0 = "0000000000000080ffbf".to_string();
            assert!(!compare(&s0, &s1, &j(r#"{"ret":"st0"}"#)).0);
        });
    }

    #[test]
    fn memory_stack_and_esp_differences_fail() {
        with_state(|_| {
            let a = ok_obs();
            let mut h = ok_obs();
            h.heap_n = 1;
            h.heap_hash = 9;
            let (p, _, first) = compare(&a, &h, &j("{}"));
            assert!(!p && first.starts_with("heap"), "{first}");
            assert!(compare(&a, &h, &j(r#"{"heap":false}"#)).0);
            let mut st = ok_obs();
            st.stack_hash = 1;
            assert!(!compare(&a, &st, &j("{}")).0);
            let mut e = ok_obs();
            e.esp_delta = 4;
            assert!(!compare(&a, &e, &j("{}")).0);
            let mut g = ok_obs();
            g.globals_writes.push((0x10, 1));
            assert!(!compare(&a, &g, &j("{}")).0);
            let mut u = ok_obs();
            u.undeclared_n = 1;
            assert!(!compare(&a, &u, &j("{}")).0);
        });
    }

    #[test]
    fn termination_rules() {
        with_state(|_| {
            let mut fa = ok_obs();
            fa.status = "fault".to_string();
            fa.fault_code = 0xC000_0005;
            let mut fb = fa.clone();
            // Fault against fault with the same code passes termination and
            // compares nothing else.
            let (p, json, _) = compare(&fa, &fb, &j("{}"));
            assert!(p);
            assert_eq!(checks_named(&json).len(), 2);
            fb.fault_code = 0xC000_0094;
            assert!(!compare(&fa, &fb, &j("{}")).0);
            assert!(!compare(&ok_obs(), &fa, &j("{}")).0);
            let mut cheat = ok_obs();
            cheat.status = "cheat".to_string();
            let (p, _, first) = compare(&ok_obs(), &cheat, &j("{}"));
            assert!(!p && first.starts_with("termination"), "{first}");
        });
    }

    #[test]
    fn calls_compare_in_order_with_snapshots_and_vectors() {
        with_state(|s| {
            let mut cal = callee(1);
            cal.xmm.log[3] = true;
            s.callees.insert(1, cal);
            let rec = |snap: Vec<u32>, x3: u32| -> CallRec {
                (1, 0, 0, vec![1, 2], snap, [0; 4], [0; 4], 0, vec![(3, [x3, 0, 0, 0])])
            };
            let mut a = ok_obs();
            a.calls = vec![rec(vec![5; 20], 7)];
            let mut b = a.clone();
            assert!(compare(&a, &b, &j("{}")).0);
            // A snapshot word past the eighth differs.
            b.calls[0].4[12] = 6;
            assert!(!compare(&a, &b, &j("{}")).0);
            // XMM3 at the call differs.
            let mut c = a.clone();
            c.calls[0].8[0].1[0] = 8;
            assert!(!compare(&a, &c, &j("{}")).0);
            // A missing call fails; an argument can be skipped explicitly.
            let mut d = a.clone();
            d.calls.clear();
            assert!(!compare(&a, &d, &j("{}")).0);
            let mut e = a.clone();
            e.calls[0].3[1] = 3;
            assert!(!compare(&a, &e, &j("{}")).0);
            assert!(compare(&a, &e, &j(r#"{"call_skip":{"1":[1]}}"#)).0);
        });
    }

    #[test]
    fn transport_callees_compare_registers_not_stack_words() {
        with_state(|s| {
            let mut cal = callee(1);
            cal.xmm.log[2] = true;
            cal.xmm.from_stack[2] = Some(0);
            s.callees.insert(1, cal);
            let mut a = ok_obs();
            a.calls = vec![(1, 0, 0, vec![1, 2], vec![], [0; 4], [0; 4], 0, vec![(2, [9, 0, 0, 0])])];
            let mut b = a.clone();
            b.calls[0].3 = vec![7, 7]; // garbage stack words on one side
            assert!(compare(&a, &b, &j("{}")).0);
            b.calls[0].8[0].1[0] = 10;
            assert!(!compare(&a, &b, &j("{}")).0);
            let key = callkey(&a.calls[0], &j("{}"));
            assert!(key.contains("args=transport") && key.contains("xmm2=[9, 0, 0, 0]"), "{key}");
        });
    }

    #[test]
    fn truncated_call_log_fails_loudly() {
        with_state(|_| {
            let mut a = ok_obs();
            a.log_attempted = 300;
            a.log_logged = 256;
            let (p, _, first) = compare(&a, &a.clone(), &j("{}"));
            assert!(!p && first.contains("call log truncated"), "{first}");
        });
    }

    fn x87_obs(top: u8, tags: u8, st0: x87::F80) -> Obs {
        let mut o = ok_obs();
        let mut regs = [x87::F80::default(); 8];
        regs[0] = st0;
        o.x87 = Some(x87::X87State { top, tags, regs });
        o
    }

    #[test]
    fn x87_state_check_is_opt_in_and_strict() {
        with_state(|_| {
            let one = x87::F80 {
                man: 1 << 63,
                sexp: 0x3FFF,
            };
            let empty = x87_obs(0, 0, one);
            let pushed = x87_obs(7, 0x80, one);
            // Off unless asked: v4 check lists are unchanged.
            let (p, json, _) = compare(&empty, &pushed, &j("{}"));
            assert!(p);
            assert!(!checks_named(&json).iter().any(|c| c.0 == "x87"));
            let on = j(r#"{"x87_state":true}"#);
            let (p, json, first) = compare(&empty, &pushed, &on);
            assert!(!p && first.starts_with("x87"), "{first}");
            assert!(checks_named(&json).contains(&("x87".to_string(), false)));
            assert!(compare(&pushed, &pushed.clone(), &on).0);
            let two = x87_obs(7, 0x80, x87::F80 { man: 1 << 63, sexp: 0x4000 });
            assert!(!compare(&pushed, &two, &on).0);
            // A side whose state was not captured never passes.
            assert!(!compare(&ok_obs(), &ok_obs(), &on).0);
        });
    }

    #[test]
    fn nan_only_differences_fail_with_a_diagnostic() {
        with_state(|_| {
            let mut a = ok_obs();
            a.regs[0] = 0x7FC0_0001;
            let mut b = ok_obs();
            b.regs[0] = 0x7FC0_0002;
            let mut x = ok_obs();
            x.xmm0 = "0100c07f000000000000000000000000".to_string();
            let mut y = ok_obs();
            y.xmm0 = "0200c07f000000000000000000000000".to_string();
            let (p, _, first) = compare(&x, &y, &j(r#"{"ret":"xmm0"}"#));
            assert!(!p);
            assert!(first.contains("nan-diagnostic"), "{first}");
            // eax is not a float channel: no diagnostic, still a failure.
            let (p, _, first) = compare(&a, &b, &j("{}"));
            assert!(!p && !first.contains("nan-diagnostic"));
        });
    }

    #[test]
    fn pointers_normalise_by_region() {
        with_state(|s| {
            assert_eq!(norm_ptr(s.h as u32 + 0x10), "H+10");
            assert_eq!(norm_ptr(s.esp0 + 8), "S+8");
            assert_eq!(norm_ptr(s.esp0 - 8), "S-8");
            assert_eq!(norm_ptr(s.img as u32 + 0x20), "I+20");
            assert_eq!(norm_ptr(5), "0x5");
        });
    }

    #[test]
    fn observation_json_carries_v5_fields_only_when_present() {
        with_state(|_| {
            let mut o = ok_obs();
            o.calls = vec![(1, 0, 0, vec![], vec![], [0; 4], [0; 4], 0, vec![])];
            let plain = obs_json(&o);
            assert!(j(&plain).get("x87").is_none());
            assert!(!plain.contains("xmm2"));
            o.calls[0].8.push((2, [1, 2, 3, 4]));
            o.x87 = Some(x87::X87State {
                top: 7,
                tags: 0x80,
                regs: [x87::F80 { man: 1 << 63, sexp: 0x3FFF }; 8],
            });
            let v = j(&obs_json(&o));
            let x = v.get("x87").unwrap();
            assert_eq!(x.get("top").unwrap().as_i64(), 7);
            assert_eq!(x.get("st").unwrap().as_arr().len(), 1);
            let call = &v.get("calls").unwrap().as_arr()[0];
            assert_eq!(call.get("xmm2").unwrap().as_arr()[3].as_u32(), 4);
        });
    }
}
