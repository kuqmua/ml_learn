//! Симметричное дерево.

/// Все узлы одного уровня проверяют один и тот же признак и порог.
#[derive(Debug)]
pub struct ObliviousTree {
    pub splits: Vec<(usize, f64)>,
    pub leaves: Vec<f64>,
}

impl ObliviousTree {
    /// Биты результатов проверок образуют индекс листа.
    pub fn predict(&self, features: &[f64]) -> Result<f64, &'static str> {
        if self.leaves.len() != 1 << self.splits.len() {
            return Err("число листьев должно быть 2^depth");
        }
        let mut leaf = 0;
        for &(feature, threshold) in &self.splits {
            let value = *features.get(feature).ok_or("нет признака")?;
            leaf = (leaf << 1) | usize::from(value > threshold);
        }
        Ok(self.leaves[leaf])
    }
}

#[cfg(test)]
mod tests {
    use super::ObliviousTree;
    #[test]
    fn all_four_leaves_and_invalid_shape() {
        let tree = ObliviousTree {
            splits: vec![(0, 0.5), (1, 0.5)],
            leaves: vec![0.0, 1.0, 2.0, 3.0],
        };
        assert_eq!(tree.predict(&[0.0, 0.0]), Ok(0.0));
        assert_eq!(tree.predict(&[0.0, 1.0]), Ok(1.0));
        assert_eq!(tree.predict(&[1.0, 0.0]), Ok(2.0));
        assert_eq!(tree.predict(&[1.0, 1.0]), Ok(3.0));
        assert!(tree.predict(&[1.0]).is_err());
    }
}
