//! Schema-driven decoding of object bodies into value trees.
//!
//! [`decode_object`] walks an [`ObjectEntry`](crate::container::ObjectEntry)
//! with a [`Schema`](crate::schema::Schema): the header fields, then the
//! type-specific fields. Bytes the schema does not describe are kept in
//! [`DecodedObject::trailing`] rather than failing; an unknown type id keeps
//! the whole body there with [`DecodedObject::type_name`] set to `None`.

use crate::{
    Cursor, Error, ErrorKind,
    container::ObjectEntry,
    schema::{CountWidth, FieldDef, FieldKind, IntWidth, Schema},
};

/// A decoded field value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Unsigned 8-bit integer.
    U8(u8),
    /// Unsigned 16-bit integer.
    U16(u16),
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Signed 8-bit integer.
    I8(i8),
    /// Signed 16-bit integer.
    I16(i16),
    /// Signed 32-bit integer.
    I32(i32),
    /// 32-bit float.
    F32(f32),
    /// 32-bit name hash; see [`crate::hash`].
    Hash(u32),
    /// Counted string.
    Text(String),
    /// Array elements in order; struct elements are [`Value::Fields`].
    Array(Vec<Value>),
    /// Named sub-fields (struct element or optional-bitfield group).
    Fields(Vec<DecodedField>),
    /// Enum value: raw number plus the schema name when known.
    Enum(EnumValue),
    /// Bytes of explicitly unknown layout (`Rest` fields).
    Bytes(Vec<u8>),
}

/// A decoded enum field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    /// Stored number.
    pub raw: i64,
    /// Schema name, or `None` when the value is not listed.
    pub name: Option<&'static str>,
}

/// One named field of a decoded header, body or struct element.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedField {
    /// Field name from the schema.
    pub name: String,
    /// Decoded value.
    pub value: Value,
}

/// An object decoded with its schema.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedObject {
    /// Object name from the directory.
    pub name: String,
    /// Leading type-id byte.
    pub type_id: u8,
    /// Type name, or `None` for an unknown type id.
    pub type_name: Option<&'static str>,
    /// Decoded header fields.
    pub header: Vec<DecodedField>,
    /// Decoded body fields.
    pub body: Vec<DecodedField>,
    /// Body bytes past what the schema describes (empty when the schema
    /// fits exactly).
    pub trailing: Vec<u8>,
}

/// Decode one directory entry with `schema`.
///
/// Never fails on unknown content: unknown type ids and bytes past the
/// schema land in [`DecodedObject::trailing`]. Fails only when the bytes end
/// mid-field or a string is not UTF-8.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn decode_object(schema: &Schema, entry: &ObjectEntry) -> Result<DecodedObject, Error> {
    let base = u64::from(entry.offset());
    let mut cur = Cursor::new(entry.data());
    let type_id = cur.u8("type id")?;
    let _name_offset = cur.u32("name offset")?;
    let mut header = Vec::new();
    decode_fields(&mut cur, base, &schema.header(), &mut header)?;
    let mut body = Vec::new();
    let type_name = schema.type_name(type_id);
    if let Some(fields) = schema.type_fields(type_id) {
        decode_fields(&mut cur, base, &fields, &mut body)?;
    }
    let trailing = cur
        .bytes(cur.remaining(), "trailing")
        .unwrap_or(&[])
        .to_vec();
    Ok(DecodedObject {
        name: entry.name().to_string(),
        type_id,
        type_name,
        header,
        body,
        trailing,
    })
}

fn decode_fields(
    cur: &mut Cursor,
    base: u64,
    fields: &[FieldDef],
    out: &mut Vec<DecodedField>,
) -> Result<(), Error> {
    for field in fields {
        let value = decode_field(cur, base, &field.kind)?;
        out.push(DecodedField {
            name: field.name.to_string(),
            value,
        });
    }
    Ok(())
}

fn decode_element(cur: &mut Cursor, base: u64, element: &[FieldDef]) -> Result<Value, Error> {
    if element.len() == 1 && element[0].name.is_empty() {
        return decode_field(cur, base, &element[0].kind);
    }
    let mut fields = Vec::with_capacity(element.len());
    decode_fields(cur, base, element, &mut fields)?;
    Ok(Value::Fields(fields))
}

fn decode_field(cur: &mut Cursor, base: u64, kind: &FieldKind) -> Result<Value, Error> {
    Ok(match kind {
        FieldKind::U8 => Value::U8(cur.u8("u8")?),
        FieldKind::U16 => Value::U16(cur.u16("u16")?),
        FieldKind::U32 => Value::U32(cur.u32("u32")?),
        FieldKind::I8 => Value::I8(cur.i8("i8")?),
        FieldKind::I16 => Value::I16(cur.i16("i16")?),
        FieldKind::I32 => Value::I32(cur.i32("i32")?),
        FieldKind::F32 => Value::F32(cur.f32("f32")?),
        FieldKind::Hash => Value::Hash(cur.u32("hash")?),
        FieldKind::Text(width) => {
            let len = read_count(cur, *width)? as usize;
            let at = base + cur.pos() as u64;
            let bytes = cur.bytes(len, "string")?;
            Value::Text(
                core::str::from_utf8(bytes)
                    .map_err(|_| Error::new(at, ErrorKind::Invalid, "string field not UTF-8"))?
                    .to_string(),
            )
        }
        FieldKind::Array { count, element } => {
            let n = read_count(cur, *count)? as usize;
            let mut items = Vec::with_capacity(n.min(cur.remaining() + 1));
            for _ in 0..n {
                items.push(decode_element(cur, base, element)?);
            }
            Value::Array(items)
        }
        FieldKind::FixedArray { count, element } => {
            let mut items = Vec::with_capacity(*count);
            for _ in 0..*count {
                items.push(decode_element(cur, base, element)?);
            }
            Value::Array(items)
        }
        FieldKind::OptionalBitfield(children) => {
            let mask = cur.u32("presence mask")?;
            let mut present = Vec::new();
            for (i, child) in children.iter().enumerate() {
                if i < 32 && mask & (1u32 << i) != 0 {
                    let value = decode_field(cur, base, &child.kind)?;
                    present.push(DecodedField {
                        name: child.name.to_string(),
                        value,
                    });
                }
            }
            Value::Fields(present)
        }
        FieldKind::Enum {
            base: width,
            values,
        } => {
            let raw = match width {
                IntWidth::U8 => i64::from(cur.u8("enum")?),
            };
            let name = values.iter().find(|(v, _)| *v == raw).map(|(_, n)| *n);
            Value::Enum(EnumValue { raw, name })
        }
        FieldKind::Rest => {
            let rest = cur.bytes(cur.remaining(), "rest").unwrap_or(&[]).to_vec();
            Value::Bytes(rest)
        }
    })
}

