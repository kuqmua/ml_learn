# Общий загрузчик PNG MNIST

Crate `mnist_data` используется уроками 243–246. Он содержит `Digit`, `ALL_DIGITS` и `load_labeled_png_images`.

```rust
use mnist_data::{Digit, load_labeled_png_images};

let images: Vec<([f64; 784], Digit)> =
    load_labeled_png_images(&dataset_directory.join("train"))?;
```

Загрузчик читает папки `0` … `9`, сортирует имена PNG в каждой папке и берёт метку из имени папки. Проверяет статический PNG 28×28, grayscale, 8 бит; нормализует яркости делением на 255. Ошибки содержат путь. Разделение данных, обучение и оценка остаются в уроках.

Проверки: `cargo test -p mnist_data`.
