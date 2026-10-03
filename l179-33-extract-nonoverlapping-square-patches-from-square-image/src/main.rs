// Урок 33.1. Последовательность участков изображения: разбиение на неперекрывающиеся квадраты.
// Связь с принятой терминологией: Разбиение квадратного изображения на неперекрывающиеся патчи ViT.
// Зачем здесь эта тема: ViT использует Transformer для изображения, значит сначала превращает
//   пиксели в последовательность элементов.
// Почему код устроен так: Режем квадратную картинку на одинаковые неперекрывающиеся патчи с
//   проверкой формы.
// Представь: Картинку 4×4 можно разрезать на четыре патча 2×2 и обрабатывать их как
//   последовательность.
// Изображение превращается в последовательность неперекрывающихся патчей.

use l179_33_extract_nonoverlapping_square_patches_from_square_image::extract_nonoverlapping_square_patches_from_square_image;

fn main() {
    let image: [[f64; 4]; 4] = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    assert_eq!(
        std::convert::TryInto::<[[f64; 4]; 4]>::try_into(
            extract_nonoverlapping_square_patches_from_square_image(&image, 2)
                .unwrap()
                .into_iter()
                .map(|patch| {
                    patch
                        .try_into()
                        .expect("ожидалось четыре пикселя в патче 2×2")
                })
                .collect::<Vec<[f64; 4]>>(),
        )
        .expect("ожидалось четыре патча 2×2 из изображения 4×4")[0],
        [1.0, 2.0, 5.0, 6.0]
    );

    plot_square_image_before_splitting_into_blocks(&image);
}

fn plot_square_image_before_splitting_into_blocks(image: &[[f64; 4]; 4]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "image-patches",
        "Исходное изображение 4x4",
        &image.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
