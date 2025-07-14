use std::fs;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_results(
    path: &Path,
    obsids: &Vec<usize>,
    results: &mut Vec<Vec<f64>>,
) -> std::io::Result<()> {
    // Check if directory exists
    if let Some(parent_dir) = path.parent() {
        // If it doesn't exist, create dir
        fs::create_dir_all(parent_dir)?;
    }

    // Create file
    let mut file = fs::File::create(path)?;

    let num_obs = obsids.len();

    // Pad vectors
    let mut max_len = 0;
    for i in 0..num_obs {
        let len = results[i].len();
        if len > max_len {
            max_len = len
        }
    }

    for i in 0..num_obs {
        if results[i].len() < max_len {
            // Calculate how many NaNs are needed for this specific vector
            let num_nans_to_add = max_len - results[i].len();

            // Extend the current vector with the required number of NaNs
            results[i].extend(std::iter::repeat(f64::NAN).take(num_nans_to_add));
        }
    }

    for i in 0..num_obs {
        let smoothness_strings: Vec<String> = results[i]
            .iter()
            .map(|&val| format!("{:.10}", val))
            .collect();

        let smoothness_line = smoothness_strings.join(" ");

        writeln!(file, "{} {}", obsids[i], smoothness_line)?;
    }

    Ok(())
}
