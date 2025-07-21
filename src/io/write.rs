use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

pub(crate) fn write_results(
    path: &Path,
    obsids: &[usize],
    results: &mut [Vec<f64>],
) -> std::io::Result<()> {
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    let max_len = results.iter().map(|v| v.len()).max().unwrap_or(0);

    for (obsid, data) in obsids.iter().zip(results.iter_mut()) {
        data.resize(max_len, f64::NAN);
        let line = data
            .iter()
            .map(|&val| format!("{:.10}", val))
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(writer, "{} {}", obsid, line)?;
    }

    Ok(())
}

pub(crate) fn write_results_1D(
    path: &Path,
    obsids: &[usize],
    results: &[f64],
) -> std::io::Result<()> {
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);

    for (obsid, result) in obsids.iter().zip(results.iter()) {
        writeln!(writer, "{} {}", obsid, result)?;
    }

    Ok(())
}
