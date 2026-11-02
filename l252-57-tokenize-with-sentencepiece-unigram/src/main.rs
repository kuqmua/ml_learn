// Маленький Unigram: каждая запись имеет ID и вероятность.
// Выбираем весь путь с максимальной суммой ln(p), а не длиннейший первый кусок.
fn segment<'a>(text: &str, vocab: &[(&'a str, f64)], trace: bool) -> Option<(f64, Vec<&'a str>)> {
    let mut best: Vec<Option<(f64, Vec<&str>)>> = vec![None; text.len() + 1];
    best[0] = Some((0.0, vec![]));
    for start in 0..text.len() {
        if !text.is_char_boundary(start) {
            continue;
        }
        let Some((score, path)) = best[start].clone() else { continue };
        for &(piece, probability) in vocab {
            assert!(!piece.is_empty() && probability > 0.0 && probability <= 1.0);
            if !text[start..].starts_with(piece) {
                continue;
            }
            let end = start + piece.len();
            let candidate = score + probability.ln();
            if trace {
                println!("  {start} -> {end}: {piece:?}, сумма ln(p) = {candidate:.3}");
            }
            if best[end].as_ref().is_none_or(|(old, _)| candidate > *old) {
                let mut next = path.clone();
                next.push(piece);
                best[end] = Some((candidate, next));
            }
        }
    }
    best.pop().flatten()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Вероятности суммируются в 1. ID — просто индекс в этом массиве.
    let vocab = [("▁", 0.10), ("a", 0.35), ("b", 0.35), ("ab", 0.05), ("▁ab", 0.15)];
    println!("Учебный словарь (ID, кусок, p):");
    for (id, entry) in vocab.iter().enumerate() {
        println!("  {id}: {entry:?}");
    }
    let (_, pieces) = segment("▁ab", &vocab, true).unwrap();
    println!("Лучший путь: {pieces:?}");
    // ▁ab: 0.15; ▁ + ab: 0.005; ▁ + a + b: 0.01225.
    // Меняем вероятности: даже существующий длинный токен может проиграть.
    let changed = [("▁", 0.10), ("a", 0.44), ("b", 0.44), ("ab", 0.01), ("▁ab", 0.01)];
    println!("После изменения p: {:?}", segment("▁ab", &changed, false).unwrap());
    let sp = sentencepiece_rs::SentencePieceProcessor::from_serialized_model(include_bytes!("../data/toy.model"))?;
    for text in ["banana", "Привет мир", "  banana   banana  ", "ＢＡＮＡＮＡ", "🦀", ""] {
        println!("\nSentencePiece: {text:?}");
        println!("  нормализация: {:?}", sp.normalize(text)?);
        println!("  куски: {:?}", sp.encode(text)?);
        let ids = sp.encode_to_ids(text)?;
        println!("  ID: {ids:?}; decode: {:?}", sp.decode_ids(&ids)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn global_path_beats_longest_piece_and_handles_unicode() {
        let vocab = [("я", 0.45), ("м", 0.45), ("ям", 0.10)];
        assert_eq!(segment("ям", &vocab, false).unwrap().1, vec!["я", "м"]);
        assert!(segment("x", &vocab, false).is_none());
        assert_eq!(segment("", &vocab, false).unwrap().1, Vec::<&str>::new());
    }
    #[test]
    fn matches_official_sentencepiece_021_fixture() {
        let sp = sentencepiece_rs::SentencePieceProcessor::from_serialized_model(include_bytes!("../data/toy.model")).unwrap();
        for line in include_str!("../data/reference.tsv").lines() {
            let columns: Vec<_> = line.split('\t').collect();
            let expected: Vec<usize> = columns[1].split(',').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
            let ids = sp.encode_to_ids(columns[0]).unwrap();
            assert_eq!(ids, expected, "{:?}", columns[0]);
            assert_eq!(sp.decode_ids(&ids).unwrap(), columns[2]);
        }
    }
}
