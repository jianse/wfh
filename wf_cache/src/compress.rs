use std::ffi::c_void;
use std::fmt::Debug;
use std::io::BufRead;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::ptr::null_mut;

use binrw::BinRead;
use binrw::binrw;

use crate::dir::FileNode;

pub fn get_data_and_decompress_post<R: Read + Seek + BufRead>(
    entry: &FileNode,
    reader: &mut R,
) -> Vec<u8> {
    let mut decompressed_data = vec![0_u8; entry.length as usize];

    // common block size is 0x40000
    let mut buf = [0_u8; 0x40000];
    let mut dec_pos = 0_usize;
    reader
        .seek(SeekFrom::Start(entry.cache_offset as u64))
        .unwrap();

    while dec_pos < entry.length as usize {
        let mut block_len = get_block_length(reader);
        // dbg!(&block_len);
        if block_len.compress_len == 0 && block_len.decompress_len == 0 {
            block_len.compress_len = entry.compressed_size;
            block_len.decompress_len = entry.length;
        }

        // TODO: overflow check

        let is_oodle = is_oodle_block(reader);
        // dbg!(is_oodle);
        reader
            .read_exact(&mut buf[..block_len.compress_len as usize])
            .unwrap();

        // de

        let src = &buf[..block_len.compress_len as usize];
        let dest = &mut decompressed_data[dec_pos..dec_pos + block_len.decompress_len as usize];

        if is_oodle {
            // decompressed_data.extend_from_slice(&buf[..block_len.decompress_len as usize]);
            decompress_oodle(src, dest)
        } else {
            decompress_lz(src, dest)
        }

        dec_pos += block_len.decompress_len as usize;
    }

    decompressed_data
}
#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct BlockLength {
    pub compress_len: u32,
    pub decompress_len: u32,
}

#[inline]
pub fn get_block_length<R: Read + Seek>(reader: &mut R) -> BlockLength {
    let mut block_length = BlockLength::read(reader).unwrap();

    if block_length.compress_len & 0xff000000 != 0x80000000
        || block_length.decompress_len & 0x0f != 0x1
    {
        block_length.compress_len = 0;
        block_length.decompress_len = 0;
        reader.seek_relative(-8).unwrap();
    } else {
        block_length.compress_len = (block_length.compress_len >> 2) & 0xffffff;
        block_length.decompress_len = (block_length.decompress_len >> 5) & 0xffffff;
    }

    block_length
}

fn is_oodle_block<R: Read + Seek>(reader: &mut R) -> bool {
    let mut buf = [0_u8; 1];
    reader.read_exact(&mut buf).unwrap();
    reader.seek(SeekFrom::Current(-1)).unwrap();
    buf[0] == 0x8C
}

fn decompress_oodle(src: &[u8], dest: &mut [u8]) {
    unsafe {
        oodle_sys::OodleLZ_Decompress(
            src.as_ptr() as *const c_void,
            src.len() as isize,
            dest.as_mut_ptr() as *mut c_void,
            dest.len() as isize,
            oodle_sys::OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_No,
            oodle_sys::OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
            oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            null_mut(),
            0,
            None,
            null_mut(),
            null_mut(),
            0,
            oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
        );
    }
}

fn decompress_lz(src: &[u8], dest: &mut [u8]) {
    let r = lzf::decompress(src, dest.len()).unwrap();
    dest.copy_from_slice(r.as_slice());
}

#[cfg(test)]
mod tests {
    use crate::compress::get_block_length;

    #[test]
    fn test_get_data_and_decompress_post() {
        let file = std::fs::File::open("../Packages.bin").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let s = get_block_length(&mut reader);
        println!("{:x?}", s);
    }
}
