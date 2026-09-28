// Урок 33.1. Патчи изображения для ViT.
// Изображение превращается в последовательность неперекрывающихся патчей.

use part_175_lesson_33_vit_patches::patches;
fn main() {
    let image = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0],
    ];
    // Участок изображения, передаваемый трансформеру, называют visual token.
    let image_patches = patches(&image, 2).unwrap();
    assert_eq!(image_patches.len(), 4);
    println!("4 патча 2x2: {image_patches:?}");
    visualize(&image);
}

fn visualize(image: &[Vec<f64>]) {
    let path = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "image-patches",
        "Исходное изображение 4x4",
        image,
    )
    .expect("график");
    println!("график: {}", path.display());
}
