// Урок 08.6. Практика: повторяемое обучение, простая модель для сравнения и версия данных.
// Связь с принятой терминологией: Воспроизводимый эксперимент с базовой моделью и версией данных.
// Зачем здесь эта тема: Честный эксперимент требует одновременно baseline, фиксированной
//   случайности, параметров и версии данных.
// Почему код устроен так: Собираем их в один запуск, чтобы результат можно было объяснить и
//   повторить.
// Представь: Сравнивать модели можно только если известно, на каких данных, с какими настройками и
//   seed их запускали.
//
// Что повторяем вместе: seed, baseline, конфигурация, журнал метрик, версии данных.
// Зачем это нужно: Повторяемый seed и отпечаток данных позволяют связать полученную метрику с конкретным
//   запуском и входом.
// Что показывает программа: Берём seed из аргумента командной строки или используем фиксированное значение.
//   Запускаем повторяемый эксперимент и считаем простую базовую метрику. Записываем seed, отпечаток данных
//   и результат для сравнения запусков.
// Что проверить при изменении примера: Два запуска с одинаковыми входами дают одинаковый результат;
//   изменение seed отражается в отчёте.
// Дополнительная практика: Создай CLI с параметрами seed и пути к данным; сохрани конфигурацию, метрики и хеш
//   входного файла.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Шаг: Берём seed из аргумента командной строки или используем фиксированное значение."
    );
    lesson_trace::trace_note!("Берём элемент с указанным порядковым номером.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("При отсутствии значения используем запасной вариант.");
    let seed: u64 = std::env::args()
        .nth(1)
        .map(|seed_text| seed_text.parse::<u64>().expect("seed: целое число"))
        .unwrap_or(42);
    lesson_trace::trace_step!(seed);
    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const SAMPLE_DATA: &str = "1,0\n2,0\n3,1\n4,1\n";

    lesson_trace::trace_note!(
        "Шаг: Запускаем повторяемый эксперимент и считаем простую базовую метрику."
    );
    let (random_state, baseline_accuracy): (u64, f64) = (|| -> (u64, f64) {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Из одного seed получаем то же состояние генератора и ту же базовую метрику."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `seed`.");
        let seed: u64 = seed;
        lesson_trace::trace_step!(seed);
        lesson_trace::trace_note!(
            "Один шаг линейного конгруэнтного генератора: фиксированный множитель и +1 по mod 2⁶⁴."
        );
        lesson_trace::trace_note!("Так один seed всегда приводит к одному и тому же состоянию.");
        let state: u64 = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        lesson_trace::trace_step!(state);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `baseline_accuracy` для следующих операций."
        );
        lesson_trace::trace_note!("Разбиваем текст на строки для последовательной обработки.");
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        let baseline_accuracy: f64 = SAMPLE_DATA
            .lines()
            .filter(|line| line.ends_with(",1"))
            .count() as f64
            / SAMPLE_DATA.lines().count() as f64;
        lesson_trace::trace_step!(baseline_accuracy);
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        (state, baseline_accuracy)
    })();
    lesson_trace::trace_step!(random_state);
    lesson_trace::trace_step!(baseline_accuracy);

    lesson_trace::trace_note!(
        "Шаг: Записываем seed, отпечаток данных и результат для сравнения запусков."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!(
        "Отпечаток данных получаем последовательным смешиванием байтов строки."
    );
    lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
    lesson_trace::trace_note!("Создаём хешер, чтобы получить воспроизводимый отпечаток данных.");
    lesson_trace::trace_note!("Добавляем байты входных данных в состояние хешера.");
    lesson_trace::trace_note!("Завершаем хеширование и получаем числовой отпечаток.");
    println!(
        "seed={seed}, data_hash={}, random_state={random_state}, baseline_accuracy={baseline_accuracy:.2}",
        (|| -> u64 {
            let data: &str = SAMPLE_DATA;
            lesson_trace::trace_step!(data);
            lesson_trace::trace_step!(data);

            let mut hasher: std::collections::hash_map::DefaultHasher =
                std::collections::hash_map::DefaultHasher::new();
            lesson_trace::trace_step!(hasher);
            lesson_trace::trace_step!(hasher);

            std::hash::Hash::hash(data, &mut hasher);

            std::hash::Hasher::finish(&hasher)
        })()
    );
}
