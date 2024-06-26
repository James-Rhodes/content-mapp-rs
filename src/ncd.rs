// Named after the Normalized Compressed Distance concept from this paper https://aclanthology.org/2023.findings-acl.426.pdf
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

use crate::cache::CompressedByteCache;

#[derive(Debug)]
pub struct NcdIdMapping {
    pub path_idx: usize,
    pub ncd_value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NormalizedCompressedDistance {
    pub file_path: PathBuf,
    pub ncd_value: f64,
}

fn get_compressed_byte_count(buf: &[u8]) -> Result<usize> {
    let receive = Vec::with_capacity(buf.len());
    // Cant use fast compression (Compression::fast()) as it makes the results worse
    let mut e = ZlibEncoder::new(receive, Compression::default());
    e.write_all(buf)?;
    let compressed_bytes = e.finish()?;
    Ok(compressed_bytes.len())
}

pub fn ncds(curr_path: &PathBuf, paths: &[PathBuf]) -> Result<Vec<NcdIdMapping>> {
    let mut buf = Vec::with_capacity(8 * 1024);
    let f = std::fs::File::open(curr_path)?;
    let mut reader = std::io::BufReader::new(f);

    let cx_file_len = std::io::copy(&mut reader, &mut buf)? as usize;

    let mut results = Vec::with_capacity(paths.len());

    let cx = get_compressed_byte_count(&buf)?;
    for (idx, path) in paths.iter().enumerate() {
        if curr_path == path {
            continue;
        }
        let f = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(f);
        std::io::copy(&mut reader, &mut buf)?;

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
    haystack: &[PathBuf],
) -> Result<Vec<NcdIdMapping>> {
    let mut res = ncds(needle, haystack)?;
    res.sort_by(|a, b| a.ncd_value.partial_cmp(&b.ncd_value).unwrap());
    Ok(res.into_iter().take(n).collect())
}

pub fn get_n_most_similar_files(
    n: usize,
    needle: &PathBuf,
    haystack: &[PathBuf],
) -> Result<Vec<NormalizedCompressedDistance>> {
    let res = get_n_most_similar_files_by_id(n, needle, haystack)?;
    Ok(res
        .iter()
        .map(|id_map| NormalizedCompressedDistance {
            file_path: haystack[id_map.path_idx].to_path_buf(),
            ncd_value: id_map.ncd_value,
        })
        .collect())
}

pub fn ncds_cached(
    cache: &CompressedByteCache,
    curr_path: &PathBuf,
    paths: &[PathBuf],
) -> Result<Vec<NcdIdMapping>> {
    let mut buf = Vec::with_capacity(8 * 1024);
    let f = std::fs::File::open(curr_path)?;
    let mut reader = std::io::BufReader::new(f);

    let cx_file_len = std::io::copy(&mut reader, &mut buf)? as usize;

    let mut results = Vec::with_capacity(paths.len());

    let cx = cache.get_or(curr_path, || get_compressed_byte_count(&buf))?;
    for (idx, path) in paths.iter().enumerate() {
        if curr_path == path {
            continue;
        }
        let f = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(f);
        std::io::copy(&mut reader, &mut buf)?;

        let cy = cache.get_or(path, || get_compressed_byte_count(&buf[cx_file_len..]))?;
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

pub fn get_n_most_similar_files_cached(
    cache: &CompressedByteCache,
    n: usize,
    curr_path: &PathBuf,
    paths: &[PathBuf],
) -> Result<Vec<NormalizedCompressedDistance>> {
    let mut res = ncds_cached(cache, curr_path, paths)?;
    res.sort_by(|a, b| a.ncd_value.partial_cmp(&b.ncd_value).unwrap());
    Ok(res
        .into_iter()
        .take(n)
        .map(|id_map| NormalizedCompressedDistance {
            file_path: paths[id_map.path_idx].to_path_buf(),
            ncd_value: id_map.ncd_value,
        })
        .collect())
}
