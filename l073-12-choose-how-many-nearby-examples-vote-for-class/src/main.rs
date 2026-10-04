// Урок 073. Выбирать класс большинством голосов первых k соседей из готового списка.
// Изменение k может изменить ответ, поэтому число соседей является отдельной настройкой метода.

fn main() {
    let neighbors = [true, false, false, true, true];
    let mut results = Vec::new();
    for count in [1, 3, 5] {
        let votes = neighbors[..count].iter().filter(|&&v| v).count();
        let prediction = votes * 2 > count;
        println!("Соседей={count}: положительных голосов={votes}, класс={prediction}");
        results.push(prediction);
    }
    assert_eq!(results, vec![true, false, true]);
}

// Чему учит этот урок:
// Учимся выбирать класс большинством голосов первых k соседей из готового списка.
// Изменение k может изменить ответ, поэтому число соседей является отдельной настройкой метода.
