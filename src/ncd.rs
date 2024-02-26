// Named after the Normalized Compressed Distance concept from this paper https://aclanthology.org/2023.findings-acl.426.pdf
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::{io::Write, path::PathBuf};

use std::error::Error;

#[derive(Debug)]
struct NcdIdMapping {
    pub path_idx: usize,
    pub ncd_value: f64,
}

#[derive(Debug)]
pub struct NormalizedCompressedDistance {
    pub path: PathBuf,
    pub ncd_value: f64,
}

fn get_compressed_byte_count(buf: &[u8]) -> Result<usize, Box<dyn Error>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(buf)?;
    let compressed_bytes = e.finish()?;
    Ok(compressed_bytes.len())
}

pub fn ncds(
    curr_path: &PathBuf,
    paths: &Vec<PathBuf>,
) -> Result<Vec<NcdIdMapping>, Box<dyn Error>> {
    let mut buf = Vec::with_capacity(8 * 1024);
    let f = std::fs::File::open(curr_path)?;
    let mut reader = std::io::BufReader::new(f);

    let cx_file_len = std::io::copy(&mut reader, &mut buf)? as usize;
    println!("vec len: {}, read bytes: {}", buf.len(), cx_file_len);

    let mut results = Vec::with_capacity(paths.len());

    let cx = get_compressed_byte_count(&buf)?;
    for (idx, path) in paths.iter().enumerate() {
        if curr_path == path {
            continue;
        }
        let f = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(f);
        let cy_file_len = std::io::copy(&mut reader, &mut buf)?;
        println!(
            "cx len: {}, cy len: {}, buf len: {}",
            cx_file_len,
            cy_file_len,
            buf.len()
        );

        let cy = get_compressed_byte_count(&buf[cx_file_len..])?;
        let cxy = get_compressed_byte_count(&buf)?;

        let ncd = (cxy - cx.min(cy)) as f64 / cx.max(cy) as f64;

        let ncd = NcdIdMapping {
            path_idx: idx,
            ncd_value: ncd,
        };
        results.push(ncd);

        buf.truncate(cx_file_len);
    }

    Ok(results)
}

fn get_n_most_similar_files_by_id(
    n: usize,
    needle: &PathBuf,
    haystack: &Vec<PathBuf>,
) -> Result<Vec<NcdIdMapping>, Box<dyn Error>> {
    let mut res = ncds(needle, haystack)?;
    res.sort_by(|a, b| a.ncd_value.partial_cmp(&b.ncd_value).unwrap());
    res.truncate(n);
    Ok(res)
}

pub fn get_n_most_similar_files(
    n: usize,
    needle: &PathBuf,
    haystack: &Vec<PathBuf>,
) -> Result<Vec<NormalizedCompressedDistance>, Box<dyn Error>> {
    let res = get_n_most_similar_files_by_id(n, needle, haystack)?;
    Ok(res
        .iter()
        .map(|id_map| NormalizedCompressedDistance {
            path: haystack[id_map.path_idx].to_path_buf(),
            ncd_value: id_map.ncd_value,
        })
        .collect())
}
