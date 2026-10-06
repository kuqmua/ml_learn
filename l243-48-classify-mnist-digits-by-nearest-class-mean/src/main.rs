fn main() -> Result<(), String> {
    // Урок 243. Путь данных: PNG -> 784 числа -> средние изображения -> прогноз цифры.
    // Каждая картинка имеет размер 28×28, поэтому в ней 28*28 = 784 пикселя.
    // У одной записи два поля: массив яркостей и правильная метка (цифра 0–9).
    // «Обучение» здесь — накопление и усреднение примеров каждой цифры.
    // Затем новую картинку сравниваем с десятью средними и выбираем ближайшее.
    //
    // Сначала объявлены Digit и общие операции: загрузка PNG, оценка и вывод метрик.
    // Затем выполняются обучение средних, оценка validation и оценка test.
    // Train учит модель; validation проверяет её на отложенных примерах;
    // test даёт итоговую независимую оценку. Ниже средние вычисляются только по train.
    // Фигурные скобки ограничивают жизнь временных имён; последняя строка блока
    // без точки с запятой возвращает результат следующему этапу.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 — дробное число двойной точности; средние: [[f64; 784]; 10].

    // Метка — одна из десяти цифр. Значения вроде 42 теперь нельзя записать как метку.
    // digit as usize даёт число 0..9 для индексов массивов, имён папок и печати.
    // Copy позволяет копировать метки; PartialEq/Eq — сравнивать правильную цифру с прогнозом.
    // repr(u8) задаёт однобайтовое хранение варианта, но тип метки остаётся Digit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum Digit {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
    }
    // Порядок соответствует папкам 0..9 и индексам классов в массивах.
    let classes: [Digit; 10] = [
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

    // Корень PNG фиксирован относительно папки урока; аргументы запуска не требуются.
    let data: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");

    // Запись датасета: ([f64; 784], Digit), например (pixels, Digit::Seven).
    // Один загрузчик PNG для train и test: возвращает пиксели и правильные метки.
    let load_digits = |directory: &std::path::Path| -> Result<Vec<([f64; 784], Digit)>, String> {
        let mut digits: Vec<([f64; 784], Digit)> = Vec::new();
        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
        // Имя PNG обозначает номер файла; правильная цифра определяется папкой.
        // Типы переменных: class: Digit.
        for class in classes {
            let class_directory: std::path::PathBuf = directory.join((class as usize).to_string());
            let entries: std::fs::ReadDir =
                std::fs::read_dir(&class_directory).map_err(|e: std::io::Error| -> String {
                    format!(
                        "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                        class_directory.display()
                    )
                })?;
            let mut paths: Vec<std::path::PathBuf> = Vec::new();
            // Типы переменных: entry: Result<std::fs::DirEntry, std::io::Error>.
            for entry in entries {
                let path: std::path::PathBuf = entry
                    .map_err(|e: std::io::Error| -> String { e.to_string() })?
                    .path();
                if path
                    .extension()
                    .is_some_and(|ext: &std::ffi::OsStr| -> bool {
                        ext.eq_ignore_ascii_case("png")
                    })
                {
                    paths.push(path);
                }
            }
            // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
            // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
            paths.sort();
            if paths.is_empty() {
                return Err(format!("{}: нет PNG", class_directory.display()));
            }
            // Типы переменных: path: std::path::PathBuf.
            for path in paths {
                // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                let decoded: [f64; 784] = (|| -> Result<[f64; 784], String> {
                    let file: std::fs::File = std::fs::File::open(&path)
                        .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                    let mut reader: png::Reader<std::io::BufReader<std::fs::File>> =
                        png::Decoder::new(std::io::BufReader::new(file))
                            .read_info()
                            .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    let info: &png::Info<'_> = reader.info();
                    // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                    // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                    if info.width != 28
                        || info.height != 28
                        || info.color_type != png::ColorType::Grayscale
                        || info.bit_depth != png::BitDepth::Eight
                        || info.animation_control.is_some()
                    {
                        return Err("Ожидается статический PNG 28×28, grayscale, 8 бит".into());
                    }
                    // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                    // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                    let mut pixels: [u8; 784] = [0u8; 784];
                    reader
                        .next_frame(&mut pixels)
                        .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    reader
                        .finish()
                        .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                    // Это фиксированная нормализация, не требующая статистик validation или test.
                    Ok(std::array::from_fn(|i: usize| -> f64 {
                        f64::from(pixels[i]) / 255.0
                    }))
                })()
                // Добавляем путь к ошибке декодирования, чтобы найти проблемный PNG.
                .map_err(|e: String| -> String { format!("{}: {e}", path.display()) })?;
                digits.push((decoded, class));
            }
        }
        Ok(digits)
    };

    // Общая оценка validation и test. Возвращаем число верных прогнозов модели,
    // число верных прогнозов baseline и матрицу ошибок; доли вычисляет report.
    let evaluate = |means: &[[f64; 784]; 10],
                    records: &[([f64; 784], Digit)],
                    indices: &[usize],
                    majority: Digit|
     -> (usize, usize, [[usize; 10]; 10]) {
        // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
        // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
        let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
        // Здесь baseline — счётчик верных ответов постоянной цифрой majority, а не сама цифра.
        let mut baseline: usize = 0usize;
        // Типы переменных: index: usize.
        for &index in indices {
            let (pixels, label): &([f64; 784], Digit) = &records[index];
            // Проверяем все десять классов и выбираем наименьшее расстояние до среднего.
            // Метка текущей картинки не участвует в выборе; её используем позже для проверки ответа.
            let predicted: Digit = classes
                .into_iter()
                .min_by(|&a: &Digit, &b: &Digit| -> std::cmp::Ordering {
                    // Квадрат евклидова расстояния: sum_p((pixel[p]-mean[c,p])²).
                    // Квадраты не дают положительным и отрицательным различиям сократиться.
                    // Корень не нужен: он не меняет порядок расстояний и выбранную цифру.
                    let distance = |class: Digit| -> f64 {
                        pixels
                            .iter()
                            .zip(means[class as usize])
                            .map(|(x, y): (&f64, f64)| -> f64 { (x - y).powi(2) })
                            .sum::<f64>()
                    };
                    distance(a).total_cmp(&distance(b))
                })
                .unwrap();
            // Digit преобразуем в usize только для выбора строки и столбца счётчиков.
            confusion[*label as usize][predicted as usize] += 1;
            baseline += usize::from(*label == majority);
        }
        // Сумма диагонали матрицы ошибок — число правильных прогнозов.
        let correct: usize = (0..10).map(|i: usize| -> usize { confusion[i][i] }).sum();
        (correct, baseline, confusion)
    };

    // Печатаем одинаковые метрики для обеих частей; матрица ошибок нужна для test.
    let report = |name: &str, count: usize, metrics: (usize, usize, [[usize; 10]; 10])| -> () {
        let (correct, baseline, confusion): (usize, usize, [[usize; 10]; 10]) = metrics;
        // Accuracy = correct/count; для baseline аналогично. Например, 8 из 10 дают 0.8.
        println!(
            "{name}: n={count}, accuracy={:.4}, baseline={:.4}",
            correct as f64 / count as f64,
            baseline as f64 / count as f64
        );
        if name == "test" {
            println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
            // Типы переменных: row: [usize; 10].
            for row in confusion {
                println!("{row:?}");
            }
        }
    };

    // Загружаем исходный train, отделяем validation и обучаем десять средних изображений.
    // Наружу выходят means и цифра baseline; картинки и счётчики остаются в этом блоке.
    let (means, majority): ([[f64; 784]; 10], Digit) = {
        // Загрузчик читает весь исходный train; разделение на train/validation идёт ниже.
        let digits: Vec<([f64; 784], Digit)> = load_digits(&data.join("train"))?;
        let (means, majority, validation): ([[f64; 784]; 10], Digit, Vec<usize>) = {
            // Для каждой цифры считаем только обучающие примеры.
            // Эти числа нужны для деления суммы пикселей на число картинок и выбора baseline.
            let mut counts: [usize; 10] = [0usize; 10];
            // sums[c][p] — сумма яркости пикселя p у всех train-картинок цифры c.
            // После деления получим десять средних изображений, каждое из 784 чисел.
            let mut sums: [[f64; 784]; 10] = [[0.0; 784]; 10];
            // seen считает все просмотренные примеры класса, включая отложенные.
            // Он отвечает за разделение, а counts — только за количество использованных в обучении.
            let mut seen: [usize; 10] = [0usize; 10];
            let mut validation: Vec<usize> = Vec::new();
            // Типы переменных: index: usize, pixels: &[f64; 784], label: &Digit.
            for (index, (pixels, label)) in digits.iter().enumerate() {
                let class: usize = *label as usize;
                // Отложенный пример добавляем только в validation: он не меняет sums или counts.
                // Так оценка проверяет перенос на картинки, которых среднее изображение не видело.
                if seen[class] % 5 == 0 {
                    validation.push(index);
                } else {
                    counts[class] += 1;
                    // Типы переменных: sum: &mut f64, pixel: &f64.
                    for (sum, pixel) in sums[class].iter_mut().zip(pixels) {
                        // Прибавляем яркость в ТОЙ ЖЕ координате. Это выравнивание всех картинок по пикселям:
                        // сдвинутые или очень разные почерки будут размываться в среднем изображении.
                        *sum += pixel;
                    }
                }
                seen[class] += 1;
            }
            if counts.contains(&0) {
                return Err("В train нужны все десять классов".into());
            }
            // Делим покоординатную сумму на число примеров класса: mean[c,p]=sum[c,p]/count[c].
            // Обучение этой простой модели на этом заканчивается — итераций и градиентов здесь нет.
            let mut means: [[f64; 784]; 10] = sums;
            // Типы переменных: mean: &mut [f64; 784], count: usize.
            for (mean, count) in means.iter_mut().zip(counts) {
                // Типы переменных: pixel: &mut f64.
                for pixel in mean {
                    *pixel /= count as f64;
                }
            }
            // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
            // насколько модель лучше простого ответа без анализа пикселей.
            // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
            let majority: Digit = classes
                .into_iter()
                .max_by_key(|&digit: &Digit| -> (usize, std::cmp::Reverse<usize>) {
                    let index: usize = digit as usize;
                    (counts[index], std::cmp::Reverse(index))
                })
                .unwrap();
            println!(
                "train: n={}, baseline digit={}; split: every fifth per class",
                counts.iter().sum::<usize>(),
                majority as usize
            );

            (means, majority, validation)
        };
        // Общие evaluate/report проверяют отложенные индексы, не изменяя обученные means.
        report(
            "validation",
            validation.len(),
            evaluate(&means, &digits, &validation, majority),
        );
        (means, majority)
    };
    // Test загружается после обучения; его данные и метрики остаются здесь.
    {
        let test: Vec<([f64; 784], Digit)> = load_digits(&data.join("test"))?;
        let indices: Vec<usize> = (0..test.len()).collect();
        report(
            "test",
            indices.len(),
            evaluate(&means, &test, &indices, majority),
        );
    }

    Ok(())
}