fn read_count(cur: &mut Cursor, width: CountWidth) -> Result<u32, Error> {
    match width {
        CountWidth::U8 => Ok(u32::from(cur.u8("count")?)),
        CountWidth::U16 => Ok(u32::from(cur.u16("count")?)),
        CountWidth::U32 => cur.u32("count"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::MetaFile;

    fn entry_of(buf: &[u8]) -> ObjectEntry<'_> {
        // tiny_file layout (covered by container tests): blob starts at
        // byte 8, the object sits at blob offset 1 with size 8.
        let f = MetaFile::parse(buf).unwrap();
        assert_eq!(f.objects().len(), 1);
        let o = f.objects()[0];
        // Re-express against `buf` so the entry outlives `f`.
        let start = 8 + o.offset() as usize;
        ObjectEntry::for_test(o.name(), o.offset(), &buf[start..start + o.size() as usize])
    }

    #[test]
    fn sounds_simple_sound_decodes() {
        // Hand-built audSimpleSound: type 12, zero name offset, flags,
        // unknown u16, empty presence mask, wave slot, archive, sound.
        let mut obj = vec![12u8];
        obj.extend_from_slice(&0u32.to_le_bytes());
        obj.extend_from_slice(&0x10u32.to_le_bytes());
        obj.extend_from_slice(&0u16.to_le_bytes());
        obj.extend_from_slice(&0u32.to_le_bytes());
        obj.extend_from_slice(&7u32.to_le_bytes());
        obj.extend_from_slice(&0xAAABu32.to_le_bytes());
        obj.extend_from_slice(&0xCCCCu32.to_le_bytes());
        let entry = ObjectEntry::for_test("T", 0, &obj);
        let d = decode_object(&Schema::Sounds, &entry).unwrap();
        assert_eq!(d.type_name, Some("audSimpleSound"));
        assert!(d.trailing.is_empty());
        assert_eq!(d.body.len(), 3);
        assert_eq!(d.body[0].value, Value::U32(7));
        assert_eq!(d.body[1].value, Value::Hash(0xAAAB));
        // header group present but empty
        assert_eq!(d.header.len(), 3);
        assert_eq!(d.header[2].value, Value::Fields(vec![]));
    }

    #[test]
    fn optional_bitfield_picks_present_fields() {
        // type 6 wrapper with volume + category bits set (bits 0 and 13).
        let mut obj = vec![6u8];
        obj.extend_from_slice(&0u32.to_le_bytes());
        obj.extend_from_slice(&0u32.to_le_bytes());
        obj.extend_from_slice(&0u16.to_le_bytes());
        obj.extend_from_slice(&0x2001u32.to_le_bytes());
        obj.extend_from_slice(&(-6i16).to_le_bytes());
        obj.extend_from_slice(&0x1234u32.to_le_bytes());
        obj.extend_from_slice(&0x7777u32.to_le_bytes());
        let entry = ObjectEntry::for_test("W", 0, &obj);
        let d = decode_object(&Schema::Sounds, &entry).unwrap();
        assert!(d.trailing.is_empty());
        match &d.header[2].value {
            Value::Fields(fs) => {
                assert_eq!(fs.len(), 2);
                assert_eq!(fs[0].name, "volume");
                assert_eq!(fs[0].value, Value::I16(-6));
                assert_eq!(fs[1].name, "category");
            }
            v => panic!("unexpected {v:?}"),
        }
    }

    #[test]
    fn unknown_type_keeps_body_as_trailing() {
        let obj = [250u8, 0, 0, 0, 0, 1, 2, 3];
        let entry = ObjectEntry::for_test("U", 0, &obj);
        // categories header is empty, so only type id + name offset consumed.
        let d = decode_object(&Schema::Categories, &entry).unwrap();
        assert_eq!(d.type_name, None);
        assert_eq!(d.trailing, vec![1, 2, 3]);
    }

    #[test]
    fn truncated_field_is_an_error() {
        // claims a u32 wave slot but ends early
        let obj = [12u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2];
        let entry = ObjectEntry::for_test("T", 0, &obj);
        assert_eq!(
            decode_object(&Schema::Sounds, &entry).unwrap_err().kind(),
            ErrorKind::Truncated
        );
    }

    #[test]
    fn entry_from_container_decodes() {
        let buf = crate::container::tests::tiny_file();
        let entry = entry_of(&buf);
        // type 1 under effects schema: empty body, 15-byte header needs
        // 15 bytes but object holds 8 -> truncated.
        assert_eq!(
            decode_object(&Schema::Effects, &entry).unwrap_err().kind(),
            ErrorKind::Truncated
        );
    }
}
