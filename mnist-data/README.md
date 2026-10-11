# Общий загрузчик PNG MNIST

Crate `mnist_data` используется уроками 243–246. Он содержит `Digit`, `ALL_DIGITS`, `mnist_dataset_directory`, `load_labeled_png_images` `load_train_validation_images` и `load_mnist_images`.

```rust
use mnist_data::{load_mnist_images, mnist_dataset_directory};

let dataset_directory = mnist_dataset_directory();
let (train_images, validation_images, test_images) =
    load_mnist_images(&dataset_directory)?;
```

Загрузчик читает папки `0` … `9`, сортирует имена PNG в каждой папке и берёт метку из имени папки. Проверяет статический PNG 28×28, grayscale, 8 бит; нормализует яркости делением на 255. Ошибки содержат путь. `load_train_validation_images(&dataset_directory)` загружает папку train и возвращает `(train_images, validation_images)`: позиции 0, 5, 10… внутри каждой цифры идут в validation. `load_mnist_images` добавляет загрузку официального test и возвращает три набора. Её используют все четыре урока 243–246. В уроках 245–246 индексы batch относятся к соответствующему набору train или validation. Обучение и оценка остаются в уроках.

Проверки: `cargo test -p mnist_data`.

`load_mnist_images` возвращает три набора `Vec<([NormalizedPixelBrightness; 784], Digit)>`. Тип проверяет конечность и диапазон 0..=1 до возврата данных в урок. Низкоуровневые `load_labeled_png_images` и `load_train_validation_images` сохраняют представление `[f64; 784]`.

В `mnist_data::numeric_types` находятся общие типы яркости, веса, смещения, оценки, вероятности и градиента с закрытыми полями и проверяемыми преобразованиями. `NormalizedPixelBrightness` также доступен непосредственно из `mnist_data`.
