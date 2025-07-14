use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub(crate) fn write_smoothness(
    path: &Path,
    obsids: &Vec<usize>,
    smoothness: &Vec<Vec<f64>>,
) -> std::io::Result<()> {
    // Check if directory exists
    if let Some(parent_dir) = path.parent() {
        // If it doesn't exist, create dir
        fs::create_dir_all(parent_dir);
    }

    let mut file = fs::File::create(path)?;

    let num_obs = obsids.len();

    for i in 0..num_obs {
        let smoothness_strings: Vec<String> = smoothness[i]
            .iter()
            .map(|&val| format!("{:.10}", val))
            .collect();

        let smoothness_line = smoothness_strings.join(" ");

        writeln!(file, "{} {}", obsids[i], smoothness_line)?;
    }

    Ok(())
}
