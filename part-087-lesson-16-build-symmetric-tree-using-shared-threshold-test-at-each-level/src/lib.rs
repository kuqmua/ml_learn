//! Урок 087. Симметричное дерево решений: общая пороговая проверка для всех узлов одного уровня.
//! Связь с принятой терминологией: Симметричное дерево.

/// Все узлы одного уровня проверяют один и тот же признак и порог.
#[derive(Debug)]
pub struct ObliviousTree {
    pub splits: Vec<(usize, f64)>,
    pub leaves: Vec<f64>,
}

impl ObliviousTree {
    /// Биты результатов проверок образуют индекс листа.
    /// Симметричное дерево (oblivious tree): на каждом уровне одна проверка порога; её результат задаёт следующий бит номера листа.
    pub fn predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(
        &self,
        features: &[f64],
    ) -> Result<f64, &'static str> {
        if self.leaves.len() != 1 << self.splits.len() {
            return Err("число листьев должно быть 2^depth");
        }
        let mut leaf: usize = 0;
        lesson_trace::trace_step!(leaf);
        for &(feature, threshold) in &self.splits {
            lesson_trace::trace_step!(feature);
            lesson_trace::trace_step!(threshold);
            let value: f64 = *features.get(feature).ok_or("нет признака")?;
            lesson_trace::trace_step!(value);
            leaf = (leaf << 1) | usize::from(value > threshold);
            lesson_trace::trace_step!(leaf);
        }
        Ok(self.leaves[leaf])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_four_leaves_and_invalid_shape() {
        let tree: super::ObliviousTree = super::ObliviousTree {
            splits: vec![(0, 0.5), (1, 0.5)],
            leaves: vec![0.0, 1.0, 2.0, 3.0],
        };
        assert_eq!(
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[0.0, 0.0]),
            Ok(0.0)
        );
        assert_eq!(
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[0.0, 1.0]),
            Ok(1.0)
        );
        assert_eq!(
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[1.0, 0.0]),
            Ok(2.0)
        );
        assert_eq!(
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[1.0, 1.0]),
            Ok(3.0)
        );
        assert!(
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[1.0])
                .is_err()
        );
    }
}
