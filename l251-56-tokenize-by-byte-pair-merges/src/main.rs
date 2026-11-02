use std::collections::BTreeMap;

// ID 0..255 — все возможные одиночные байты. Дальше добавляем куски.
fn vocabulary() -> BTreeMap<Vec<u8>, u32> {
    (0..=255u8).map(|b| (vec![b], u32::from(b))).collect()
}

// Один учебный фрагмент: без regex-разбиения и специальных токенов.
// Выбираем минимальный ранг объединения, а НЕ самый длинный кусок.
fn encode(text: &str, vocab: &BTreeMap<Vec<u8>, u32>, trace: bool) -> Vec<u32> {
    let mut parts: Vec<Vec<u8>> = text.bytes().map(|b| vec![b]).collect();
    loop {
        let best = parts
            .windows(2)
            .enumerate()
            .filter_map(|(i, pair)| {
                let joined = [pair[0].as_slice(), pair[1].as_slice()].concat();
                vocab.get(&joined).map(|&rank| (rank, i, joined))
            })
            .min_by_key(|(rank, i, _)| (*rank, *i));
        let Some((rank, i, joined)) = best else { break };
        if trace {
            println!("  ранг {rank}: {:?} + {:?} -> {:?}", parts[i], parts[i + 1], joined);
        }
        parts.splice(i..i + 2, [joined]);
    }
    parts.iter().map(|part| vocab[part]).collect()
}

// Обучение: считаем пары в корпусе; самый частый новый кусок получает следующий ID.
// При равенстве частот BTreeMap даёт воспроизводимый лексикографический выбор.
fn train(corpus: &[&str], merges: usize) -> BTreeMap<Vec<u8>, u32> {
    let mut vocab = vocabulary();
    let mut words: Vec<Vec<Vec<u8>>> = corpus.iter().map(|s| s.bytes().map(|b| vec![b]).collect()).collect();
    for _ in 0..merges {
        let mut counts = BTreeMap::new();
        for word in &words {
            for pair in word.windows(2) {
                *counts.entry([pair[0].as_slice(), pair[1].as_slice()].concat()).or_insert(0usize) += 1;
            }
        }
        let Some((piece, count)) = counts
            .into_iter()
            .filter(|(p, _)| !vocab.contains_key(p))
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        else {
            break;
        };
        let id = vocab.len() as u32;
        println!("Обучение: {:?}, частота {count}, ID {id}", String::from_utf8_lossy(&piece));
        vocab.insert(piece.clone(), id);
        for word in &mut words {
            let mut i = 0;
            while i + 1 < word.len() {
                if [word[i].as_slice(), word[i + 1].as_slice()].concat() == piece {
                    word.splice(i..i + 2, [piece.clone()]);
                }
                i += 1;
            }
        }
    }
    vocab
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "banana";
    let chars: BTreeMap<_, _> = text
        .chars()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(id, c)| (c, id))
        .collect();
    println!("Знакомый словарь символов: {chars:?}");
    println!("Символьные ID: {:?}", text.chars().map(|c| chars[&c]).collect::<Vec<_>>());
    let vocab = train(&["banana", "banana", "bandana"], 4);
    println!("Применяем уже обученный словарь к {text:?}:");
    println!("ID: {:?}", encode(text, &vocab, true));
    let bpe = tiktoken_rs::cl100k_base()?;
    for text in ["banana", " banana", "Привет, мир!", "🦀", "hello  world"] {
        let ids = bpe.encode_ordinary(text);
        println!("\nНастоящий cl100k_base: {text:?} -> {ids:?}");
        for &id in &ids {
            // Один токен может содержать только ЧАСТЬ UTF-8 символа.
            let bytes = bpe.decode_bytes(&[id])?;
            println!("  {id}: байты {bytes:?}, вид {:?}", String::from_utf8_lossy(&bytes));
        }
        assert_eq!(bpe.decode(&ids)?, text);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn training_learns_repeated_pieces() {
        let v = train(&["banana", "banana", "bandana"], 4);
        assert_eq!(v[b"an".as_slice()], 256);
        assert_eq!(encode("banana", &v, false), vec![259]);
        assert_eq!(v.len(), 260);
    }
    #[test]
    fn rank_beats_longest_match_and_unknown_unicode_roundtrips() {
        let mut v = vocabulary();
        v.insert(b"bc".to_vec(), 256);
        v.insert(b"bcd".to_vec(), 257);
        v.insert(b"ab".to_vec(), 258);
        v.insert(b"abc".to_vec(), 259);
        // Самый длинный префикс дал бы abc + d, но ранги дают a + bcd.
        assert_eq!(encode("abcd", &v, false), vec![97, 257]);
        let v = vocabulary();
        let bytes: Vec<_> = encode("я🦀", &v, false).into_iter().map(|id| id as u8).collect();
        assert_eq!(String::from_utf8(bytes).unwrap(), "я🦀");
        assert!(encode("", &v, false).is_empty());
    }
}
