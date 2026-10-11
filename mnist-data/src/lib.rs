//! Общие типы и чтение PNG MNIST для уроков 243–246.

/// Правильная или предсказанная цифра. Порядок соответствует папкам 0–9.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// Все цифры в порядке номеров папок и выходных оценок модели.
pub const ALL_DIGITS: [Digit; 10] = [
    Digit::Zero,
    Digit::One,
    Digit::Two,
    Digit::Three,
    Digit::Four,
    Digit::Five,
    Digit::Six,
    Digit::Seven,
    Digit::Eight,
    Digit::Nine,
];

/// Каталог PNG MNIST относительно crate mnist_data, независимо от рабочей папки запуска.
pub fn mnist_dataset_directory() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist")
}

/// Загружает train/validation и отдельный официальный test.
pub fn load_mnist_images(
    dataset_directory: &std::path::Path,
) -> Result<
    (
        Vec<([f64; 784], Digit)>,
        Vec<([f64; 784], Digit)>,
        Vec<([f64; 784], Digit)>,
    ),
    String,
> {
    let (train_images, validation_images): (Vec<([f64; 784], Digit)>, Vec<([f64; 784], Digit)>) =
        load_train_validation_images(dataset_directory)?;
    let test_images: Vec<([f64; 784], Digit)> =
        load_labeled_png_images(&dataset_directory.join("test"))?;
    Ok((train_images, validation_images, test_images))
}

/// Загружает train и откладывает позиции 0, 5, 10… каждой цифры в validation.
/// Порядок цифр и отсортированных имён PNG сохраняется. Официальный test не читается.
pub fn load_train_validation_images(
    dataset_directory: &std::path::Path,
) -> Result<(Vec<([f64; 784], Digit)>, Vec<([f64; 784], Digit)>), String> {
    let original_train_images: Vec<([f64; 784], Digit)> =
        load_labeled_png_images(&dataset_directory.join("train"))?;
    let mut train_images: Vec<([f64; 784], Digit)> = Vec::new();
    let mut validation_images: Vec<([f64; 784], Digit)> = Vec::new();
    // Типы переменных: digit: Digit.
    for digit in ALL_DIGITS {
        // Фильтруем картинки одной цифры, сохраняя порядок имён файлов.
        let images_of_current_digit: Vec<&([f64; 784], Digit)> = original_train_images
            .iter()
            .filter(|image: &&([f64; 784], Digit)| -> bool { image.1 == digit })
            .collect();
        // В train копируем все картинки, кроме каждой пятой внутри этой цифры.
        train_images.extend(
            images_of_current_digit
                .iter()
                .enumerate()
                .filter(|(position, _): &(usize, &&([f64; 784], Digit))| -> bool {
                    position % 5 != 0
                })
                .map(
                    |(_, &image): (usize, &&([f64; 784], Digit))| -> ([f64; 784], Digit) { *image },
                ),
        );
        // В validation копируем каждую пятую: позиции 0, 5, 10…
        validation_images.extend(
            images_of_current_digit
                .iter()
                .enumerate()
                .filter(|(position, _): &(usize, &&([f64; 784], Digit))| -> bool {
                    position % 5 == 0
                })
                .map(
                    |(_, &image): (usize, &&([f64; 784], Digit))| -> ([f64; 784], Digit) { *image },
                ),
        );
    }
    Ok((train_images, validation_images))
}

/// Читает папки 0–9, сортирует PNG внутри каждой и возвращает картинки с метками.
/// Каждый PNG должен быть статическим, 28×28, grayscale, 8 бит.
/// Яркости преобразуются из байтов 0–255 в f64 от 0 до 1.
/// Разделение train/validation доступно через load_train_validation_images.
pub fn load_labeled_png_images(
    images_directory: &std::path::Path,
) -> Result<Vec<([f64; 784], Digit)>, String> {
    let mut labeled_images: Vec<([f64; 784], Digit)> = Vec::new();
    // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
    // Имя PNG обозначает номер файла; правильная цифра определяется папкой.
    // Типы переменных: digit: Digit.
    for digit in ALL_DIGITS {
        let digit_directory: std::path::PathBuf =
            images_directory.join((digit as usize).to_string());
        // Изменяемый список нужен только сбору и сортировке; дальше png_file_paths неизменяемый.
        let png_file_paths: Vec<std::path::PathBuf> = {
            let directory_entries: std::fs::ReadDir =
                std::fs::read_dir(&digit_directory).map_err(|error: std::io::Error| -> String {
                    format!(
                        "{}: {error}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                        digit_directory.display()
                    )
                })?;
            let mut png_file_paths: Vec<std::path::PathBuf> = Vec::new();
            // Типы переменных: directory_entry: Result<std::fs::DirEntry, std::io::Error>.
            for directory_entry in directory_entries {
                let png_file_path: std::path::PathBuf = directory_entry
                    .map_err(|error: std::io::Error| -> String { error.to_string() })?
                    .path();
                if png_file_path.extension().is_some_and(
                    |file_extension: &std::ffi::OsStr| -> bool {
                        file_extension.eq_ignore_ascii_case("png")
                    },
                ) {
                    png_file_paths.push(png_file_path);
                }
            }
            // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
            // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
            png_file_paths.sort();
            if png_file_paths.is_empty() {
                return Err(format!("{}: нет PNG", digit_directory.display()));
            }
            png_file_paths
        };
        // Типы переменных: png_file_path: std::path::PathBuf.
        for png_file_path in png_file_paths {
            // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
            let normalized_pixels: [f64; 784] = {
                // Читатель и изменяемый буфер живут только во время декодирования PNG.
                // К каждой ошибке добавляем путь, чтобы найти проблемную картинку.
                let pixel_bytes: [u8; 784] = {
                    let png_file: std::fs::File = std::fs::File::open(&png_file_path).map_err(
                        |error: std::io::Error| -> String {
                            format!("{}: {error}", png_file_path.display())
                        },
                    )?;
                    let mut png_reader: png::Reader<std::io::BufReader<std::fs::File>> =
                        png::Decoder::new(std::io::BufReader::new(png_file))
                            .read_info()
                            .map_err(|error: png::DecodingError| -> String {
                                format!("{}: {error}", png_file_path.display())
                            })?;
                    let png_metadata: &png::Info<'_> = png_reader.info();
                    // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                    // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                    if png_metadata.width != 28
                        || png_metadata.height != 28
                        || png_metadata.color_type != png::ColorType::Grayscale
                        || png_metadata.bit_depth != png::BitDepth::Eight
                        || png_metadata.animation_control.is_some()
                    {
                        return Err(format!(
                            "{}: Ожидается статический PNG 28×28, grayscale, 8 бит",
                            png_file_path.display()
                        ));
                    }
                    // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                    // Пиксели развёрнуты из двух пространственных измерений: индекс y*28+x соответствует (y,x).
                    let mut pixel_bytes: [u8; 784] = [0u8; 784];
                    png_reader.next_frame(&mut pixel_bytes).map_err(
                        |error: png::DecodingError| -> String {
                            format!("{}: {error}", png_file_path.display())
                        },
                    )?;
                    png_reader
                        .finish()
                        .map_err(|error: png::DecodingError| -> String {
                            format!("{}: {error}", png_file_path.display())
                        })?;
                    pixel_bytes
                };
                // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                // Это фиксированная нормализация, не требующая статистик validation или test.
                std::array::from_fn(|pixel_index: usize| -> f64 {
                    f64::from(pixel_bytes[pixel_index]) / 255.0
                })
            };
            labeled_images.push((normalized_pixels, digit));
        }
    }
    Ok(labeled_images)
}
