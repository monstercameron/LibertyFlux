//! Unit tests on hand-built byte arrays. Every fixture below is constructed
//! in code; none contains bytes copied from game files.

use lf_shaderpack::bytecode;
use lf_shaderpack::dcl;
use lf_shaderpack::sps;
use lf_shaderpack::{AnnotationValue, ErrorKind, ParamKind, ShaderPack, Stage};

fn sized_str(out: &mut Vec<u8>, s: &str) {
    out.push((s.len() + 1) as u8);
    out.extend_from_slice(s.as_bytes());
    out.push(0);
}

fn u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

/// Minimal valid Direct3D 9 blob: version token, one empty instruction, end.
fn tiny_blob(version: u32) -> Vec<u8> {
    let mut b = Vec::new();
    u32(&mut b, version);
    u32(&mut b, 0x0000_0000); // opcode with zero parameter dwords
    u32(&mut b, bytecode::END);
    b
}

/// A complete container: 1 vs + 1 ps, 1 shared + 1 material param,
/// 1 technique with 1 pass carrying 1 render state.
fn minimal_container() -> Vec<u8> {
    let mut b = Vec::new();
    u32(&mut b, lf_shaderpack::MAGIC);
    b.push(1); // n_vs
    b.push(1); // one bound var
    b.push(5); // type: vector
    b.push(0); // index
    u16(&mut b, 39); // register
    sized_str(&mut b, "globalScalars");
    let vs = tiny_blob(0xFFFE_0300);
    u16(&mut b, vs.len() as u16);
    u16(&mut b, vs.len() as u16);
    b.extend_from_slice(&vs);
    b.push(2); // n_ps_raw: 2 means 1 program
    b.extend_from_slice(&[0, 0, 0, 0, 0]);
    b.push(0); // ps binds no names
    let ps = tiny_blob(0xFFFF_0300);
    u16(&mut b, ps.len() as u16);
    u16(&mut b, ps.len() as u16);
    b.extend_from_slice(&ps);
    b.push(1); // one shared param
    b.push(5); // vector
    b.push(0); // single
    sized_str(&mut b, "globalScalars");
    sized_str(&mut b, "globalScalars");
    b.push(1); // one annotation
    sized_str(&mut b, "UIName");
    b.push(2); // string kind
    sized_str(&mut b, "Global Scalars");
    b.push(4); // four default dwords: 1,1,1,1
    for _ in 0..4 {
        u32(&mut b, 0x3F80_0000);
    }
    b.push(1); // one material param
    b.push(7); // bool switch
    b.push(0);
    sized_str(&mut b, "switchOn");
    sized_str(&mut b, "switchOn");
    b.push(0); // no annotations
    b.push(1); // default: off
    u32(&mut b, 0);
    b.push(1); // one technique
    sized_str(&mut b, "draw");
    b.push(1); // one pass
    b.push(0); // vs 0
    b.push(1); // ps 1-based -> program 0
    b.push(1); // one render state
    u32(&mut b, 10);
    u32(&mut b, 1);
    b
}

