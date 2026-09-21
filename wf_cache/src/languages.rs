use crate::error::CacheResult;
use binrw::BinRead;
use binrw::binrw;
use indexmap::IndexMap;
use std::fmt::Debug;
use std::io::Read;
use std::io::Seek;

#[binrw]
#[br(little)]
#[derive(Debug)]
pub struct LanguagesBin {
    pub hash: [u8; 16],
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub suffix_count: u32,
    #[br(count = suffix_count)]
    pub suffixes: Vec<LenU8>,

    pub dict_bin: LenU8,
    pub path_count: u32,

    #[br(count = path_count)]
    pub paths: Vec<PathE>,
}

#[binrw]
#[derive(Debug)]
pub struct PathE {
    pub path: LenU8,
    pub chunk: LenU8,
    pub label_count: u32,

    #[br(count = label_count)]
    pub labels: Vec<LabelE>,
}

#[binrw]
#[derive(Debug)]
pub struct LabelE {
    pub name: LenU8,
    pub offset: u32,
    pub size: u16,
    pub flags: u16,
}

#[binrw]
// #[br(little)]
// #[derive(Debug)]
pub struct LenU8 {
    pub len: u32,
    #[br(count = len)]
    pub data: Vec<u8>,
}

impl Debug for LenU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if len
        f.debug_struct("LenU8")
            .field("len", &self.len)
            .field("data", &"[...]")
            .finish()
    }
}

fn read_u32_dyn_le<R: Read>(reader: &mut R) -> u32 {
    let mut value = 0_u32;
    let mut buf: [u8; 1] = [0_u8];
    let mut shift = 0;
    while shift < 28 {
        let r = reader.read(&mut buf).unwrap();
        assert_eq!(r, 1);

        value |= ((buf[0] & 0x7f) as u32) << shift;
        if buf[0] & 0x80 == 0 {
            // break;
            return value;
        }
        shift += 7;
    }
    let r = reader.read(&mut buf).unwrap();
    assert!(r == 1);
    assert!(buf[0] <= 0xf);
    value |= (buf[0] as u32) << shift;
    value
}

pub fn decode<R: Read + Seek>(reader: &mut R) -> CacheResult<IndexMap<String, String>> {
    let mut m = IndexMap::new();

    let s = LanguagesBin::read(reader)?;
    let dd = zstd::dict::DecoderDictionary::copy(&s.dict_bin.data);
    for ele in s.paths {
        let s = String::from_utf8_lossy(&ele.path.data);
        for label in &ele.labels {
            let l = String::from_utf8_lossy(&label.name.data);
            let k = format!("{}{}", s, l);
            let slice =
                &ele.chunk.data[label.offset as usize..(label.offset + label.size as u32) as usize];
            if label.size == 0 {
                println!("warn {} size is 0", k);
                // dbg!(&label);
            }
            let v = if label.flags & 0x200 != 0 {
                let mut reader = std::io::Cursor::new(slice);
                let len = read_u32_dyn_le(&mut reader) as usize;

                let mut buf = vec![0_u8; len as usize];
                let mut r = zstd::Decoder::with_prepared_dictionary(&mut reader, &dd)?;
                r.set_parameter(zstd::zstd_safe::DParameter::Format(
                    zstd::zstd_safe::FrameFormat::Magicless,
                ))?;
                r.read_exact(&mut buf)?;
                String::from_utf8_lossy(&buf).to_string()
            } else {
                String::from_utf8_lossy(slice).to_string()
            };

            m.insert(k, v);
        }
    }
    Ok(m)
}
