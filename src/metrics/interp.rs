use ndarray::prelude::*;
use num_traits::{Float, FromPrimitive};
use std::fmt::Debug;

/// Trait to define linear interpolation function for Array1s
pub trait InterpolateNans<F> {
    /// Linearly interpolates over NaN values in-place.
    fn interp_nans_inplace(&mut self);
}

impl<F> InterpolateNans<F> for Array1<F>
where
    F: Float + FromPrimitive + Debug,
{
    fn interp_nans_inplace(&mut self) {
        let mut i = 0;
        while i < self.len() {
            if self[i].is_nan() {
                let gap_start_index = i;

                let mut gap_end_index = i + 1;
                while gap_end_index < self.len() && self[gap_end_index].is_nan() {
                    gap_end_index += 1;
                }

                let y0 = if gap_start_index == 0 {
                    if let Some(first_valid) = self.iter().find(|v| !v.is_nan()) {
                        *first_valid
                    } else {
                        return;
                    }
                } else {
                    self[gap_start_index - 1]
                };

                let y1 = if gap_end_index >= self.len() {
                    y0
                } else {
                    self[gap_end_index]
                };

                let gap_len = F::from_usize(gap_end_index - gap_start_index + 1).unwrap();

                for j in gap_start_index..gap_end_index {
                    let step = F::from_usize(j - gap_start_index + 1).unwrap();
                    self[j] = y0 + (y1 - y0) * (step / gap_len);
                }
                i = gap_end_index;
            } else {
                i += 1;
            }
        }
    }
}
