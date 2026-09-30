// Урок 33.1. Последовательность участков изображения: разбиение на неперекрывающиеся квадраты.
// Связь с принятой терминологией: Разбиение квадратного изображения на неперекрывающиеся патчи ViT.
// Зачем здесь эта тема: ViT использует Transformer для изображения, значит сначала превращает
//   пиксели в последовательность элементов.
// Почему код устроен так: Режем квадратную картинку на одинаковые неперекрывающиеся патчи с
//   проверкой формы.
// Представь: Картинку 4×4 можно разрезать на четыре патча 2×2 и обрабатывать их как
//   последовательность.
// Изображение превращается в последовательность неперекрывающихся патчей.

use l179_33_create_image_block_sequence_by_splitting_image_into_nonoverlapping_squares::extract_nonoverlapping_square_patches_from_square_image;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    let image: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0],
    ];
    trace_step!(image);
    trace_note!("Участок изображения, передаваемый трансформеру, называют visual token.");
    let image_patches: Vec<Vec<f64>> =
        extract_nonoverlapping_square_patches_from_square_image(&image, 2).unwrap();
    trace_step!(image_patches);
    assert_eq!(image_patches.len(), 4);
    println!("4 патча 2x2: {image_patches:?}");
    disable();
    plot_square_image_before_splitting_into_blocks(&image);
}

fn plot_square_image_before_splitting_into_blocks(image: &[Vec<f64>]) {
    let path: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "image-patches",
        "Исходное изображение 4x4",
        image,
    )
    .expect("график");
    println!("график: {}", path.display());
}
