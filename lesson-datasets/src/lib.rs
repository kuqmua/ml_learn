//! Типизированные записи и воспроизводимое разделение открытых учебных наборов.
//! Подготовь файлы командой `python3 scripts/prepare_open_datasets.py all`.

use std::collections::BTreeMap;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

pub fn prepared_dataset_path(file_name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("пакет находится в корне workspace")
        .join("datasets/processed")
        .join(file_name)
}

fn read_prepared_dataset(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "{}: {error}; подготовь набор командой python3 scripts/prepare_open_datasets.py all",
                path.display()
            ),
        )
    })
}

fn invalid_data(line: usize, message: &str) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, format!("строка {line}: {message}"))
}

fn parse_finite_float(value: &str, line: usize, field: &str) -> io::Result<f64> {
    let parsed = value.parse::<f64>().map_err(|_| {
        invalid_data(
            line,
            &format!("{field}: требуется число, получено {value:?}"),
        )
    })?;
    if !parsed.is_finite() {
        return Err(invalid_data(
            line,
            &format!("{field}: число должно быть конечным"),
        ));
    }
    Ok(parsed)
}

fn parse_fixed_features<const COUNT: usize>(
    fields: &[&str],
    line: usize,
    field: &str,
) -> io::Result<[f64; COUNT]> {
    let mut features = [0.0; COUNT];
    for (index, feature) in features.iter_mut().enumerate() {
        *feature = parse_finite_float(fields[index], line, field)?;
    }
    Ok(features)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrisSpecies {
    Setosa,
    Versicolor,
    Virginica,
}

impl IrisSpecies {
    pub fn class_identifier(self) -> u8 {
        match self {
            Self::Setosa => 0,
            Self::Versicolor => 1,
            Self::Virginica => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IrisRecord {
    /// Длина и ширина чашелистика, затем длина и ширина лепестка (сантиметры).
    pub features: [f64; 4],
    pub species: IrisSpecies,
}

pub fn load_iris_records() -> io::Result<Vec<IrisRecord>> {
    load_iris_records_from_path(&prepared_dataset_path("iris.csv"))
}

pub fn load_iris_records_from_path(path: &Path) -> io::Result<Vec<IrisRecord>> {
    parse_iris_records(&read_prepared_dataset(path)?)
}

fn parse_iris_records(text: &str) -> io::Result<Vec<IrisRecord>> {
    let mut lines = text.lines();
    if lines.next() != Some("sepal_length,sepal_width,petal_length,petal_width,species") {
        return Err(invalid_data(1, "неверный заголовок Iris"));
    }
    let mut records = Vec::new();
    for (index, line) in lines.enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 5 {
            return Err(invalid_data(number, "ожидалось четыре признака и класс"));
        }
        let features = parse_fixed_features(&fields, number, "признак Iris")?;
        let species = match fields[4] {
            "setosa" => IrisSpecies::Setosa,
            "versicolor" => IrisSpecies::Versicolor,
            "virginica" => IrisSpecies::Virginica,
            _ => return Err(invalid_data(number, "неизвестный класс Iris")),
        };
        records.push(IrisRecord { features, species });
    }
    if records.is_empty() {
        return Err(invalid_data(2, "набор Iris пуст"));
    }
    Ok(records)
}

#[derive(Clone, Debug, PartialEq)]
pub struct WineQualityRedRecord {
    /// Порядок одиннадцати признаков указан в DATASETS.md.
    pub features: [f64; 11],
    pub quality: f64,
}

pub fn load_wine_quality_red_records() -> io::Result<Vec<WineQualityRedRecord>> {
    load_wine_quality_red_records_from_path(&prepared_dataset_path("wine_quality_red.csv"))
}

pub fn load_wine_quality_red_records_from_path(
    path: &Path,
) -> io::Result<Vec<WineQualityRedRecord>> {
    parse_wine_quality_red_records(&read_prepared_dataset(path)?)
}

fn parse_wine_quality_red_records(text: &str) -> io::Result<Vec<WineQualityRedRecord>> {
    let mut lines = text.lines();
    if lines.next()
        != Some(
            "fixed_acidity,volatile_acidity,citric_acid,residual_sugar,chlorides,free_sulfur_dioxide,total_sulfur_dioxide,density,ph,sulphates,alcohol,quality",
        )
    {
        return Err(invalid_data(1, "неверный заголовок Wine Quality"));
    }
    let mut records = Vec::new();
    for (index, line) in lines.enumerate() {
        let number = index + 2;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 12 {
            return Err(invalid_data(
                number,
                "ожидалось одиннадцать признаков и оценка качества",
            ));
        }
        let features = parse_fixed_features(&fields, number, "признак Wine Quality")?;
        let quality = parse_finite_float(fields[11], number, "quality")?;
        records.push(WineQualityRedRecord { features, quality });
    }
    if records.is_empty() {
        return Err(invalid_data(2, "набор Wine Quality пуст"));
    }
    Ok(records)
}

#[derive(Clone, Debug, PartialEq)]
pub struct SmsSpamRecord {
    pub is_spam: bool,
    pub message: String,
}

pub fn load_sms_spam_records() -> io::Result<Vec<SmsSpamRecord>> {
    load_sms_spam_records_from_path(&prepared_dataset_path("sms_spam.tsv"))
}

pub fn load_sms_spam_records_from_path(path: &Path) -> io::Result<Vec<SmsSpamRecord>> {
    parse_sms_spam_records(&read_prepared_dataset(path)?)
}

fn parse_sms_spam_records(text: &str) -> io::Result<Vec<SmsSpamRecord>> {
    let mut lines = text.lines();
    if lines.next() != Some("is_spam\tmessage") {
        return Err(invalid_data(1, "неверный заголовок SMS Spam"));
    }
    let mut records = Vec::new();
    for (index, line) in lines.enumerate() {
        let number = index + 2;
        let (label, message) = line
            .split_once('\t')
            .ok_or_else(|| invalid_data(number, "нужны метка и сообщение"))?;
        let is_spam = match label {
            "0" => false,
            "1" => true,
            _ => return Err(invalid_data(number, "метка должна быть 0 или 1")),
        };
        if message.is_empty() || message.contains('\t') {
            return Err(invalid_data(
                number,
                "сообщение пусто или содержит табуляцию",
            ));
        }
        records.push(SmsSpamRecord {
            is_spam,
            message: message.to_owned(),
        });
    }
    if records.is_empty() {
        return Err(invalid_data(2, "набор SMS Spam пуст"));
    }
    Ok(records)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatasetSplit {
    pub training_indices: Vec<usize>,
    pub validation_indices: Vec<usize>,
    pub test_indices: Vec<usize>,
}

fn shuffle_indices(indices: &mut [usize], seed: u64) {
    let mut state = seed;
    for index in (1..indices.len()).rev() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        indices.swap(index, (state as usize) % (index + 1));
    }
}

fn append_split_group(split: &mut DatasetSplit, mut indices: Vec<usize>, seed: u64) {
    shuffle_indices(&mut indices, seed);
    let training_end = indices.len() * 70 / 100;
    let validation_end = training_end + indices.len() * 15 / 100;
    split
        .training_indices
        .extend_from_slice(&indices[..training_end]);
    split
        .validation_indices
        .extend_from_slice(&indices[training_end..validation_end]);
    split
        .test_indices
        .extend_from_slice(&indices[validation_end..]);
}

/// Фиксированное разделение 70/15/15. Сохраняй seed рядом с метриками.
pub fn split_indices(number_of_records: usize, seed: u64) -> Result<DatasetSplit, &'static str> {
    if number_of_records < 7 {
        return Err("для трёх частей требуется хотя бы семь записей");
    }
    let mut split = DatasetSplit {
        training_indices: Vec::new(),
        validation_indices: Vec::new(),
        test_indices: Vec::new(),
    };
    append_split_group(&mut split, (0..number_of_records).collect(), seed);
    Ok(split)
}

/// Делит каждый класс отдельно, чтобы редкий класс был в каждой части.
pub fn split_indices_stratified_by_class(
    class_identifiers: &[u8],
    seed: u64,
) -> Result<DatasetSplit, &'static str> {
    let mut groups: BTreeMap<u8, Vec<usize>> = BTreeMap::new();
    for (index, &class_identifier) in class_identifiers.iter().enumerate() {
        groups.entry(class_identifier).or_default().push(index);
    }
    if groups.is_empty() || groups.values().any(|indices| indices.len() < 7) {
        return Err("для стратификации нужно хотя бы семь записей каждого класса");
    }
    let mut split = DatasetSplit {
        training_indices: Vec::new(),
        validation_indices: Vec::new(),
        test_indices: Vec::new(),
    };
    for (&class_identifier, indices) in &groups {
        append_split_group(
            &mut split,
            indices.clone(),
            seed ^ u64::from(class_identifier),
        );
    }
    shuffle_indices(&mut split.training_indices, seed ^ 0x11);
    shuffle_indices(&mut split.validation_indices, seed ^ 0x22);
    shuffle_indices(&mut split.test_indices, seed ^ 0x33);
    Ok(split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typed_records_and_rejects_wrong_schema() {
        let iris = parse_iris_records(
            "sepal_length,sepal_width,petal_length,petal_width,species\n5.1,3.5,1.4,0.2,setosa\n",
        )
        .unwrap();
        assert_eq!(iris[0].features, [5.1, 3.5, 1.4, 0.2]);
        assert_eq!(iris[0].species.class_identifier(), 0);
        assert!(parse_iris_records("wrong\n5.1,3.5,1.4,0.2,setosa\n").is_err());
        let sms = parse_sms_spam_records("is_spam\tmessage\n1\twin now\n").unwrap();
        assert!(sms[0].is_spam);
        assert_eq!(sms[0].message, "win now");
        let wine = parse_wine_quality_red_records(concat!(
            "fixed_acidity,volatile_acidity,citric_acid,residual_sugar,chlorides,",
            "free_sulfur_dioxide,total_sulfur_dioxide,density,ph,sulphates,alcohol,quality\n",
            "7.4,0.7,0.0,1.9,0.076,11.0,34.0,0.9978,3.51,0.56,9.4,5\n",
        ))
        .unwrap();
        assert_eq!(wine[0].features[10], 9.4);
        assert_eq!(wine[0].quality, 5.0);
        assert!(parse_sms_spam_records("is_spam\tmessage\n2\tbad label\n").is_err());
    }

    #[test]
    fn seeded_stratified_split_has_no_overlap_and_contains_both_classes() {
        let labels: Vec<u8> = (0..20).map(|index| (index % 2) as u8).collect();
        let first = split_indices_stratified_by_class(&labels, 42).unwrap();
        assert_eq!(
            first,
            split_indices_stratified_by_class(&labels, 42).unwrap()
        );
        let mut all = [
            first.training_indices.clone(),
            first.validation_indices.clone(),
            first.test_indices.clone(),
        ]
        .concat();
        all.sort_unstable();
        assert_eq!(all, (0..20).collect::<Vec<_>>());
        for indices in [
            &first.training_indices,
            &first.validation_indices,
            &first.test_indices,
        ] {
            assert!(indices.iter().any(|&index| labels[index] == 0));
            assert!(indices.iter().any(|&index| labels[index] == 1));
        }
    }
}
