//! Урок 179. Последовательность участков изображения: разбиение на неперекрывающиеся квадраты.
//! Связь с принятой терминологией: Патчи изображения для ViT.

/// Делит квадратное изображение на неперекрывающиеся квадратные патчи.
/// Сторона патча задаётся во время выполнения: число патчей и пикселей в каждом из них
/// зависит от `side`, поэтому результат хранится в векторах.

pub fn extract_nonoverlapping_square_patches_from_square_image<const N: usize>(
    image: &[[f64; N]; N],
    side: usize,
) -> Result<Vec<Vec<f64>>, &'static str> {
    let height: usize = N;
    if side == 0 || height == 0 || height % side != 0 {
        return Err("размер патча должен делить сторону непустого изображения");
    }
    let mut image_patches: Vec<Vec<f64>> = Vec::new();
    for top in (0..height).step_by(side) {
        for column_start in (0..height).step_by(side) {
            let mut patch: Vec<f64> = Vec::new();
            for row in &image[top..top + side] {
                patch.extend_from_slice(&row[column_start..column_start + side]);
            }
            image_patches.push(patch);
        }
    }
    Ok(image_patches)
}
#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем, что квадратные участки изображения перечисляются строка за строкой.
    fn image_blocks_are_read_row_by_row() {
        let image: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
        assert_eq!(
            super::extract_nonoverlapping_square_patches_from_square_image(&image, 1).unwrap(),
            vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]]
        );
        assert_eq!(
            super::extract_nonoverlapping_square_patches_from_square_image(&image, 2).unwrap(),
            vec![vec![1.0, 2.0, 3.0, 4.0]]
        );
    }
}