#[test]
fn parses_minimal_container() {
    let bytes = minimal_container();
    let pack = ShaderPack::parse(&bytes).expect("minimal container parses");
    assert!(pack.trailing_bytes.is_empty());
    assert_eq!(pack.vs_programs.len(), 1);
    assert_eq!(pack.ps_programs.len(), 1);
    assert_eq!(pack.ps_padding, [0, 0, 0, 0, 0]);

    let vs = &pack.vs_programs[0];
    assert_eq!(vs.stage, Stage::Vertex);
    assert_eq!(vs.vars.len(), 1);
    assert_eq!(vs.vars[0].register, 39);
    assert_eq!(vs.vars[0].name, "globalScalars");
    assert_eq!(vs.version_token(), Some(0xFFFE_0300));
    let info = vs.validate().expect("vs blob validates");
    assert_eq!((info.major, info.minor), (3, 0));
    assert_eq!(info.instr_count, 1);
    assert_eq!(info.end_offset, vs.bytecode.len());

    let ps = &pack.ps_programs[0];
    assert_eq!(ps.stage, Stage::Pixel);
    assert!(ps.vars.is_empty());
    ps.validate().expect("ps blob validates");

    assert_eq!(pack.shared_params.len(), 1);
    let p = &pack.shared_params[0];
    assert_eq!(p.kind(), ParamKind::Vec4);
    assert!(!p.is_array());
    assert_eq!(p.defaults_f32(), vec![1.0, 1.0, 1.0, 1.0]);
    assert_eq!(p.annotations.len(), 1);
    assert_eq!(p.annotations[0].name, "UIName");
    assert_eq!(
        p.annotations[0].value,
        AnnotationValue::Text("Global Scalars".to_owned())
    );

    assert_eq!(pack.material_params.len(), 1);
    assert_eq!(pack.material_params[0].kind(), ParamKind::Bool);
    assert_eq!(pack.param("switchOn").unwrap().type_code, 7);
    assert!(pack.param("missing").is_none());

    assert_eq!(pack.techniques.len(), 1);
    let tech = &pack.techniques[0];
    assert_eq!(tech.name, "draw");
    assert_eq!(tech.passes.len(), 1);
    assert_eq!(tech.passes[0].vs_index, 0);
    assert_eq!(tech.passes[0].ps_index, Some(0));
    assert_eq!(tech.passes[0].states.len(), 1);
    assert_eq!(tech.passes[0].states[0].id, 10);

    assert_eq!(pack.programs().count(), 2);
    assert_eq!(pack.all_params().count(), 2);
}

#[test]
fn rejects_bad_magic() {
    let mut bytes = minimal_container();
    bytes[0] = 0x00;
    let err = ShaderPack::parse(&bytes).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::BadMagic { .. }));
}

#[test]
fn truncation_anywhere_is_an_error_not_a_panic() {
    let bytes = minimal_container();
    // Every prefix except a few structural edge points must fail cleanly.
    for cut in 0..bytes.len() {
        let err = ShaderPack::parse(&bytes[..cut]).unwrap_err();
        assert!(
            matches!(
                err.kind,
                ErrorKind::UnexpectedEof { .. } | ErrorKind::BadMagic { .. }
            ),
            "cut at {cut}: {err:?}"
        );
    }
}

#[test]
fn rejects_unknown_annotation_kind() {
    let mut bytes = minimal_container();
    // Flip the annotation kind byte (the 2 after "UIName") to 9.
    let pos = bytes
        .windows(7)
        .position(|w| w == b"UIName\0")
        .expect("fixture has UIName");
    bytes[pos + 7] = 9;
    let err = ShaderPack::parse(&bytes).unwrap_err();
    assert_eq!(err.kind, ErrorKind::UnknownAnnotationType { code: 9 });
}

#[test]
fn zero_ps_count_means_no_pixel_programs() {
    let mut bytes = minimal_container();
    // n_ps_raw sits right after the vs blob; find it via the vs size field.
    let vs_len = tiny_blob(0xFFFE_0300).len();
    // magic(4) + n_vs(1) + var header(1+1+1+2) + sized name(1+13+1) + sizes(4)
    let name_off = 4 + 1 + 1 + 1 + 1 + 2;
    let ps_count_at = name_off + 1 + 14 + 4 + vs_len;
    assert_eq!(bytes[ps_count_at], 2);
    bytes[ps_count_at] = 0;
    // Drop the ps fragment bytes too so the rest still aligns: the ps
    // fragment is 1 (nvars) + 4 (sizes) + blob bytes long.
    let frag_len = 1 + 4 + tiny_blob(0xFFFF_0300).len();
    bytes.drain(ps_count_at + 1 + 5..ps_count_at + 1 + 5 + frag_len);
    let pack = ShaderPack::parse(&bytes).expect("zero-ps container parses");
    assert!(pack.ps_programs.is_empty());
}

