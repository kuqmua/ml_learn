//! Патчи изображения для ViT.

/// Делит квадратное изображение на неперекрывающиеся квадратные патчи.
pub fn patches(image: &[Vec<f64>], side: usize) -> Result<Vec<Vec<f64>>, &'static str> {
    let height = image.len();
    if side == 0 || height == 0 || image.iter().any(|row| row.len() != height) || height % side != 0
    {
        return Err("неверная форма изображения или патча");
    }
    let mut result = Vec::new();
    for top in (0..height).step_by(side) {
        for left in (0..height).step_by(side) {
            let mut patch = Vec::new();
            for row in &image[top..top + side] {
                patch.extend_from_slice(&row[left..left + side]);
            }
            result.push(patch);
        }
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::patches;
    #[test]
    fn patch_order() {
        let image = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert_eq!(
            patches(&image, 1).unwrap(),
            vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]]
        );
        assert_eq!(patches(&image, 2).unwrap(), vec![vec![1.0, 2.0, 3.0, 4.0]]);
    }
}
