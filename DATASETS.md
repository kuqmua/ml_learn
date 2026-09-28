# Открытые датасеты для практики

Из корня workspace выполни две команды:

```bash
python3 scripts/prepare_open_datasets.py all
cargo run -p lesson-datasets --example iris_classification
```

Скрипт использует только стандартную библиотеку Python. Он скачивает официальные ZIP-файлы UCI, сверяет SHA-256, проверяет число строк и создаёт файлы с постоянной схемой в `datasets/processed/`. Повторный запуск использует проверенный архив из `datasets/raw/`. Папка `datasets/` исключена из Git. Если контрольная сумма не совпадает, скрипт останавливается до использования данных.

Можно скачать только нужный набор: `python3 scripts/prepare_open_datasets.py iris`, `wine_quality_red` или `sms_spam`. После подготовки сеть не нужна для запуска Rust-примеров. Для работы с собственным файлом используй функции `load_*_from_path` из пакета `lesson-datasets`.

## Выбор набора

| Набор | ZIP / подготовленный файл | Файл после подготовки | Тип в Rust | Подходит для уроков |
|---|---:|---|---|---|
| Iris | 3,7 / 3,9 КБ | `datasets/processed/iris.csv` | `[f64; 4]` + `IrisSpecies` | 06–08 статистика и разделение, 11–12 классификация и ближайшие соседи, 18–19 кластеризация и главные компоненты |
| Wine Quality, красное вино | 91,4 / 92,0 КБ | `datasets/processed/wine_quality_red.csv` | `[f64; 11]` + `quality: f64` | 06–08 статистика и подготовка, 09 регрессия, 17 проверка модели, 46 итоговый проект |
| SMS Spam Collection | 203,4 / 465,9 КБ | `datasets/processed/sms_spam.tsv` | `is_spam: bool` + `message: String` | 07–08 разделение, 11 метрики, 13 наивный Байес, 27–30 токенизация и поиск |

### Iris: классификация и геометрия

```bash
python3 scripts/prepare_open_datasets.py iris
cargo run -p lesson-datasets --example iris_classification
```

В [примере](lesson-datasets/examples/iris_classification.rs) четыре измерения цветка сразу передаются в функцию `squared_euclidean_distance_between_points` из урока 01.4. Метка — один из трёх видов. Пример разделяет каждый класс на train/validation/test с фиксированным seed и классифицирует по ближайшему обучающему цветку. Исходный файл отсортирован по видам, поэтому нельзя брать первые 70% строк как train.

Источник: [Iris, UCI Machine Learning Repository](https://archive.ics.uci.edu/dataset/53/iris), Fisher (1936). Лицензия, указанная UCI: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). В архиве 150 строк с четырьмя признаками и классом.

### Wine Quality: регрессия

```bash
python3 scripts/prepare_open_datasets.py wine_quality_red
cargo run -p lesson-datasets --example wine_quality_regression
```

В [примере](lesson-datasets/examples/wine_quality_regression.rs) одиннадцать числовых признаков преобразуются в `[f64; 11]`, а оценка качества — в `f64`. Обучающий набор задаёт средний прогноз и коэффициенты простой модели по содержанию алкоголя. `arithmetic_mean_of_values`, `mean_absolute_error` и `mean_squared_error` вызываются из уже пройденных пакетов; validation и test не участвуют в подгонке. Порядок признаков: fixed acidity, volatile acidity, citric acid, residual sugar, chlorides, free sulfur dioxide, total sulfur dioxide, density, pH, sulphates, alcohol.

Источник: [Wine Quality, UCI Machine Learning Repository](https://archive.ics.uci.edu/dataset/186/wine+quality), Cortez и соавт. (2009). Лицензия, указанная UCI: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Скрипт выбирает только красное вино: 1599 строк.

### SMS Spam: текст и несбалансированные классы

```bash
python3 scripts/prepare_open_datasets.py sms_spam
cargo run -p lesson-datasets --example sms_spam_classification
```

В [примере](lesson-datasets/examples/sms_spam_classification.rs) метка `spam` становится `true`, `ham` — `false`, сообщение остаётся строкой UTF-8. Разделение сохраняет долю каждого класса. Прогноз большинства, выученный на train, оценивается функциями уроков 11.1 и 11.3. Сравни `accuracy` и `recall`: высокая доля верных ответов здесь возможна даже при пропуске всего спама. Следующий шаг — заменить прогноз большинства классификатором из блока 13 и использовать те же индексы разделения.

Источник: [SMS Spam Collection, UCI Machine Learning Repository](https://archive.ics.uci.edu/dataset/228/sms+spam+collection), Almeida и Hidalgo (2011). Лицензия, указанная UCI: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). Проверенный ZIP содержит 5572 сообщения; на странице каталога UCI указано 5574, поэтому скрипт проверяет содержимое конкретного архива.

## Подключение к своему уроку

В `Cargo.toml` нужного пакета добавь:

```toml
[dependencies]
lesson-datasets = { path = "../lesson-datasets" }
```

Затем загрузи записи и раздели их **до** обучения преобразований:

```rust
use lesson_datasets::{load_iris_records, split_indices_stratified_by_class};

let records = load_iris_records()?;
let classes: Vec<u8> = records.iter()
    .map(|record| record.species.class_identifier())
    .collect();
let split = split_indices_stratified_by_class(&classes, 42)?;
let training_features: Vec<[f64; 4]> = split.training_indices.iter()
    .map(|&index| records[index].features)
    .collect();
```

Функции `load_wine_quality_red_records()` и `load_sms_spam_records()` возвращают соответствующие типы. Для регрессии используй `split_indices`, для классификации — `split_indices_stratified_by_class`. Средние, масштабирование, словарь и другие обучаемые преобразования вычисляй только по `training_indices`. Порог и настройки выбирай по `validation_indices`; `test_indices` оставь для итоговой оценки. Seed `42` в примерах нужен для воспроизводимости, его можно заменить, сохранив в отчёте.

Если подготовленные файлы отсутствуют, загрузчик печатает команду для скачивания. Если схема файла нарушена, он сообщает номер ошибочной строки.

Обычные `part-*` уроки продолжают запускать свои маленькие встроенные примеры. Для работы с открытым набором запусти один из трёх примеров `lesson-datasets` или добавь загрузчик в нужный урок по образцу выше.