#[test]
fn bytecode_walk_covers_comments_and_errors() {
    // Comment token: low word 0xFFFE, high word = 1 dword follows.
    let mut blob = Vec::new();
    u32(&mut blob, 0xFFFE_0200); // vs_2_0 is accepted, not just 3_0
    u32(&mut blob, 0x0001_FFFE);
    u32(&mut blob, 0x4241_5443); // "CTAB" little-endian
    u32(&mut blob, bytecode::END);
    let info = bytecode::validate(&blob).expect("comment blob validates");
    assert_eq!((info.major, info.minor), (2, 0));
    assert_eq!(info.instr_count, 0);
    assert!(info.has_ctab);
    assert_eq!(bytecode::find_fourcc(&blob, *b"CTAB"), Some(8));

    assert_eq!(
        bytecode::validate(&[]),
        Err(bytecode::BytecodeError::TooShort)
    );
    assert_eq!(
        bytecode::validate(&[0, 1, 2]),
        Err(bytecode::BytecodeError::TooShort)
    );
    let bad = [0x78, 0x56, 0x34, 0x12, 0xFF, 0xFF, 0x00, 0x00];
    assert_eq!(
        bytecode::validate(&bad),
        Err(bytecode::BytecodeError::BadVersion(0x1234_5678))
    );
    let mut no_end = tiny_blob(0xFFFE_0300);
    no_end.truncate(no_end.len() - 4);
    assert_eq!(
        bytecode::validate(&no_end),
        Err(bytecode::BytecodeError::MissingEnd)
    );
    // Comment claims 5 dwords but only 1 follows.
    let mut trunc = vec![];
    u32(&mut trunc, 0xFFFE_0300);
    u32(&mut trunc, 0x0005_FFFE);
    u32(&mut trunc, 0);
    assert!(matches!(
        bytecode::validate(&trunc),
        Err(bytecode::BytecodeError::TruncatedComment { .. })
    ));
}

#[test]
fn param_helpers_behave() {
    let bytes = minimal_container();
    let pack = ShaderPack::parse(&bytes).unwrap();
    let p = &pack.shared_params[0];
    assert_eq!(p.sampler_states().len(), 2); // 4 dwords pair up
    assert_eq!(pack.material_params[0].defaults, vec![0]);
}

#[test]
fn dcl_parses_mask_and_channels() {
    let decl =
        dcl::parse_dcl("89 ; see header for bit values\n; position diffuse texcoord0 normal\n")
            .expect("sample dcl parses");
    assert_eq!(decl.mask, 89);
    assert_eq!(
        decl.channels,
        vec!["position", "diffuse", "texcoord0", "normal"]
    );
    let bare = dcl::parse_dcl("1 ; see grcore/fvfchannels.h for bit values\n; \n").unwrap();
    assert_eq!(bare.mask, 1);
    assert!(bare.channels.is_empty());
    assert_eq!(dcl::parse_dcl(""), Err(dcl::DclError::Empty));
    assert!(matches!(
        dcl::parse_dcl("zz ; x"),
        Err(dcl::DclError::BadMask(_))
    ));
}

#[test]
fn sps_parses_preset_blocks() {
    let text = "shader gta_default\n__rage_drawbucket {\n\tint 1\n}\nSpecularColor {\n\tfloat 1 0.5 0.25\n}\n";
    let preset = sps::parse_preset(text).expect("sample preset parses");
    assert_eq!(preset.shader, "gta_default");
    assert_eq!(preset.overrides.len(), 2);
    assert_eq!(preset.overrides[0].name, "__rage_drawbucket");
    assert_eq!(preset.overrides[0].type_name, "int");
    assert_eq!(preset.overrides[0].values, "1");
    assert_eq!(preset.overrides[1].values, "1 0.5 0.25");
    assert_eq!(sps::parse_preset(""), Err(sps::SpsError::MissingHeader));
    assert!(matches!(
        sps::parse_preset("shader x\nbroken {\nint 1\n"),
        Err(sps::SpsError::Malformed { .. })
    ));
}
