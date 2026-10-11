use mnist_data::{ALL_DIGITS, load_labeled_png_images};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TemporaryDataset(PathBuf);
impl TemporaryDataset {
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "mnist-data-test-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        for digit in 0..10 {
            std::fs::create_dir_all(root.join(digit.to_string())).unwrap();
        }
        Self(root)
    }
}
impl Drop for TemporaryDataset {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn write_png(path: &Path, width: u32, color: png::ColorType, brightness: u8) {
    let file = std::fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(file, width, 28);
    encoder.set_color(color);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let channels = if color == png::ColorType::Rgb { 3 } else { 1 };
    writer
        .write_image_data(&vec![brightness; width as usize * 28 * channels])
        .unwrap();
    writer.finish().unwrap();
}
#[test]
fn sorts_png_names_and_returns_digit_labels_and_normalized_pixels() {
    let dataset = TemporaryDataset::new();
    for digit in 0..10 {
        let directory = dataset.0.join(digit.to_string());
        write_png(&directory.join("b.png"), 28, png::ColorType::Grayscale, 255);
        write_png(
            &directory.join("a.PNG"),
            28,
            png::ColorType::Grayscale,
            digit,
        );
        std::fs::write(directory.join("ignored.txt"), "not a PNG").unwrap();
    }
    let images = load_labeled_png_images(&dataset.0).unwrap();
    assert_eq!(images.len(), 20);
    for (index, digit) in ALL_DIGITS.into_iter().enumerate() {
        assert_eq!(images[2 * index], ([index as f64 / 255.0; 784], digit));
        assert_eq!(images[2 * index + 1], ([1.0; 784], digit));
    }
}
#[test]
fn rejects_empty_classes_wrong_dimensions_color_and_corrupt_png_with_path() {
    let dataset = TemporaryDataset::new();
    assert!(
        load_labeled_png_images(&dataset.0)
            .unwrap_err()
            .contains("нет PNG")
    );
    let path = dataset.0.join("0/a.png");
    for (width, color) in [(27, png::ColorType::Grayscale), (28, png::ColorType::Rgb)] {
        write_png(&path, width, color, 0);
        let error = load_labeled_png_images(&dataset.0).unwrap_err();
        assert!(error.contains(&path.display().to_string()));
        assert!(error.contains("Ожидается статический PNG"));
    }
    std::fs::write(&path, "broken PNG").unwrap();
    assert!(
        load_labeled_png_images(&dataset.0)
            .unwrap_err()
            .contains(&path.display().to_string())
    );
    let missing = dataset.0.join("missing");
    assert!(
        load_labeled_png_images(&missing)
            .unwrap_err()
            .contains(&missing.join("0").display().to_string())
    );
}

#[test]
fn splits_sorted_images_per_digit_without_reading_test() {
    let dataset = TemporaryDataset::new();
    let mut expected_train = Vec::new();
    let mut expected_validation = Vec::new();
    for (digit_index, digit) in ALL_DIGITS.into_iter().enumerate() {
        let directory = dataset.0.join("train").join(digit_index.to_string());
        std::fs::create_dir_all(&directory).unwrap();
        // Разные размеры классов проверяют, что отсчёт начинается заново для каждой цифры.
        let count = digit_index + 2;
        for position in (0..count).rev() {
            write_png(
                &directory.join(format!("{position:02}.png")),
                28,
                png::ColorType::Grayscale,
                position as u8,
            );
        }
        for position in 0..count {
            let image = ([position as f64 / 255.0; 784], digit);
            if position % 5 == 0 {
                expected_validation.push(image);
            } else {
                expected_train.push(image);
            }
        }
    }
    let (train, validation) = mnist_data::load_train_validation_images(&dataset.0).unwrap();
    assert_eq!(train, expected_train);
    assert_eq!(validation, expected_validation);
    // Ошибка чтения train должна передаваться вызывающему коду.
    std::fs::remove_dir_all(dataset.0.join("train/0")).unwrap();
    assert!(mnist_data::load_train_validation_images(&dataset.0).is_err());
}

#[test]
fn loads_three_sets_with_test_kept_separate() {
    let dataset = TemporaryDataset::new();
    for digit in 0..10 {
        for (folder, brightness) in [("train", 32), ("test", 224)] {
            let directory = dataset.0.join(folder).join(digit.to_string());
            std::fs::create_dir_all(&directory).unwrap();
            write_png(&directory.join("a.png"), 28, png::ColorType::Grayscale, brightness);
            write_png(&directory.join("b.png"), 28, png::ColorType::Grayscale, brightness);
        }
    }
    let (train, validation, test) = mnist_data::load_mnist_images(&dataset.0).unwrap();
    assert_eq!((train.len(), validation.len(), test.len()), (10, 10, 20));
    for (index, digit) in ALL_DIGITS.into_iter().enumerate() {
        assert_eq!(train[index], ([32.0 / 255.0; 784], digit));
        assert_eq!(validation[index], ([32.0 / 255.0; 784], digit));
        assert_eq!(test[2 * index], ([224.0 / 255.0; 784], digit));
        assert_eq!(test[2 * index + 1], ([224.0 / 255.0; 784], digit));
    }
    std::fs::remove_dir_all(dataset.0.join("test/0")).unwrap();
    assert!(mnist_data::load_mnist_images(&dataset.0).unwrap_err().contains("test/0"));
}
