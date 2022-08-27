use crate::{Indices};

impl Indices for &[usize] {
    /// Inverts an index, eg. from sort index to ranks.
    /// This is a symmetric operation: any even number of applications
    /// gives the original index, odd number gives the inverted form.
    fn invindex(self) -> Vec<usize> {
        let n = self.len();
        let mut index: Vec<usize> = vec![0; n];
        for (i, &indxpos) in self.iter().enumerate() {
            index[indxpos] = i
        }
        index
    }

    /// Collects values from v in the order given by self index.
    /// When ascending is false, collects in descending order.  
    /// It is used here by msort for ascending or descending sort.   
    fn unindex<T>(self, v: &[T], ascending: bool) -> Vec<T>
    where
        T: Clone,
    {
        if ascending {
            self.iter().map(|&i| v[i].clone()).collect()
        } else {
            self.iter().rev().map(|&i| v[i].clone()).collect()
        }
    }

    /// Complement of an index  (is symmetric) -
    /// .complindex() toggles rank index between ascending/descending.
    /// To toggle sort index between ascending/descending, use the general reversal `revs`:
    /// `ranks.complindex().invindex()` = ranks.invindex().revs()
    fn complindex(self) -> Vec<usize> {
        let n = self.len();
        let mut index: Vec<usize> = vec![0; n];
        for (i, &inx) in self.iter().enumerate() {
            index[i] = n - inx - 1
        }
        index
    }

    /// Pearson's correlation coefficient of two `$[usize]` slices.
    /// When the inputs are ranks, then this gives Spearman's correlation
    /// of the original data. However, in general, any other ordinal measures
    /// could be deployed (not just the ranks).
    fn ucorrelation(self, v: &[usize]) -> f64 {
        let (mut sy, mut sxy, mut sx2, mut sy2) = (0_f64, 0_f64, 0_f64, 0_f64);
        let sx: f64 = self
            .iter()
            .zip(v)
            .map(|(&ux, &uy)| {
                let x = ux as f64;
                let y = uy as f64;
                sy += y;
                sxy += x * y;
                sx2 += x * x;
                sy2 += y * y;
                x
            })
            .sum();
        let nf = self.len() as f64;
        (sxy - sx / nf * sy) / ((sx2 - sx / nf * sx) * (sy2 - sy / nf * sy)).sqrt()
    }

    /// Potentially useful clone-recast of &[usize] to Vec<f64>
    fn indx_to_f64(self) -> Vec<f64> {
        self.iter().map(|&x| x as f64).collect()
    }
}
