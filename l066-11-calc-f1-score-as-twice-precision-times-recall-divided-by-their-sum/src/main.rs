// Урок 11.4. Оценка F1: удвоенное произведение точности и полноты, делённое на их сумму.
// Зачем здесь эта тема: Precision и recall могут расходиться; F1 сводит их в число, чувствительное
//   к меньшему из двух.
// Почему код устроен так: Используем гармоническое среднее и рассматриваем нулевые знаменатели.
// Представь: Если precision высокий, а recall низкий, F1 не позволит одному хорошему числу скрыть
//   другое.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

use l066_11_calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum::calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum;

fn main() {
    for (_description, correct_pos_prediction_share, actual_pos_detection_share, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        assert_eq!(
            calc_f1_score_as_twice_precision_times_recall_divided_by_their_sum(
                Some(correct_pos_prediction_share),
                Some(actual_pos_detection_share),
            ),
            expected
        );
    }
}

// Чему учит этот урок:
// Учимся объединять точность положительных прогнозов и полноту обнаружения в F1.
// Проверяем, как низкое значение одной составляющей снижает общий результат, и обрабатываем
// нулевой знаменатель.
