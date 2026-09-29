//! Урок 175. Последовательность участков изображения: разбиение на неперекрывающиеся квадраты.
//! Связь с принятой терминологией: Патчи изображения для ViT.

/// Делит квадратное изображение на неперекрывающиеся квадратные патчи.
pub fn extract_nonoverlapping_square_patches_from_square_image(
    image: &[Vec<f64>],
    side: usize,
) -> Result<Vec<Vec<f64>>, &'static str> {
    let height: usize = image.len();
    lesson_trace::trace_step!(height);
    if side == 0 || height == 0 || image.iter().any(|row| row.len() != height) || height % side != 0
    {
        return Err("неверная форма изображения или патча");
    }
    let mut result: Vec<Vec<f64>> = Vec::new();
    lesson_trace::trace_step!(result);
    for top in (0..height).step_by(side) {
        lesson_trace::trace_step!(top);
        for left in (0..height).step_by(side) {
            lesson_trace::trace_step!(left);
            let mut patch: Vec<f64> = Vec::new();
            lesson_trace::trace_step!(patch);
            for row in &image[top..top + side] {
                lesson_trace::trace_step!(row);
                patch.extend_from_slice(&row[left..left + side]);
            }
            result.push(patch);
        }
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем, что квадратные участки изображения перечисляются строка за строкой.
    fn image_blocks_are_read_row_by_row() {
        let image: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
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
