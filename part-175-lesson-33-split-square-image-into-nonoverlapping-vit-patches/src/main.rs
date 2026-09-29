// Урок 33.1. Разбиение квадратного изображения на неперекрывающиеся патчи ViT.
// Изображение превращается в последовательность неперекрывающихся патчей.

fn main() {
    let image: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0],
    ];
    // Участок изображения, передаваемый трансформеру, называют visual token.
    let image_patches: Vec<Vec<f64>> =
        part_175_lesson_33_split_square_image_into_nonoverlapping_vit_patches::extract_nonoverlapping_square_patches_from_square_image(&image, 2).unwrap();
    assert_eq!(image_patches.len(), 4);
    println!("4 патча 2x2: {image_patches:?}");
    visualize_split_square_image_into_nonoverlapping_vit_patches(&image);
}

fn visualize_split_square_image_into_nonoverlapping_vit_patches(image: &[Vec<f64>]) {
    let path: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "image-patches",
        "Исходное изображение 4x4",
        image,
    )
    .expect("график");
    println!("график: {}", path.display());
}
