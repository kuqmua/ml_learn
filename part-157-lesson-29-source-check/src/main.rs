// Урок 29.4. Проверка источника.
//
// Ответ допускается при оценке источника не ниже порога; ниже порога система воздерживается.

fn main() {
    let minimum_reliable_score = 0.5;
    for (description, retrieval_score, expected_answer) in [
        ("слабый источник", 0.1, "нет надёжного источника"),
        ("ровно на пороге", 0.5, "подтверждённый ответ"),
        ("сильный источник", 0.9, "подтверждённый ответ"),
    ] {
        assert!((0.0..=1.0).contains(&retrieval_score));
        let answer = if retrieval_score >= minimum_reliable_score {
            "подтверждённый ответ"
        } else {
            "нет надёжного источника"
        };
        assert_eq!(answer, expected_answer);
        println!("{description}: score={retrieval_score} → {answer}");
    }
}
