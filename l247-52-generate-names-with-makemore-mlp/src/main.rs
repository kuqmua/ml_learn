// Пример входа и результата этой функции.
// ```json
// {
//   "вход": "локально включённый data/names.txt",
//   "выполнение": [
//     "подготовка данных",
//     "обучение",
//     "оценка",
//     "генерация"
//   ],
//   "успех": "Ok(())",
//   "ошибка": "любой Err через ? завершает программу"
// }
// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Как читать JSON ниже: это учебные значения, не дамп реального запуска.
    // ```json
    // {
    //   "условности": {
    //     "обёртки": "FiniteF32 и типы индексов показаны обычными JSON-числами",
    //     "точность": "числа округлены; f32 может дать немного другой последний разряд",
    //     "размеры": "в прямом и обратном проходах для наглядности EMBEDDING_DIMENSIONS=2, FIRST_LAYER_SIZE=2; рабочие значения в коде — 10 и 200",
    //     "словарь_малого_примера": [
    //       ".",
    //       "a",
    //       "n"
    //     ],
    //     "контекст_малого_примера": "ann",
    //     "правильный_символ_малого_примера": "a"
    //   },
    //   "связность_примеров": "Малый forward_pass, softmax, NLL и обратный проход используют одни данные. Примеры группировки, батчей и простого сложения градиентов — самостоятельные.",
    //   "порядок": [
    //     "проверить имена",
    //     "разделить по именам",
    //     "создать Vocabulary по train",
    //     "построить контекст и следующий символ",
    //     "вычислить оценки модели",
    //     "softmax и NLL",
    //     "накопить средние градиенты батча",
    //     "обновить параметры",
    //     "оценить dev/test и сгенерировать имена"
    //   ]
    // }
    // ```
    let program_started_at: std::time::Instant = std::time::Instant::now();
    /// Владеющий вектор, содержащий хотя бы один элемент.
    /// TryFrom отклоняет пустой вектор; map и try_map сохраняют длину. Deref предоставляет только срез.
    #[derive(Clone, Debug, derive_more::Deref, derive_more::IntoIterator)]
    #[deref(forward)]
    struct NonEmptyVec<T>(Vec<T>);
    // Проверка непустоты и сохранение длины при map.
    // ```json
    // {
    //   "вход_успешный": [7, 9],
    //   "результат_успешный": {
    //     "NonEmptyVec": [7, 9]
    //   },
    //   "вход_ошибочный": [],
    //   "ошибка": "вектор не должен быть пустым",
    //   "map_умножить_на_2": [14, 18],
    //   "длина_до_и_после": 2
    // }
    // ```
    impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
        type Error = &'static str;
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "values": [7],
        //   "результат": "Ok",
        //   "values_пустой": [],
        //   "результат_пустого": "Err"
        // }
        // ```
        fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "вход": [],
            //       "ветка": "Err"
            //     },
            //     {
            //       "вход": [7],
            //       "ветка": "Ok"
            //     }
            //   ],
            //   "назначение": "пустая коллекция не может стать NonEmptyVec или NonEmptySlice"
            // }
            // ```
            if values.is_empty() {
                Err("вектор не должен быть пустым")
            } else {
                Ok(Self(values))
            }
        }
    }
    impl<T> NonEmptyVec<T> {
        // Преобразует элементы, сохраняя их количество и непустоту.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "до": [1, 2],
        //   "transform": "умножить на 2",
        //   "после": [2, 4],
        //   "гарантия": "число элементов не изменяется, непустота сохраняется"
        // }
        // ```
        fn map<U>(&self, transform: impl FnMut(&T) -> U) -> NonEmptyVec<U> {
            NonEmptyVec(self.iter().map(transform).collect())
        }
    }
    impl<T> NonEmptyVec<T> {
        // Проверяет преобразование каждого элемента, сохраняя непустую форму.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "до": [1, 2],
        //   "transform": "FiniteF32::try_from",
        //   "после": [1, 2],
        //   "при_ошибке_одного_элемента": "Err вместо всей коллекции"
        // }
        // ```
        fn try_map<U>(
            &self,
            mut transform: impl FnMut(&T) -> Result<U, &'static str>,
        ) -> Result<NonEmptyVec<U>, &'static str> {
            Ok(NonEmptyVec(
                self.iter()
                    .map(&mut transform)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    // Поэлементное проверяемое преобразование без изменения длины массива.
    // Поэлементное преобразование массива: меняется тип значений, но не N.
    // ```json
    // {
    //   "N": 3,
    //   "вход_f32": [-0.2, 0.0, 0.7],
    //   "преобразование": "FiniteF32::try_from каждого элемента",
    //   "результат_FiniteF32": [-0.2, 0.0, 0.7],
    //   "при_неконечном_элементе": {
    //     "вход_символически": [
    //       0.0,
    //       "Infinity",
    //       0.7
    //     ],
    //     "результат": "Err; массив не возвращается"
    //   }
    // }
    // ```
    fn try_map_array<T, U, const N: usize>(
        values: [T; N],
        transform: impl FnMut(T) -> Result<U, &'static str>,
    ) -> Result<[U; N], &'static str> {
        values
            .into_iter()
            .map(transform)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_: Vec<U>| -> &'static str {
                "при преобразовании изменилась длина массива"
            })
    }
    // Словарь символов: точка под индексом 0, затем уникальные буквы a–z по алфавиту.
    // Словарь строится из букв train; индекс — позиция в этом массиве.
    // ```json
    // {
    //   "текст_train": "anna\nnan",
    //   "после_сортировки_и_удаления_повторов": [
    //     "a",
    //     "n"
    //   ],
    //   "Vocabulary": [
    //     ".",
    //     "a",
    //     "n"
    //   ],
    //   "индексы": {
    //     ".": 0,
    //     "a": 1,
    //     "n": 2
    //   },
    //   "dot_index": 0,
    //   "len": 3,
    //   "index_of_неизвестного_b": "Err"
    // }
    // ```
    struct Vocabulary(
        // Символы в порядке их индексов; создаются методом new.
        NonEmptyVec<char>,
    );
    impl Vocabulary {
        // Отбирает из текста буквы a–z, сортирует, удаляет повторы и добавляет точку под индексом 0.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "имена_train": [
        //     "anna",
        //     "nan"
        //   ],
        //   "после_join": "anna\nnan",
        //   "результат": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "dev_test": "не добавляют символы в словарь"
        // }
        // ```
        fn new(text: &str) -> Result<Self, &'static str> {
            // Пример значения и его получения.
            // ```json
            // {
            //   "text": "anna\nnan",
            //   "после_filter": [
            //     "a",
            //     "n",
            //     "n",
            //     "a",
            //     "n",
            //     "a",
            //     "n"
            //   ],
            //   "после_sort": [
            //     "a",
            //     "a",
            //     "a",
            //     "n",
            //     "n",
            //     "n",
            //     "n"
            //   ],
            //   "после_dedup": [
            //     "a",
            //     "n"
            //   ],
            //   "после_insert": [
            //     ".",
            //     "a",
            //     "n"
            //   ]
            // }
            // ```
            let mut characters: Vec<char> = text
                .chars()
                .filter(|character: &char| -> bool { character.is_ascii_lowercase() })
                .collect();
            characters.sort_unstable();
            characters.dedup();
            characters.insert(0, '.');
            Ok(Self(NonEmptyVec::try_from(characters)?))
        }
        // Возвращает количество символов, включая точку.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "результат": 3
        // }
        // ```
        fn len(&self) -> usize {
            self.0.len()
        }
        // Возвращает индекс точки: она обозначает границу имени и заполняет начальный контекст.
        // Метод new всегда помещает точку под индексом 0.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "результат": 0,
        //   "что_означает": "точка: начало/конец имени"
        // }
        // ```
        fn dot_index(&self) -> VocabularyCharacterIndexAsUsize {
            VocabularyCharacterIndexAsUsize(0)
        }
        // Находит индекс символа; для отсутствующего символа возвращает ошибку.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "character": "n",
        //   "position": 2,
        //   "передача_в_TryFrom": {
        //     "value": 2,
        //     "длина_словаря": 3
        //   }
        // }
        // ```
        fn index_of(
            &self,
            character: char,
        ) -> Result<VocabularyCharacterIndexAsUsize, &'static str> {
            // Пример значения и его получения.
            // ```json
            // {
            //   "Vocabulary": [
            //     ".",
            //     "a",
            //     "n"
            //   ],
            //   "character": "n",
            //   "position": 2,
            //   "передача_в_TryFrom": {
            //     "value": 2,
            //     "длина_словаря": 3
            //   }
            // }
            // ```
            let value: usize = self
                .0
                .iter()
                .position(|&candidate: &char| -> bool { candidate == character })
                .ok_or("символ отсутствует в словаре")?;
            VocabularyCharacterIndexAsUsize::try_from((value, self))
        }
        // Возвращает символ по числовой позиции, проверяя границы; принадлежность индекса словарю не проверяется.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "индекс": 2,
        //   "результат": "n",
        //   "индекс_3": "Err"
        // }
        // ```
        fn character(&self, index: VocabularyCharacterIndexAsUsize) -> Result<char, &'static str> {
            self.0
                .get(index.0)
                .copied()
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }
    // Числовой индекс символа Vocabulary; TryFrom проверяет границы конкретного словаря.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct VocabularyCharacterIndexAsUsize(
        // Значение от 0 до vocabulary.len() - 1 после TryFrom; сам usize не хранит связь со словарём.
        usize,
    );
    // Граница индекса зависит от конкретного Vocabulary, а не от предела usize.
    // ```json
    // {
    //   "Vocabulary": [
    //     ".",
    //     "a",
    //     "n"
    //   ],
    //   "проверки": [
    //     {
    //       "value": 0,
    //       "результат": "Ok; точка"
    //     },
    //     {
    //       "value": 2,
    //       "результат": "Ok; n"
    //     },
    //     {
    //       "value": 3,
    //       "результат": "Err"
    //     },
    //     {
    //       "value": 1000000,
    //       "результат": "Err"
    //     }
    //   ]
    // }
    // ```
    impl TryFrom<(usize, &Vocabulary)> for VocabularyCharacterIndexAsUsize {
        type Error = &'static str;
        // Создаёт индекс, только если value < vocabulary.len().
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "value": 2,
        //   "результат": "Ok(индекс 2)",
        //   "value_3": "Err"
        // }
        // ```
        fn try_from((value, vocabulary): (usize, &Vocabulary)) -> Result<Self, Self::Error> {
            vocabulary
                .0
                .get(value)
                .map(|_: &char| -> Self { Self(value) })
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }
    // Сколько предыдущих символов получает модель; значение должно быть положительным.
    const CONTEXT_LENGTH: usize = 3;
    // Обучающий пример в индексах Vocabulary: контекст длины CONTEXT_LENGTH и правильный следующий символ.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
        // Индексы предыдущих CONTEXT_LENGTH символов в Vocabulary; индекс точки заполняет начало имени.
        previous_vocabulary_character_usize_indexes:
            [VocabularyCharacterIndexAsUsize; CONTEXT_LENGTH],
        // Индекс правильного следующего символа в Vocabulary; индекс точки обозначает конец имени.
        next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize,
    }
    // Нулевое значение для координат, градиентов и накопительных сумм.
    const ZERO_F32: f32 = 0.0;
    // Число обучаемых координат одного символа; определяет длину каждой строки C.
    const EMBEDDING_DIMENSIONS: usize = 10;
    // Число компонентов результата первого слоя и длина b1.
    const FIRST_LAYER_SIZE: usize = 200;
    // Число примеров, по которым усредняются градиенты одного шага.
    const BATCH_SIZE: usize = 32;
    // Проверяем положительность размеров при компиляции.
    const _: () = assert!(
        CONTEXT_LENGTH > 0 && EMBEDDING_DIMENSIONS > 0 && FIRST_LAYER_SIZE > 0 && BATCH_SIZE > 0
    );
    // Положительное число обновлений параметров; для нуля возвращается ошибка.
    // Пример значения и его получения.
    // ```json
    // {
    //   "вход": 200000,
    //   "результат": "NonZeroUsize(200000)",
    //   "если_вход_0": "Err; обучение не начинается"
    // }
    // ```
    let training_steps: std::num::NonZeroUsize =
        std::num::NonZeroUsize::new(200_000).ok_or("число шагов должно быть положительным")?;
    // Один seed и общий генератор для всех случайных операций программы.
    // Пример значения и его получения.
    // ```json
    // {
    //   "seed": 42,
    //   "использование": [
    //     "первичное перемешивание групп имён",
    //     "инициализация параметров",
    //     "выбор символов при генерации"
    //   ],
    //   "батчи": "последовательные, RNG в них не используется",
    //   "повторный_запуск": "тот же seed и те же операции дают ту же последовательность"
    // }
    // ```
    let mut random_generator: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(42);
    // Пример значения и его получения.
    // ```json
    // {
    //   "число_групп": 10,
    //   "граница": 8,
    //   "training_name_groups": "позиции 0–7",
    //   "remaining_name_groups": "позиции 8–9",
    //   "проверка": "имя не разрывается между группами"
    // }
    // ```
    let (
        vocabulary,
        training_character_index_pairs,
        validation_character_index_pairs,
        test_character_index_pairs,
    ): (
        Vocabulary,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
    ) = {
        // 1. Проверяем записи; повторения сохраняем, чтобы не менять их частоты в наборе.
        // Пример значения и его получения.
        // ```json
        // {
        //   "исходный_файл": "data/names.txt",
        //   "пример_текста": "anna\nnan\n",
        //   "после_lines": [
        //     "anna",
        //     "nan"
        //   ],
        //   "чтение": "include_str! включает текст в программу при сборке"
        // }
        // ```
        let names_text: &str = include_str!("../data/names.txt");
        // Одно непустое имя из строчных латинских букв; заимствует исходный текст.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct Name<'a>(&'a str);
        // Проверяется каждое имя отдельно; дубликаты корректных имён сохраняются.
        // ```json
        // {
        //   "проверки": [
        //     {
        //       "имя": "anna",
        //       "результат": "Ok"
        //     },
        //     {
        //       "имя": "an",
        //       "результат": "Ok; имя может быть короче CONTEXT_LENGTH"
        //     },
        //     {
        //       "имя": "",
        //       "результат": "Err"
        //     },
        //     {
        //       "имя": "Anna",
        //       "результат": "Err"
        //     },
        //     {
        //       "имя": "a1",
        //       "результат": "Err"
        //     },
        //     {
        //       "имя": "анна",
        //       "результат": "Err"
        //     }
        //   ]
        // }
        // ```
        impl<'a> TryFrom<&'a str> for Name<'a> {
            type Error = &'static str;
            // Пример входа и результата этой функции.
            // ```json
            // {
            //   "text": "anna",
            //   "результат": "Ok(Name)",
            //   "text_Anna": "Err"
            // }
            // ```
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                // Пример выбора ветки.
                // ```json
                // {
                //   "проверки": [
                //     {
                //       "text": "anna",
                //       "ветка": "Ok"
                //     },
                //     {
                //       "text": "",
                //       "ветка": "Err"
                //     },
                //     {
                //       "text": "Anna",
                //       "ветка": "Err"
                //     },
                //     {
                //       "text": "анна",
                //       "ветка": "Err"
                //     }
                //   ]
                // }
                // ```
                if text.is_empty()
                    || !text
                        .chars()
                        .all(|character: char| -> bool { character.is_ascii_lowercase() })
                {
                    Err("имя должно быть непустым и содержать только буквы a–z")
                } else {
                    Ok(Self(text))
                }
            }
        }
        impl std::fmt::Debug for Name<'_> {
            // Выводит исходную строку имени без дополнительной обёртки.
            // Пример входа и результата этой функции.
            // ```json
            // {
            //   "Name": "anna",
            //   "Debug_вывод": "\"anna\"",
            //   "дополнительное_Name_в_логе": false
            // }
            // ```
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self.0, formatter)
            }
        }
        // Пример значения и его получения.
        // ```json
        // {
        //   "text": "anna\nnan\n",
        //   "результат": [
        //     "anna",
        //     "nan"
        //   ],
        //   "проверка_каждого": "Name::try_from",
        //   "пустой_текст": "Err через NonEmptyVec::try_from"
        // }
        // ```
        let names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            names_text
                .lines()
                .map(Name::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        println!(
            "Имена: {}, первые восемь: {:?}",
            names.len(),
            &names[..names.len().min(8)]
        );
        // 2. Объединяем одинаковые имена, чтобы их копии не оказались одновременно в train/dev/test.
        // BTreeMap задаёт стабильный порядок групп перед воспроизводимым перемешиванием с seed 42.
        // Группируем одинаковые имена до разделения: копии не должны попасть в разные выборки.
        // ```json
        // {
        //   "исходные_имена": [
        //     "anna",
        //     "nan",
        //     "anna",
        //     "an"
        //   ],
        //   "BTreeMap": {
        //     "an": [
        //       "an"
        //     ],
        //     "anna": [
        //       "anna",
        //       "anna"
        //     ],
        //     "nan": [
        //       "nan"
        //     ]
        //   },
        //   "группы_до_перемешивания": [
        //     [
        //       "an"
        //     ],
        //     [
        //       "anna",
        //       "anna"
        //     ],
        //     [
        //       "nan"
        //     ]
        //   ],
        //   "одна_возможная_перестановка": [
        //     [
        //       "nan"
        //     ],
        //     [
        //       "anna",
        //       "anna"
        //     ],
        //     [
        //       "an"
        //     ]
        //   ],
        //   "важно": "это пример возможного порядка, а не обещание результата shuffle с seed 42"
        // }
        // ```
        let mut name_groups: NonEmptyVec<NonEmptyVec<Name<'_>>> = {
            // Пример значения и его получения.
            // ```json
            // {
            //   "начальное_значение": {},
            //   "после_первой_anna": {
            //     "anna": [
            //       "anna"
            //     ]
            //   },
            //   "после_второй_anna": {
            //     "anna": [
            //       "anna",
            //       "anna"
            //     ]
            //   },
            //   "смысл": "ключ и значения содержат Name; одинаковое имя хранится в одной группе"
            // }
            // ```
            let mut groups: std::collections::BTreeMap<Name<'_>, Vec<Name<'_>>> =
                std::collections::BTreeMap::new();
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "names": [
            //     "anna",
            //     "anna",
            //     "nan"
            //   ],
            //   "итерации": [
            //     "добавить первую anna в группу anna",
            //     "добавить вторую anna в ту же группу",
            //     "создать группу nan"
            //   ]
            // }
            // ```
            for iteration_value in names.iter() {
                // Пример значения и его получения.
                // ```json
                // {
                //   "iteration_value": "anna",
                //   "name": "anna",
                //   "операция": "получаем проверенный Name; строка не копируется"
                // }
                // ```
                let &name: &Name<'_> = iteration_value;
                groups.entry(name).or_default().push(name);
            }
            NonEmptyVec::try_from(
                groups
                    .into_values()
                    .map(NonEmptyVec::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )?
        };
        rand::seq::SliceRandom::shuffle(name_groups.0.as_mut_slice(), &mut random_generator);
        // Доли относятся к группам уникальных имён, а не к количеству букв или пар.
        // ```json
        // {
        //   "число_групп": 10,
        //   "training_name_groups": "группы с позициями 0–7",
        //   "validation_name_groups": "группа с позицией 8",
        //   "test_name_groups": "группа с позицией 9",
        //   "если_anna_повторяется_дважды": "оба anna остаются в одной группе и в одной выборке",
        //   "Vocabulary": "создаётся только из training_names",
        //   "при_неизвестной_букве_в_dev": "Err, словарь не дополняется из dev"
        // }
        // ```
        // Доли 80/10/10 относятся к уникальным именам; число записей зависит от размеров групп.
        let (training_name_groups, remaining_name_groups): (
            &[NonEmptyVec<Name<'_>>],
            &[NonEmptyVec<Name<'_>>],
        ) = name_groups.split_at(name_groups.len() * 8 / 10);
        // Пример значения и его получения.
        // ```json
        // {
        //   "число_всех_групп": 10,
        //   "remaining": "позиции 8–9",
        //   "граница_внутри_remaining": 1,
        //   "validation": "позиция 8",
        //   "test": "позиция 9"
        // }
        // ```
        let (validation_name_groups, test_name_groups): (
            &[NonEmptyVec<Name<'_>>],
            &[NonEmptyVec<Name<'_>>],
        ) = remaining_name_groups.split_at(name_groups.len() * 9 / 10 - training_name_groups.len());
        // Пример значения и его получения.
        // ```json
        // {
        //   "пример_групп": [
        //     [
        //       "anna",
        //       "anna"
        //     ],
        //     [
        //       "nan"
        //     ]
        //   ],
        //   "после_flat_map": [
        //     "anna",
        //     "anna",
        //     "nan"
        //   ],
        //   "копирование": "копируются Name со ссылками на текст, а не сами строки",
        //   "проверка": "пустая выборка возвращает Err"
        // }
        // ```
        let training_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            training_name_groups
                .iter()
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        // Пример значения и его получения.
        // ```json
        // {
        //   "пример_групп": [
        //     [
        //       "anna",
        //       "anna"
        //     ],
        //     [
        //       "nan"
        //     ]
        //   ],
        //   "после_flat_map": [
        //     "anna",
        //     "anna",
        //     "nan"
        //   ],
        //   "копирование": "копируются Name со ссылками на текст, а не сами строки",
        //   "проверка": "пустая выборка возвращает Err"
        // }
        // ```
        let validation_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            validation_name_groups
                .iter()
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        // Пример значения и его получения.
        // ```json
        // {
        //   "пример_групп": [
        //     [
        //       "anna",
        //       "anna"
        //     ],
        //     [
        //       "nan"
        //     ]
        //   ],
        //   "после_flat_map": [
        //     "anna",
        //     "anna",
        //     "nan"
        //   ],
        //   "копирование": "копируются Name со ссылками на текст, а не сами строки",
        //   "проверка": "пустая выборка возвращает Err"
        // }
        // ```
        let test_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            test_name_groups
                .iter()
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        // Состав Vocabulary определяем только по train. Неизвестный символ в dev/test вернёт ошибку.
        // Пример значения и его получения.
        // ```json
        // {
        //   "имена_train": [
        //     "anna",
        //     "nan"
        //   ],
        //   "после_join": "anna\nnan",
        //   "результат": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "dev_test": "не добавляют символы в словарь"
        // }
        // ```
        let vocabulary: Vocabulary = Vocabulary::new(
            &training_names
                .map(|name: &Name<'_>| -> &str { name.0 })
                .join("\n"),
        )?;
        println!("Словарь (только train): {:?}", vocabulary.0);
        // Захватывает Vocabulary: для каждого имени сбрасывает контекст и строит пары «контекст → следующий символ».
        // Вход и результат непустые; точка в конце учит завершать имя, неизвестный символ возвращает ошибку.
        // Из anna получаются пять примеров; числа обозначают позиции в Vocabulary.
        // ```json
        // {
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "name": "anna",
        //   "перебираемые_символы": [
        //     "a",
        //     "n",
        //     "n",
        //     "a",
        //     "."
        //   ],
        //   "пары": [
        //     {
        //       "previous_vocabulary_character_usize_indexes": [0, 0, 0],
        //       "next_vocabulary_character_usize_index": 1,
        //       "читается": "... → a"
        //     },
        //     {
        //       "previous_vocabulary_character_usize_indexes": [0, 0, 1],
        //       "next_vocabulary_character_usize_index": 2,
        //       "читается": "..a → n"
        //     },
        //     {
        //       "previous_vocabulary_character_usize_indexes": [0, 1, 2],
        //       "next_vocabulary_character_usize_index": 2,
        //       "читается": ".an → n"
        //     },
        //     {
        //       "previous_vocabulary_character_usize_indexes": [1, 2, 2],
        //       "next_vocabulary_character_usize_index": 1,
        //       "читается": "ann → a"
        //     },
        //     {
        //       "previous_vocabulary_character_usize_indexes": [2, 2, 1],
        //       "next_vocabulary_character_usize_index": 0,
        //       "читается": "nna → ."
        //     }
        //   ],
        //   "перед_следующим_именем": "контекст заново заполняется [0, 0, 0]"
        // }
        // ```
        let build_previous_vocabulary_character_indexes_and_next_character_indexes =
            |names: NonEmptySlice<'_, Name<'_>>| -> Result<
                NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
                &'static str,
            > {
                // Пример значения и его получения.
                // ```json
                // {
                //   "начало": [],
                //   "после_первого_push": [
                //     {
                //       "previous_vocabulary_character_usize_indexes": [0, 0, 0],
                //       "next_vocabulary_character_usize_index": 1
                //     }
                //   ],
                //   "после_anna": "пять пар; исходный контекст копируется при каждом push"
                // }
                // ```
                let mut character_index_pairs: Vec<
                    PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                > = Vec::new();
                // Пример итераций этого цикла.
                // ```json
                // {
                //   "names": [
                //     "anna",
                //     "nan"
                //   ],
                //   "после_anna_число_пар": 5,
                //   "после_nan_число_пар": 9,
                //   "между_именами": "контекст сбрасывается в точки"
                // }
                // ```
                for iteration_value in names.iter() {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "iteration_value": "anna",
                    //   "name": "anna",
                    //   "операция": "получаем проверенный Name; строка не копируется"
                    // }
                    // ```
                    let name: &Name<'_> = iteration_value;
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "CONTEXT_LENGTH": 3,
                    //   "dot_index": 0,
                    //   "начало": [0, 0, 0],
                    //   "после_a": [0, 0, 1],
                    //   "после_n": [0, 1, 2]
                    // }
                    // ```
                    let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                    CONTEXT_LENGTH] = [vocabulary.dot_index(); CONTEXT_LENGTH];
                    // Пример итераций этого цикла.
                    // ```json
                    // {
                    //   "name": "anna",
                    //   "итерации": [
                    //     "a",
                    //     "n",
                    //     "n",
                    //     "a",
                    //     "."
                    //   ],
                    //   "за_итерацию": "одна новая пара контекст/цель и сдвиг контекста"
                    // }
                    // ```
                    for iteration_value in name.0.chars().chain(std::iter::once('.')) {
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "имя": "anna",
                        //   "значения_итераций": [
                        //     "a",
                        //     "n",
                        //     "n",
                        //     "a",
                        //     "."
                        //   ],
                        //   "точка": "chain добавляет её после последней буквы"
                        // }
                        // ```
                        let next_character: char = iteration_value;
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "Vocabulary": [
                        //     ".",
                        //     "a",
                        //     "n"
                        //   ],
                        //   "next_character": "n",
                        //   "результат": 2,
                        //   "если_символ_b": "Err; index_of не находит символ"
                        // }
                        // ```
                        let next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize =
                            vocabulary.index_of(next_character)?;
                        character_index_pairs.push(
                            PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
                                previous_vocabulary_character_usize_indexes,
                                next_vocabulary_character_usize_index,
                            },
                        );
                        // Сначала сохраняется копия контекста, затем контекст сдвигается.
                        // ```json
                        // {
                        //   "контекст_до": [0, 1, 2],
                        //   "следующий_индекс": 2,
                        //   "сохранённая_пара": {
                        //     "контекст": [0, 1, 2],
                        //     "цель": 2
                        //   },
                        //   "после_rotate_left": [1, 2, 0],
                        //   "после_замены_последнего": [1, 2, 2],
                        //   "сохранённая_пара_после_сдвига": "остаётся прежней; массив был скопирован"
                        // }
                        // ```
                        previous_vocabulary_character_usize_indexes.rotate_left(1);
                        // Пример изменения значения.
                        // ```json
                        // {
                        //   "до_замены_последнего": [1, 2, 0],
                        //   "новый_индекс": 2,
                        //   "после": [1, 2, 2],
                        //   "последний_индекс_массива": 2
                        // }
                        // ```
                        previous_vocabulary_character_usize_indexes[CONTEXT_LENGTH - 1] =
                            next_vocabulary_character_usize_index;
                    }
                }
                NonEmptyVec::try_from(character_index_pairs)
            };
        // Пример значения и его получения.
        // ```json
        // {
        //   "выборка": "training",
        //   "имена": [
        //     "anna"
        //   ],
        //   "число_пар": 5,
        //   "одна_пара": {
        //     "previous_vocabulary_character_usize_indexes": [1, 2, 2],
        //     "next_vocabulary_character_usize_index": 1
        //   },
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ]
        // }
        // ```
        let training_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&training_names),
        )?;
        // Пример значения и его получения.
        // ```json
        // {
        //   "выборка": "validation",
        //   "имена": [
        //     "anna"
        //   ],
        //   "число_пар": 5,
        //   "одна_пара": {
        //     "previous_vocabulary_character_usize_indexes": [1, 2, 2],
        //     "next_vocabulary_character_usize_index": 1
        //   },
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ]
        // }
        // ```
        let validation_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&validation_names),
        )?;
        // Пример значения и его получения.
        // ```json
        // {
        //   "выборка": "test",
        //   "имена": [
        //     "anna"
        //   ],
        //   "число_пар": 5,
        //   "одна_пара": {
        //     "previous_vocabulary_character_usize_indexes": [1, 2, 2],
        //     "next_vocabulary_character_usize_index": 1
        //   },
        //   "Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ]
        // }
        // ```
        let test_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&test_names),
        )?;
        println!(
            "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
            training_names.len(),
            validation_names.len(),
            test_names.len(),
            training_character_index_pairs.len(),
            validation_character_index_pairs.len(),
            test_character_index_pairs.len()
        );
        (
            vocabulary,
            training_character_index_pairs,
            validation_character_index_pairs,
            test_character_index_pairs,
        )
    };
    // Пример значения и его получения.
    // ```json
    // {
    //   "Vocabulary": [
    //     ".",
    //     "a",
    //     "n"
    //   ],
    //   "len": 3,
    //   "рабочий_набор": "обычно 27: точка и все буквы a–z",
    //   "минимум": 1,
    //   "максимум": 27
    // }
    // ```
    let vocabulary_len: usize = vocabulary.len();
    // Обёртка над срезом, для которого уже проверено наличие хотя бы одного элемента.
    #[derive(Debug, derive_more::Deref)]
    #[deref(forward)]
    #[derive_where::derive_where(Clone, Copy)]
    struct NonEmptySlice<'a, T>(
        // Заимствованный срез: проверка не копирует его элементы.
        &'a [T],
    );
    impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
        type Error = &'static str;
        // Отклоняет пустой срез и оборачивает непустой без копирования элементов.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "values": [7],
        //   "результат": "Ok",
        //   "values_пустой": [],
        //   "результат_пустого": "Err"
        // }
        // ```
        fn try_from(values: &'a [T]) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "вход": [],
            //       "ветка": "Err"
            //     },
            //     {
            //       "вход": [7],
            //       "ветка": "Ok"
            //     }
            //   ],
            //   "назначение": "пустая коллекция не может стать NonEmptyVec или NonEmptySlice"
            // }
            // ```
            if values.is_empty() {
                Err("срез не должен быть пустым")
            } else {
                Ok(Self(values))
            }
        }
    }
    impl<'a, T> From<&'a NonEmptyVec<T>> for NonEmptySlice<'a, T> {
        // Заимствует непустой вектор без копирования и повторной проверки.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "вход_NonEmptyVec": [7, 9],
        //   "выход_NonEmptySlice": [7, 9],
        //   "копирование_элементов": false,
        //   "повторная_проверка_длины": false
        // }
        // ```
        fn from(values: &'a NonEmptyVec<T>) -> Self {
            Self(&values.0)
        }
    }
    /// Вероятность символа в диапазоне [0, 1]; NaN и бесконечности запрещены.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct CharacterProbabilityAsF32From0To1(f32);
    impl TryFrom<f32> for CharacterProbabilityAsF32From0To1 {
        type Error = &'static str;
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "проверки": [
        //     {
        //       "value": 0.3,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": 0,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": 1,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": -0.1,
        //       "ветка": "Err"
        //     },
        //     {
        //       "value": "NaN",
        //       "ветка": "Err"
        //     }
        //   ]
        // }
        // ```
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "value": 0.3,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": 0,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": 1,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": -0.1,
            //       "ветка": "Err"
            //     },
            //     {
            //       "value": "NaN",
            //       "ветка": "Err"
            //     }
            //   ]
            // }
            // ```
            if (0.0..=1.0).contains(&value) {
                Ok(Self(value))
            } else {
                Err("вероятность символа должна быть от 0 до 1")
            }
        }
    }
    /// Конечная неотрицательная ошибка предсказания; NaN и бесконечности запрещены.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct CrossEntropyLossAsNonNegativeF32(f32);
    impl TryFrom<f32> for CrossEntropyLossAsNonNegativeF32 {
        type Error = &'static str;
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "проверки": [
        //     {
        //       "value": 0,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": 0.7,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": -0.1,
        //       "ветка": "Err"
        //     },
        //     {
        //       "value": "Infinity",
        //       "ветка": "Err"
        //     }
        //   ]
        // }
        // ```
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "value": 0,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": 0.7,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": -0.1,
            //       "ветка": "Err"
            //     },
            //     {
            //       "value": "Infinity",
            //       "ветка": "Err"
            //     }
            //   ]
            // }
            // ```
            if value.is_finite() && value >= 0.0 {
                Ok(Self(value))
            } else {
                Err("cross-entropy loss должен быть конечным и неотрицательным")
            }
        }
    }
    // Конечное число f32: NaN и обе бесконечности отклоняются при создании.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct FiniteF32(f32);
    // FiniteF32 ограничивает конечность, но не знак; JSON не поддерживает NaN и Infinity, поэтому они записаны строками.
    // ```json
    // {
    //   "FiniteF32": {
    //     "принимает": [-100.0, -0.2, 0.0, 0.3, 100.0],
    //     "отклоняет_символически": [
    //       "NaN",
    //       "Infinity",
    //       "-Infinity"
    //     ]
    //   },
    //   "CharacterProbabilityAsF32From0To1": {
    //     "принимает": [0.0, 0.3, 1.0],
    //     "отклоняет": [-0.1, 1.1]
    //   },
    //   "LearningRateAsPositiveFiniteF32": {
    //     "принимает": [0.01, 0.1],
    //     "отклоняет": [0.0, -0.1]
    //   }
    // }
    // ```
    impl TryFrom<f32> for FiniteF32 {
        type Error = &'static str;
        // Разрешает отрицательные числа, ноль и положительные числа; отклоняет NaN и ±∞.
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "проверки": [
        //     {
        //       "value": -2,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": 0,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": "Infinity",
        //       "ветка": "Err"
        //     },
        //     {
        //       "value": "NaN",
        //       "ветка": "Err"
        //     }
        //   ]
        // }
        // ```
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "value": -2,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": 0,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": "Infinity",
            //       "ветка": "Err"
            //     },
            //     {
            //       "value": "NaN",
            //       "ветка": "Err"
            //     }
            //   ]
            // }
            // ```
            if value.is_finite() {
                Ok(Self(value))
            } else {
                Err("значение должно быть конечным числом")
            }
        }
    }
    // Множитель градиента при обновлении параметров; допускается только конечное число строго больше нуля.
    #[derive(Clone, Copy, Debug)]
    struct LearningRateAsPositiveFiniteF32(f32);
    impl TryFrom<f32> for LearningRateAsPositiveFiniteF32 {
        type Error = &'static str;
        // Пример входа и результата этой функции.
        // ```json
        // {
        //   "проверки": [
        //     {
        //       "value": 0.1,
        //       "ветка": "Ok"
        //     },
        //     {
        //       "value": 0,
        //       "ветка": "Err"
        //     },
        //     {
        //       "value": -0.1,
        //       "ветка": "Err"
        //     },
        //     {
        //       "value": "NaN",
        //       "ветка": "Err"
        //     }
        //   ]
        // }
        // ```
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "проверки": [
            //     {
            //       "value": 0.1,
            //       "ветка": "Ok"
            //     },
            //     {
            //       "value": 0,
            //       "ветка": "Err"
            //     },
            //     {
            //       "value": -0.1,
            //       "ветка": "Err"
            //     },
            //     {
            //       "value": "NaN",
            //       "ветка": "Err"
            //     }
            //   ]
            // }
            // ```
            if value.is_finite() && value > ZERO_F32 {
                Ok(Self(value))
            } else {
                Err("скорость обучения должна быть положительным конечным числом")
            }
        }
    }
    // Результат cross-entropy для одного правильного следующего символа.
    struct NextCharacterProbabilitiesAndNegativeLogLikelihood {
        // Конечная неотрицательная ошибка: −ln(вероятности правильного символа).
        negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32,
        // Вероятности всех символов в порядке Vocabulary.
        next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1>,
    }
    // Для одного примера возвращает ошибку и вероятности символов.
    // loss — ошибка предсказания: -ln(вероятность правильного следующего символа).
    // Вход cross_entropy — оценки всех символов и индекс правильного символа; контекст уже учтён в forward_pass.
    // ```json
    // {
    //   "Vocabulary": [
    //     ".",
    //     "a",
    //     "n"
    //   ],
    //   "контекст": "ann",
    //   "character_scores": [0.078078769, 0.052101493, -0.264685993],
    //   "next_vocabulary_character_usize_index": 1,
    //   "правильный_символ": "a",
    //   "возвращаемый_тип": "NextCharacterProbabilitiesAndNegativeLogLikelihood",
    //   "результат": {
    //     "next_character_probabilities": [0.3725557, 0.36300234, 0.26444196],
    //     "negative_log_likelihood_loss": 1.013345998
    //   },
    //   "если_передать_цель_2": {
    //     "вероятности": "не изменятся",
    //     "negative_log_likelihood_loss": 1.330133485
    //   }
    // }
    // ```
    let cross_entropy = |character_scores: NonEmptySlice<'_, FiniteF32>,
                         next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize|
     -> Result<
        NextCharacterProbabilitiesAndNegativeLogLikelihood,
        &'static str,
    > {
        // Численный softmax для оценок из малого прямого прохода; порядок — точка, a, n.
        // ```json
        // {
        //   "character_scores": [0.078078769, 0.052101493, -0.264685993],
        //   "maximum_character_score": 0.078078769,
        //   "оценки_минус_максимум": [0.0, -0.025977275, -0.342764762],
        //   "exponentials_from_character_scores_minus_maximum": [1.0, 0.974357231, 0.709805165],
        //   "exponentials_from_character_scores_minus_maximum_sum": 2.684162396,
        //   "next_character_probabilities": [0.3725557, 0.36300234, 0.26444196],
        //   "сумма_вероятностей": 1.0
        // }
        // ```
        // Softmax: превращаем оценки символов в вероятности.
        // Максимум и сумму экспонент передаём в NLL для численно устойчивого расчёта.
        let (
            next_character_probabilities,
            maximum_character_score,
            exponentials_from_character_scores_minus_maximum_sum,
        ): (NonEmptyVec<CharacterProbabilityAsF32From0To1>, f32, f32) = {
            // Находим максимум непустого набора конечных оценок; −∞ служит только начальным накопителем fold.
            // Пример значения и его получения.
            // ```json
            // {
            //   "character_scores": [0, 2, 1],
            //   "начальный_накопитель": "-Infinity; только техническое стартовое значение",
            //   "после_каждого_сравнения": [0, 2, 2],
            //   "результат": 2
            // }
            // ```
            let maximum_character_score: f32 = character_scores
                .iter()
                .map(|score: &FiniteF32| -> f32 { score.0 })
                .fold(f32::NEG_INFINITY, f32::max);
            // Из каждой оценки вычитаем максимум: получаем значения ≤ 0, у максимумов — 0.
            // exp(оценка − максимум) даёт значения от 0 до 1; очень малые могут округлиться до 0.
            // Пример значения и его получения.
            // ```json
            // {
            //   "character_scores": [0, 2, 1],
            //   "maximum_character_score": 2,
            //   "разности": [-2, 0, -1],
            //   "после_exp": [0.135335283, 1, 0.367879441]
            // }
            // ```
            let exponentials_from_character_scores_minus_maximum: NonEmptyVec<f32> =
                NonEmptyVec::try_from(
                    character_scores
                        .iter()
                        .map(|character_score: &FiniteF32| -> f32 {
                            (character_score.0 - maximum_character_score).exp()
                        })
                        .collect::<Vec<_>>(),
                )?;
            // Сумма не меньше 1: хотя бы одна максимальная оценка даёт exp(0) = 1.
            // Пример значения и его получения.
            // ```json
            // {
            //   "экспоненты": [0.135335283, 1, 0.367879441],
            //   "сумма": 1.503214724,
            //   "зачем": "общий знаменатель для всех вероятностей"
            // }
            // ```
            let exponentials_from_character_scores_minus_maximum_sum: f32 =
                exponentials_from_character_scores_minus_maximum
                    .iter()
                    .sum();
            // Softmax: делим каждую экспоненту на общую сумму, сохраняя порядок символов Vocabulary.
            // Пример значения и его получения.
            // ```json
            // {
            //   "экспоненты": [0.135335283, 1, 0.367879441],
            //   "делитель": 1.503214724,
            //   "результат": [0.090030573, 0.665240956, 0.244728471],
            //   "порядок": [
            //     ".",
            //     "a",
            //     "n"
            //   ],
            //   "сумма_примерно": 1
            // }
            // ```
            let next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1> =
                NonEmptyVec::try_from(
                    exponentials_from_character_scores_minus_maximum
                        .into_iter()
                        .map(|value: f32| -> Result<CharacterProbabilityAsF32From0To1, &'static str> {
                            CharacterProbabilityAsF32From0To1::try_from(
                                value / exponentials_from_character_scores_minus_maximum_sum,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
            (
                next_character_probabilities,
                maximum_character_score,
                exponentials_from_character_scores_minus_maximum_sum,
            )
        };
        // NLL можно получить из вероятности, но код использует устойчивую формулу из исходных оценок.
        // ```json
        // {
        //   "правильный_индекс": 1,
        //   "target_character_score": 0.052101493,
        //   "вероятность_правильного_символа": 0.36300234,
        //   "прямой_способ": {
        //     "формула": "-ln(p)",
        //     "значение": 1.013345998
        //   },
        //   "устойчивый_способ": {
        //     "формула": "maximum_character_score - target_character_score + ln(sum_exp)",
        //     "maximum_minus_target": 0.025977275,
        //     "ln_sum_exp": 0.987368723,
        //     "результат": 1.013345998
        //   },
        //   "почему_так": "не берём логарифм вероятности, которая могла округлиться до нуля"
        // }
        // ```
        // Negative log-likelihood (NLL): −ln вероятности правильного следующего символа.
        let negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32 = {
            // Берём исходную оценку правильного следующего символа, до вычитания максимума.
            // Пример значения и его получения.
            // ```json
            // {
            //   "character_scores": [0, 2, 1],
            //   "next_vocabulary_character_usize_index": 1,
            //   "выбранная_оценка": 2,
            //   "важно": "это исходная оценка, не вероятность и не оценка после вычитания максимума"
            // }
            // ```
            let target_character_score: f32 = character_scores
                .get(next_vocabulary_character_usize_index.0)
                .ok_or("индекс целевого символа выходит за границы оценок")?
                .0;
            // negative_log_likelihood_loss = −ln(p правильного символа).
            // p = exp(target_character_score − maximum_character_score) / сумма экспонент.
            // Раскрывая −ln(p), получаем максимум − целевую оценку + ln(сумма).
            // Считаем напрямую: крошечная вероятность может округлиться до 0, и ln(0) дал бы −∞.
            // Пример значения и его получения.
            // ```json
            // {
            //   "maximum_character_score": 2,
            //   "target_character_score": 2,
            //   "ln_суммы_экспонент": 0.407605964,
            //   "формула": "2 - 2 + 0.407605964",
            //   "результат": 0.407605964,
            //   "эквивалентно": "-ln(0.665240956)"
            // }
            // ```
            let negative_log_likelihood_loss: f32 = (maximum_character_score
                - target_character_score)
                + exponentials_from_character_scores_minus_maximum_sum.ln();
            CrossEntropyLossAsNonNegativeF32::try_from(negative_log_likelihood_loss)?
        };
        Ok(NextCharacterProbabilitiesAndNegativeLogLikelihood {
            negative_log_likelihood_loss,
            next_character_probabilities,
        })
    };
    // Параметры MLP: таблица эмбеддингов и два полносвязных слоя, между которыми применяется tanh.
    // Прямой проход и loss — замыкания ниже; размер словаря хранится в локальной переменной vocabulary_len.
    #[derive(Clone)]
    // Полная маленькая модель для последующих примеров; в рабочем коде размеры больше.
    // ```json
    // {
    //   "размеры": {
    //     "CONTEXT_LENGTH": 3,
    //     "EMBEDDING_DIMENSIONS": 2,
    //     "FIRST_LAYER_SIZE": 2,
    //     "vocabulary_len": 3
    //   },
    //   "Vocabulary": [
    //     ".",
    //     "a",
    //     "n"
    //   ],
    //   "character_embeddings": [
    //     [0.0, 0.0],
    //     [0.2, -0.1],
    //     [-0.3, 0.4]
    //   ],
    //   "first_layer_weights": [
    //     [0.5, -0.2],
    //     [0.1, 0.3],
    //     [-0.4, 0.2],
    //     [0.2, -0.1],
    //     [0.3, 0.4],
    //     [-0.2, 0.5]
    //   ],
    //   "first_layer_biases": [0.1, -0.2],
    //   "second_layer_weights": [
    //     [0.1, -0.2],
    //     [0.3, 0.4],
    //     [-0.5, 0.2]
    //   ],
    //   "second_layer_biases": [0.0, 0.1, -0.1],
    //   "как_читать_W1": "first_layer_weights[координата_входа][компонента_первого_слоя]",
    //   "как_читать_W2": "second_layer_weights[индекс_символа][компонента_первого_слоя]"
    // }
    // ```
    struct Model {
        // C: одна строка из EMBEDDING_DIMENSIONS координат на каждый символ Vocabulary.
        character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — веса первого слоя.
        first_layer_weights: [[FiniteF32; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
        // b1: одно обучаемое смещение на каждую компоненту первого слоя.
        first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE],
        // W2: [индекс символа][индекс компоненты первого слоя]; одна строка на символ Vocabulary.
        second_layer_weights: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
        // b2: (vocabulary_len,) — смещения оценок следующих символов.
        second_layer_biases: NonEmptyVec<FiniteF32>,
    }
    // Пример значения и его получения.
    // ```json
    // {
    //   "структура": "Model",
    //   "обучаемые_поля": [
    //     "character_embeddings",
    //     "first_layer_weights",
    //     "first_layer_biases",
    //     "second_layer_weights",
    //     "second_layer_biases"
    //   ],
    //   "размеры_рабочей_модели": {
    //     "C": [27, 10],
    //     "W1": [30, 200],
    //     "b1": [200],
    //     "W2_в_памяти": [27, 200],
    //     "b2": [27]
    //   },
    //   "число_скалярных_параметров": 11897
    // }
    // ```
    let model: Model = {
        // Возвращает следующее начальное значение параметра из N(0, 1).
        // Обычная инициализация даёт значения N(0, 1); seed определяет воспроизводимую последовательность.
        // ```json
        // {
        //   "пример_формы_значений": [-0.6, 0.2, 1.3, -1.1],
        //   "важно": "это иллюстративные числа, не фактические первые значения seed 42",
        //   "использование": [
        //     "эмбеддинги C по умолчанию",
        //     "веса и смещения обоих слоёв"
        //   ],
        //   "FiniteF32": "готовые значения проверяются перед сохранением в Model"
        // }
        // ```
        let mut initialize_embeddings_sample_standard_normal = || -> f32 {
            rand_distr::Distribution::sample(&rand_distr::StandardNormal, &mut random_generator)
        };
        // Для 27 символов создаёт решётку 3 × 3 × 3 в первых трёх координатах.
        // Остальные координаты сначала нулевые, затем обучаются вместе со всеми параметрами.
        // Альтернативная инициализация применяется только к C и сейчас отключена через if false.
        // ```json
        // {
        //   "при_27_символах_и_10_координатах": {
        //     "число_осей_решётки": 3,
        //     "уровни_на_оси": [-1.7320508075688772, 0.0, 1.7320508075688772],
        //     "пример_первого_эмбеддинга": [-1.7320508075688772, -1.7320508075688772, -1.7320508075688772, 0, 0, 0, 0, 0, 0, 0],
        //     "число_узлов": 27
        //   },
        //   "остальные_параметры": "остаются случайными N(0, 1)",
        //   "после_начала_обучения": "все координаты C могут изменяться, включая изначально нулевые"
        // }
        // ```
        let initialize_embeddings_regular_cube = || -> NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> {
            // Пример выбора ветки.
            // ```json
            // {
            //   "vocabulary_len": 1,
            //   "Vocabulary": [
            //     "."
            //   ],
            //   "результат_при_2_координатах": [
            //     [0, 0]
            //   ],
            //   "зачем": "для одного символа не делим на vocabulary_len - 1"
            // }
            // ```
            if vocabulary_len < 2 {
                return vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                    [ZERO_F32; EMBEDDING_DIMENSIONS]
                });
            }
            // Пример значения и его получения.
            // ```json
            // {
            //   "EMBEDDING_DIMENSIONS": 10,
            //   "формула": "min(10, 3)",
            //   "результат": 3,
            //   "если_EMBEDDING_DIMENSIONS_2": 2
            // }
            // ```
            let grid_dimensions: usize = EMBEDDING_DIMENSIONS.min(3);
            // Пример значения и его получения.
            // ```json
            // {
            //   "начало": 1,
            //   "при_27_символах_и_3_осях": {
            //     "проверки": [
            //       "1^3 < 27",
            //       "2^3 < 27",
            //       "3^3 >= 27"
            //     ],
            //     "итог": 3
            //   }
            // }
            // ```
            let mut levels_per_axis: usize = 1;
            // Пример условия продолжения цикла.
            // ```json
            // {
            //   "grid_dimensions": 3,
            //   "vocabulary_len": 27,
            //   "levels_per_axis_последовательно": [1, 2, 3],
            //   "число_узлов": [1, 8, 27],
            //   "остановка": "27 узлов достаточно для 27 символов"
            // }
            // ```
            while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_len {
                // Пример изменения значения.
                // ```json
                // {
                //   "до": 2,
                //   "после": 3,
                //   "число_узлов_до": 8,
                //   "число_узлов_после": 27
                // }
                // ```
                levels_per_axis += 1;
            }
            // Пример значения и его получения.
            // ```json
            // {
            //   "levels_per_axis": 3,
            //   "grid_dimensions": 3,
            //   "формула": "3^3",
            //   "результат": 27
            // }
            // ```
            let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
            // Пример значения и его получения.
            // ```json
            // {
            //   "формула": "sqrt(3)",
            //   "результат": 1.7320508075688772,
            //   "координаты_3_уровней": [-1.7320508075688772, 0, 1.7320508075688772]
            // }
            // ```
            let half_width: f32 = 3.0f32.sqrt();
            // Пример значения и его получения.
            // ```json
            // {
            //   "начало": 0,
            //   "последовательность": [0, 1, 2],
            //   "что_считает": "номер символа Vocabulary, которому создаём эмбеддинг"
            // }
            // ```
            let mut symbol_index: usize = 0;
            vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                // Если число символов не заполняет решётку, выбираем узлы равномерно
                // по её линейной нумерации, включая оба крайних узла; случайности нет.
                // Пример значения и его получения.
                // ```json
                // {
                //   "symbol_index": 13,
                //   "node_count": 27,
                //   "vocabulary_len": 27,
                //   "формула": "13 * (27 - 1) / (27 - 1)",
                //   "результат": 13,
                //   "следующий_этап": "цифры 13 в системе с основанием levels_per_axis задают координаты"
                // }
                // ```
                let mut node_index: usize = symbol_index * (node_count - 1) / (vocabulary_len - 1);
                // Пример значения и его получения.
                // ```json
                // {
                //   "EMBEDDING_DIMENSIONS": 5,
                //   "начало": [0, 0, 0, 0, 0],
                //   "после_заполнения_центрального_узла": [0, 0, 0, 0, 0],
                //   "после_крайнего_узла": [-1.7320508075688772, -1.7320508075688772, -1.7320508075688772, 0, 0]
                // }
                // ```
                let mut embedding: [f32; EMBEDDING_DIMENSIONS] = [ZERO_F32; EMBEDDING_DIMENSIONS];
                // Пример итераций этого цикла.
                // ```json
                // {
                //   "EMBEDDING_DIMENSIONS": 5,
                //   "grid_dimensions": 3,
                //   "изменяемые_координаты": [0, 1, 2],
                //   "оставляем_нулевыми": [3, 4]
                // }
                // ```
                for iteration_value in &mut embedding[..grid_dimensions] {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "node_index": 13,
                    //   "levels_per_axis": 3,
                    //   "первый_level": 1,
                    //   "новое_значение": 0,
                    //   "значение_после_деления_node_index": 4
                    // }
                    // ```
                    let coordinate: &mut f32 = iteration_value;
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "node_index": 13,
                    //   "levels_per_axis": 3,
                    //   "формула": "13 % 3",
                    //   "результат": 1,
                    //   "координата": "-sqrt(3) + 2 * sqrt(3) * 1 / 2 = 0"
                    // }
                    // ```
                    let level: usize = node_index % levels_per_axis;
                    node_index /= levels_per_axis;
                    // Пример изменения значения.
                    // ```json
                    // {
                    //   "half_width": 1.7320508075688772,
                    //   "level": 1,
                    //   "levels_per_axis": 3,
                    //   "формула": "-sqrt(3) + 2 * sqrt(3) * 1 / 2",
                    //   "координата": 0
                    // }
                    // ```
                    *coordinate = -half_width
                        + 2.0 * half_width * level as f32 / (levels_per_axis - 1) as f32;
                }
                // Пример изменения значения.
                // ```json
                // {
                //   "до": 0,
                //   "после": 1,
                //   "следующее_действие": "строим эмбеддинг следующего символа Vocabulary"
                // }
                // ```
                symbol_index += 1;
                embedding
            })
        };
        // Создаём нормальную C в обоих режимах, чтобы остальные веса получали те же случайные числа.
        // C: (vocabulary_len, EMBEDDING_DIMENSIONS) — обучаемые координаты символов.
        // Пример значения и его получения.
        // ```json
        // {
        //   "учебные_размеры": [3, 2],
        //   "пример": [
        //     [-0.6, 0.2],
        //     [1.3, -1.1],
        //     [0.4, 0.7]
        //   ],
        //   "что_создаётся": "отдельный случайный вектор для каждого символа",
        //   "порядок": "генерируется даже при выборе куба, чтобы сохранить RNG для остальных параметров"
        // }
        // ```
        let normal_character_embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> =
            vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                std::array::from_fn(|_: usize| -> f32 {
                    initialize_embeddings_sample_standard_normal()
                })
            });
        // Пример значения и его получения.
        // ```json
        // {
        //   "до_проверки": [
        //     [0, 0],
        //     [0.2, -0.1],
        //     [-0.3, 0.4]
        //   ],
        //   "после_проверки": [
        //     [0, 0],
        //     [0.2, -0.1],
        //     [-0.3, 0.4]
        //   ],
        //   "изменение": "числа те же; теперь каждый элемент имеет тип FiniteF32"
        // }
        // ```
        let character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> = {
            // Пример выбора ветки.
            // ```json
            // {
            //   "if_false": "ветка куба не выбирается",
            //   "выбранное_значение": "normal_character_embeddings",
            //   "далее": "каждая строка проверяется через try_map_array и FiniteF32::try_from"
            // }
            // ```
            let embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> = if false {
                initialize_embeddings_regular_cube()
            } else {
                normal_character_embeddings
            };
            embeddings.try_map(|row: &[f32; EMBEDDING_DIMENSIONS]| -> Result<[FiniteF32; EMBEDDING_DIMENSIONS], &'static str> { try_map_array(*row, FiniteF32::try_from) })?
        };
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — массив фиксированной длины.
        // Пример значения и его получения.
        // ```json
        // {
        //   "учебные_размеры": [6, 2],
        //   "одна_строка": [0.5, -0.2],
        //   "W1_0_1": -0.2,
        //   "смысл": "вес от входной координаты 0 к компоненте первого слоя 1",
        //   "проверка": "каждое число FiniteF32"
        // }
        // ```
        let first_layer_weights: [[FiniteF32; FIRST_LAYER_SIZE];
            CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = try_map_array(
            std::array::from_fn(|_: usize| -> [f32; FIRST_LAYER_SIZE] {
                std::array::from_fn(|_: usize| -> f32 {
                    initialize_embeddings_sample_standard_normal()
                })
            }),
            |row: [f32; FIRST_LAYER_SIZE]| -> Result<[FiniteF32; FIRST_LAYER_SIZE], &'static str> {
                try_map_array(row, FiniteF32::try_from)
            },
        )?;
        // b1: FIRST_LAYER_SIZE случайных смещений первого слоя, проверенных на конечность.
        // Пример значения и его получения.
        // ```json
        // {
        //   "учебный_размер": 2,
        //   "пример": [0.1, -0.2],
        //   "что_означает": "по одному смещению на компоненту первого слоя",
        //   "проверка": "длина массива фиксирована; числа конечные"
        // }
        // ```
        let first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE] = try_map_array(
            std::array::from_fn(|_: usize| -> f32 {
                initialize_embeddings_sample_standard_normal()
            }),
            FiniteF32::try_from,
        )?;
        // W2: для каждого символа — FIRST_LAYER_SIZE связей с первым слоем.
        // Пример значения и его получения.
        // ```json
        // {
        //   "учебные_размеры": [3, 2],
        //   "начало": [
        //     [0, 0],
        //     [0, 0],
        //     [0, 0]
        //   ],
        //   "пример_готовых_весов": [
        //     [0.1, -0.2],
        //     [0.3, 0.4],
        //     [-0.5, 0.2]
        //   ],
        //   "смысл": "строка для каждого символа; столбец для компоненты первого слоя"
        // }
        // ```
        let mut second_layer_weights: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]> = {
            // Пример значения и его получения.
            // ```json
            // {
            //   "вход": 0,
            //   "проверка": "FiniteF32::try_from",
            //   "выход": 0,
            //   "использование": "временное заполнение W2 перед случайной инициализацией"
            // }
            // ```
            let zero_weight: FiniteF32 = FiniteF32::try_from(ZERO_F32)?;
            vocabulary
                .0
                .map(|_: &char| -> [FiniteF32; FIRST_LAYER_SIZE] {
                    [zero_weight; FIRST_LAYER_SIZE]
                })
        };
        // Сначала перебираем компоненты первого слоя, затем символы: порядок случайных чисел сохраняется.
        // Пример итераций этого цикла.
        // ```json
        // {
        //   "учебный_FIRST_LAYER_SIZE": 2,
        //   "first_layer_index": [0, 1],
        //   "смысл": "каждую компоненту первого слоя обрабатываем отдельно",
        //   "внутри_обратного_прохода": "суммируем вклады связанных символов или координат"
        // }
        // ```
        for first_layer_index in 0..FIRST_LAYER_SIZE {
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "first_layer_index": 0,
            //   "строки_символов": [
            //     ".",
            //     "a",
            //     "n"
            //   ],
            //   "последовательно_заполняем": [
            //     "W2[0][0]",
            //     "W2[1][0]",
            //     "W2[2][0]"
            //   ],
            //   "новый_случайный_вес": "отдельный для каждого элемента"
            // }
            // ```
            for iteration_value in second_layer_weights.0.iter_mut() {
                // Пример значения и его получения.
                // ```json
                // {
                //   "символ": "a",
                //   "строка_W2_до": [0, 0],
                //   "first_layer_index": 0,
                //   "случайный_вес": 0.3,
                //   "строка_после": [0.3, 0]
                // }
                // ```
                let character_weights: &mut [FiniteF32; FIRST_LAYER_SIZE] = iteration_value;
                // Пример изменения значения.
                // ```json
                // {
                //   "символ": "a",
                //   "строка_W2_до": [0, 0],
                //   "first_layer_index": 0,
                //   "случайный_вес": 0.3,
                //   "строка_после": [0.3, 0]
                // }
                // ```
                character_weights[first_layer_index] =
                    FiniteF32::try_from(initialize_embeddings_sample_standard_normal())?;
            }
        }
        // b2: (vocabulary_len,) — смещения оценок символов, размер зависит от словаря.
        // Пример значения и его получения.
        // ```json
        // {
        //   "учебный_Vocabulary": [
        //     ".",
        //     "a",
        //     "n"
        //   ],
        //   "пример": [0, 0.1, -0.1],
        //   "что_означает": "одно смещение итоговой оценки на каждый символ"
        // }
        // ```
        let second_layer_biases: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
            (0..vocabulary_len)
                .map(|_: usize| -> Result<FiniteF32, &'static str> {
                    FiniteF32::try_from(initialize_embeddings_sample_standard_normal())
                })
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        Model {
            character_embeddings,
            first_layer_weights,
            first_layer_biases,
            second_layer_weights,
            second_layer_biases,
        }
    };
    // Результат прямого прохода; промежуточные значения нужны для расчёта градиентов.
    struct ForwardPassResult {
        // Эмбеддинги символов контекста, объединённые в один входной вектор.
        character_embeddings_flattened: [FiniteF32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
        // FIRST_LAYER_SIZE результатов tanh; каждый находится в диапазоне от −1 до 1.
        first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE],
        // Оценки следующих символов до softmax, в порядке Vocabulary.
        character_scores: NonEmptyVec<FiniteF32>,
    }
    // Вычисляет оценки следующего символа и сохраняет промежуточные значения прямого прохода.
    // Пример значения и его получения.
    // ```json
    // {
    //   "контекст": "ann",
    //   "предыдущие_индексы": [1, 2, 2],
    //   "возвращает": [
    //     "объединённые координаты C",
    //     "активации первого слоя",
    //     "оценки символов до softmax"
    //   ],
    //   "изменение_параметров": false
    // }
    // ```
    let forward_pass = |model: &Model,
                        previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                            CONTEXT_LENGTH]|
     -> Result<ForwardPassResult, &'static str> {
        // Lookup и flatten не выполняют умножение: выбирают строки C и соединяют их.
        // ```json
        // {
        //   "previous_vocabulary_character_usize_indexes": [1, 2, 2],
        //   "контекст": "ann",
        //   "context_embeddings": [
        //     [0.2, -0.1],
        //     [-0.3, 0.4],
        //     [-0.3, 0.4]
        //   ],
        //   "character_embeddings_flattened": [0.2, -0.1, -0.3, 0.4, -0.3, 0.4],
        //   "пояснение": "строка n встречается дважды, поэтому её координаты скопированы дважды"
        // }
        // ```
        // Берём векторы символов контекста из C и объединяем в исходном порядке.
        // Длина входного вектора: CONTEXT_LENGTH * EMBEDDING_DIMENSIONS.
        let character_embeddings_flattened: [FiniteF32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = {
            // Пример значения и его получения.
            // ```json
            // {
            //   "индексы": [1, 2, 2],
            //   "C": [
            //     [0, 0],
            //     [0.2, -0.1],
            //     [-0.3, 0.4]
            //   ],
            //   "выбранные_строки": [
            //     [0.2, -0.1],
            //     [-0.3, 0.4],
            //     [-0.3, 0.4]
            //   ]
            // }
            // ```
            let context_embeddings: [[FiniteF32; EMBEDDING_DIMENSIONS]; CONTEXT_LENGTH] =
                previous_vocabulary_character_usize_indexes.map(
                    |index: VocabularyCharacterIndexAsUsize| -> [FiniteF32; EMBEDDING_DIMENSIONS] {
                        model.character_embeddings[index.0]
                    },
                );
            context_embeddings.as_flattened().try_into().map_err(
                |_: std::array::TryFromSliceError| -> &'static str {
                    "неверная длина объединённых эмбеддингов"
                },
            )?
        };
        // Первый слой малого примера: в каждой компоненте накапливается собственная сумма.
        // ```json
        // {
        //   "вход": [0.2, -0.1, -0.3, 0.4, -0.3, 0.4],
        //   "начальные_суммы_из_b1": [0.1, -0.2],
        //   "после_координаты_0": [0.2, -0.24],
        //   "после_всех_6_координат_до_tanh": [0.22, -0.29],
        //   "first_layer_activations": [0.216518061, -0.282134813],
        //   "формула_компоненты_0": "tanh(b1[0] + сумма_i(input[i] * W1[i][0]))"
        // }
        // ```
        // Первый слой: начинаем с b1, накапливаем вход × W1, затем применяем tanh.
        let first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE] = {
            // Пример значения и его получения.
            // ```json
            // {
            //   "самостоятельный_пример": true,
            //   "b1": [0.1, -0.2],
            //   "вход": [0.2, -0.1],
            //   "W1": [
            //     [0.5, -0.2],
            //     [0.1, 0.3]
            //   ],
            //   "суммы_до_tanh": [0.19, -0.27],
            //   "после_tanh": [0.18774620586828542, -0.2636248354722033],
            //   "важно": "локальный массив сначала содержит смещения, затем суммы, затем результаты tanh"
            // }
            // ```
            let mut first_layer_activations: [f32; FIRST_LAYER_SIZE] = model
                .first_layer_biases
                .map(|value: FiniteF32| -> f32 { value.0 });
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "вход": [0.2, -0.1, -0.3, 0.4, -0.3, 0.4],
            //   "первые_пары_enumerate": [
            //     [0, 0.2],
            //     [1, -0.1]
            //   ],
            //   "за_итерацию": "одна координата влияет на все компоненты первого слоя"
            // }
            // ```
            for iteration_value in character_embeddings_flattened.iter().enumerate() {
                // Пример значения и его получения.
                // ```json
                // {
                //   "iteration_value": [0, 0.2],
                //   "context_embedding_coordinate_index": 0,
                //   "context_embedding_coordinate": 0.2,
                //   "тип_второго": "FiniteF32 извлекается из ссылки"
                // }
                // ```
                let (context_embedding_coordinate_index, &context_embedding_coordinate): (
                    usize,
                    &FiniteF32,
                ) = iteration_value;
                // Пример итераций этого цикла.
                // ```json
                // {
                //   "одна_координата_входа": 0.2,
                //   "суммы_первого_слоя_до": [0.1, -0.2],
                //   "строка_W1": [0.5, -0.2],
                //   "новые_суммы": [0.2, -0.24]
                // }
                // ```
                for iteration_value in first_layer_activations
                    .iter_mut()
                    .zip(&model.first_layer_weights[context_embedding_coordinate_index])
                {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "first_layer_activation_до": 0.1,
                    //   "first_layer_weight": 0.5,
                    //   "координата_входа": 0.2,
                    //   "после_прибавления": 0.2,
                    //   "ссылка": "изменяется элемент массива first_layer_activations"
                    // }
                    // ```
                    let (first_layer_activation, &first_layer_weight): (&mut f32, &FiniteF32) =
                        iteration_value;
                    // Пример изменения значения.
                    // ```json
                    // {
                    //   "накоплено": 0.1,
                    //   "context_embedding_coordinate": 0.2,
                    //   "first_layer_weight": 0.5,
                    //   "вклад": 0.1,
                    //   "новое_значение": 0.2
                    // }
                    // ```
                    *first_layer_activation +=
                        context_embedding_coordinate.0 * first_layer_weight.0;
                }
            }
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "до": [0.2, -0.24],
            //   "после": [0.197375320224904, -0.23549574953849794],
            //   "операция": "tanh для каждой компоненты"
            // }
            // ```
            for iteration_value in &mut first_layer_activations {
                // Пример значения и его получения.
                // ```json
                // {
                //   "до_накопления": 0.1,
                //   "координата_входа": 0.2,
                //   "вес": 0.5,
                //   "после_накопления": 0.2,
                //   "после_tanh": 0.197375320224904
                // }
                // ```
                let first_layer_activation: &mut f32 = iteration_value;
                // Пример изменения значения.
                // ```json
                // {
                //   "до_tanh": 0.2,
                //   "после_tanh": 0.197375320224904,
                //   "проверка": "конечность суммы проверяется перед tanh"
                // }
                // ```
                *first_layer_activation = FiniteF32::try_from(*first_layer_activation)?.0.tanh();
            }
            try_map_array(first_layer_activations, FiniteF32::try_from)?
        };
        // Второй слой выдаёт три оценки, а не вероятности; веса хранятся отдельной строкой для каждого символа.
        // ```json
        // {
        //   "first_layer_activations": [0.216518061, -0.282134813],
        //   "second_layer_weights": [
        //     [0.1, -0.2],
        //     [0.3, 0.4],
        //     [-0.5, 0.2]
        //   ],
        //   "second_layer_biases": [0.0, 0.1, -0.1],
        //   "character_scores": [0.078078769, 0.052101493, -0.264685993],
        //   "оценка_a": {
        //     "bias": 0.1,
        //     "вклад_компоненты_0": 0.064955418,
        //     "вклад_компоненты_1": -0.112853925,
        //     "сумма": 0.052101493
        //   }
        // }
        // ```
        // Второй слой: b2 + сумма(активация первого слоя × W2); здесь ещё нет softmax.
        let character_scores: NonEmptyVec<f32> = {
            // Пример значения и его получения.
            // ```json
            // {
            //   "самостоятельный_пример": true,
            //   "активации": [0.2, -0.1],
            //   "W2": [
            //     [0.1, -0.2],
            //     [0.3, 0.4],
            //     [-0.5, 0.2]
            //   ],
            //   "b2": [0, 0.1, -0.1],
            //   "результат": [0.04, 0.12, -0.22],
            //   "важно": "оценки могут быть отрицательными; это не вероятности"
            // }
            // ```
            let mut character_scores: NonEmptyVec<f32> = model
                .second_layer_biases
                .map(|value: &FiniteF32| -> f32 { value.0 });
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "активации": [0.2, -0.1],
            //   "пары": [
            //     [0, 0.2],
            //     [1, -0.1]
            //   ],
            //   "за_итерацию": "добавляем вклад одной компоненты первого слоя во все символы"
            // }
            // ```
            for iteration_value in first_layer_activations.iter().enumerate() {
                // Пример значения и его получения.
                // ```json
                // {
                //   "first_layer_index": 0,
                //   "first_layer_activation": 0.2,
                //   "назначение": "передать вклад этой компоненты во все оценки второго слоя"
                // }
                // ```
                let (first_layer_index, &first_layer_activation): (usize, &FiniteF32) =
                    iteration_value;
                // Пример итераций этого цикла.
                // ```json
                // {
                //   "first_layer_index": 0,
                //   "активация": 0.2,
                //   "оценки_до": [0, 0.1, -0.1],
                //   "веса_для_всех_символов": [0.1, 0.3, -0.5],
                //   "оценки_после": [0.02, 0.16, -0.2]
                // }
                // ```
                for iteration_value in character_scores
                    .0
                    .iter_mut()
                    .zip(model.second_layer_weights.iter())
                {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "character_score_до": 0.1,
                    //   "character_weights": [0.3, 0.4],
                    //   "first_layer_index": 0,
                    //   "активация": 0.2,
                    //   "character_score_после": 0.16
                    // }
                    // ```
                    let (character_score, character_weights): (
                        &mut f32,
                        &[FiniteF32; FIRST_LAYER_SIZE],
                    ) = iteration_value;
                    // Пример изменения значения.
                    // ```json
                    // {
                    //   "character_score_до": 0.1,
                    //   "first_layer_activation": 0.2,
                    //   "вес_W2": 0.3,
                    //   "вклад": 0.06,
                    //   "character_score_после": 0.16
                    // }
                    // ```
                    *character_score +=
                        first_layer_activation.0 * character_weights[first_layer_index].0;
                }
            }
            character_scores
        };
        // Проверяем готовые оценки один раз перед передачей в cross_entropy.
        // Пример значения и его получения.
        // ```json
        // {
        //   "самостоятельный_пример": true,
        //   "активации": [0.2, -0.1],
        //   "W2": [
        //     [0.1, -0.2],
        //     [0.3, 0.4],
        //     [-0.5, 0.2]
        //   ],
        //   "b2": [0, 0.1, -0.1],
        //   "результат": [0.04, 0.12, -0.22],
        //   "важно": "оценки могут быть отрицательными; это не вероятности"
        // }
        // ```
        let character_scores: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
            character_scores
                .into_iter()
                .map(FiniteF32::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        Ok(ForwardPassResult {
            character_embeddings_flattened,
            first_layer_activations,
            character_scores,
        })
    };
    // Усредняет cross-entropy по непустому набору; захватывает forward_pass и cross_entropy, модель не изменяет.
    // Оценка набора усредняет NLL; веса при этом не обновляются.
    // ```json
    // {
    //   "NLL_примеров": [0.2, 0.8, 0.5],
    //   "total_negative_log_likelihood_loss": 1.5,
    //   "число_примеров": 3,
    //   "средний_loss": 0.5,
    //   "накопление": "сумма ведётся в f64, результат преобразуется в проверенный f32",
    //   "train_dev_test": "считаются отдельно для каждого набора"
    // }
    // ```
    let loss = move |model: &Model,
                     character_index_pairs: NonEmptySlice<
        '_,
        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
    >|
          -> Result<CrossEntropyLossAsNonNegativeF32, &'static str> {
        // Пример значения и его получения.
        // ```json
        // {
        //   "начальное_значение": 0,
        //   "следующие_NLL": [0.2, 0.8, 0.5],
        //   "значения_накопителя": [0.2, 1, 1.5],
        //   "усреднение": "только после обработки всех примеров"
        // }
        // ```
        let mut total_negative_log_likelihood_loss: f64 = 0.0;
        // Пример итераций этого цикла.
        // ```json
        // {
        //   "NLL_по_итерациям": [0.2, 0.8, 0.5],
        //   "сумма_по_итерациям": [0.2, 1, 1.5],
        //   "веса": "не обновляются, это оценка набора"
        // }
        // ```
        for iteration_value in character_index_pairs.iter() {
            // Пример значения и его получения.
            // ```json
            // {
            //   "previous_vocabulary_character_usize_indexes": [1, 2, 2],
            //   "next_vocabulary_character_usize_index": 1,
            //   "читается": "ann → a",
            //   "операция": "берём ссылку на один пример без копирования параметров модели"
            // }
            // ```
            let character_index_pair: &PreviousVocabularyCharacterIndexesAndNextCharacterIndex =
                iteration_value;
            // Пример значения и его получения.
            // ```json
            // {
            //   "самостоятельный_пример": true,
            //   "character_scores": [0, 2, 1],
            //   "цель": 1,
            //   "результат": {
            //     "next_character_probabilities": [0.090030573, 0.665240956, 0.244728471],
            //     "negative_log_likelihood_loss": 0.407605964
            //   }
            // }
            // ```
            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                NonEmptySlice::from(
                    &forward_pass(
                        model,
                        character_index_pair.previous_vocabulary_character_usize_indexes,
                    )?
                    .character_scores,
                ),
                character_index_pair.next_vocabulary_character_usize_index,
            )?;
            // Пример изменения значения.
            // ```json
            // {
            //   "до": 0.4,
            //   "NLL_текущего_примера": 0.7,
            //   "после": 1.1,
            //   "тип_накопителя": "f64 для оценки полного набора",
            //   "усреднение": "после цикла делим на число примеров",
            //   "проверка": "среднее преобразуется в f32 и проверяется через CrossEntropyLossAsNonNegativeF32::try_from"
            // }
        // ```
            total_negative_log_likelihood_loss += f64::from(
                next_character_probabilities_and_negative_log_likelihood
                    .negative_log_likelihood_loss
                    .0,
            );
        }
        CrossEntropyLossAsNonNegativeF32::try_from(
            (total_negative_log_likelihood_loss / character_index_pairs.len() as f64) as f32,
        )
    };
    // Пример значения и его получения.
    // ```json
    // {
    //   "C": 270,
    //   "W1": 6000,
    //   "b1": 200,
    //   "W2": 5400,
    //   "b2": 27,
    //   "итого": 11897,
    //   "важно": "длину матрицы в строках умножаем на длину строки"
    // }
    // ```
    let trainable_scalar_count: usize = model.character_embeddings.len() * EMBEDDING_DIMENSIONS
        + model.first_layer_weights.len() * FIRST_LAYER_SIZE
        + model.first_layer_biases.len()
        + model.second_layer_weights.len() * FIRST_LAYER_SIZE
        + model.second_layer_biases.len();
    println!(
        "C=({},{}), W1=({},{}), b1=({}), W2=({},{}), b2=({}); параметров: {}",
        vocabulary_len,
        EMBEDDING_DIMENSIONS,
        CONTEXT_LENGTH * EMBEDDING_DIMENSIONS,
        FIRST_LAYER_SIZE,
        FIRST_LAYER_SIZE,
        FIRST_LAYER_SIZE,
        vocabulary_len,
        vocabulary_len,
        trainable_scalar_count
    );
    // Пример значения и его получения.
    // ```json
    // {
    //   "момент": "до первого обновления модели",
    //   "набор": "validation_character_index_pairs",
    //   "пример_лога": "До обучения dev loss=26.192036",
    //   "число": "иллюстрация; фактическое зависит от инициализации"
    // }
    // ```
    let initial_validation_loss: CrossEntropyLossAsNonNegativeF32 = loss(
        &model,
        NonEmptySlice::from(&validation_character_index_pairs),
    )?;
    println!("До обучения dev loss={:.6}", initial_validation_loss.0);
    // Батчи идут последовательно; внутри обучения нет случайной выборки или нового перемешивания.
    // ```json
    // {
    //   "учебные_настройки": {
    //     "число_примеров": 7,
    //     "BATCH_SIZE": 3
    //   },
    //   "батчи": [
    //     {
    //       "шаг": 0,
    //       "batch_start": 0,
    //       "batch_end": 3,
    //       "позиции": [0, 1, 2]
    //     },
    //     {
    //       "шаг": 1,
    //       "batch_start": 3,
    //       "batch_end": 6,
    //       "позиции": [3, 4, 5]
    //     },
    //     {
    //       "шаг": 2,
    //       "batch_start": 6,
    //       "batch_end": 7,
    //       "позиции": [6]
    //     },
    //     {
    //       "шаг": 3,
    //       "batch_start": 0,
    //       "batch_end": 3,
    //       "позиции": [0, 1, 2]
    //     }
    //   ],
    //   "порядок": "после конца снова тот же порядок; первоначальное разделение имён уже было перемешано",
    //   "последний_батч": "делим loss и градиенты на 1, а не на 3"
    // }
    // ```
    // 3. Последовательно берём непересекающиеся батчи train; после конца начинаем новый проход в том же порядке.
    let model: Model = {
        // Пример значения и его получения.
        // ```json
        // {
        //   "структура": "Model",
        //   "обучаемые_поля": [
        //     "character_embeddings",
        //     "first_layer_weights",
        //     "first_layer_biases",
        //     "second_layer_weights",
        //     "second_layer_biases"
        //   ],
        //   "размеры_рабочей_модели": {
        //     "C": [27, 10],
        //     "W1": [30, 200],
        //     "b1": [200],
        //     "W2_в_памяти": [27, 200],
        //     "b2": [27]
        //   },
        //   "число_скалярных_параметров": 11897
        // }
        // ```
        let mut model: Model = model;
        // Пример значения и его получения.
        // ```json
        // {
        //   "начало": 0,
        //   "BATCH_SIZE": 3,
        //   "пример_7_записей": [0, 3, 6, 0],
        //   "что_означает": "позиция первого ещё не обработанного примера"
        // }
        // ```
        let mut batch_start: usize = 0;
        // Пример значения и его получения.
        // ```json
        // {
        //   "что_измеряется": "всё обучение",
        //   "момент_записи": "Instant::now()",
        //   "пример_прошло_секунд": 1.25,
        //   "вывод": "elapsed в секундах"
        // }
        // ```
        let training_started_at: std::time::Instant = std::time::Instant::now();
        // Пример итераций этого цикла.
        // ```json
        // {
        //   "training_steps": 3,
        //   "training_step": [0, 1, 2],
        //   "за_один_шаг": "один батч, средние градиенты, одно обновление всех параметров"
        // }
        // ```
        for training_step in 0..training_steps.get() {
            // Пример значения и его получения.
            // ```json
            // {
            //   "что_измеряется": "один шаг",
            //   "момент_записи": "Instant::now()",
            //   "пример_прошло_секунд": 1.25,
            //   "вывод": "iteration_ms = 1250"
            // }
            // ```
            let iteration_started_at: std::time::Instant = std::time::Instant::now();
            // Пример значения и его получения.
            // ```json
            // {
            //   "значения_на_границе": [
            //     {
            //       "training_step": 99999,
            //       "learning_rate": 0.1
            //     },
            //     {
            //       "training_step": 100000,
            //       "learning_rate": 0.01
            //     }
            //   ],
            //   "тип": "LearningRateAsPositiveFiniteF32",
            //   "действие": "умножает градиент при вычитании из параметра"
            // }
            // ```
            let learning_rate: LearningRateAsPositiveFiniteF32 =
                // Пример выбора ветки.
                // ```json
                // {
                //   "значения_на_границе": [
                //     {
                //       "training_step": 99999,
                //       "learning_rate": 0.1
                //     },
                //     {
                //       "training_step": 100000,
                //       "learning_rate": 0.01
                //     }
                //   ],
                //   "тип": "LearningRateAsPositiveFiniteF32",
                //   "действие": "умножает градиент при вычитании из параметра"
                // }
                // ```
                LearningRateAsPositiveFiniteF32::try_from(if training_step < 100_000 {
                    0.1
                } else {
                    0.01
                })?;
            // Пример значения и его получения.
            // ```json
            // {
            //   "NLL_примеров": [0.2, 0.8, 0.5],
            //   "сумма": 1.5,
            //   "число_примеров": 3,
            //   "средний_loss": 0.5
            // }
            // ```
            let loss: CrossEntropyLossAsNonNegativeF32 = {
                // Формы совпадают с Model; каждый градиент проверяется при изменении.
                // ModelGradients хранит производные, а не параметры; перед каждым батчем все элементы — нули.
                // ```json
                // {
                //   "Model_пример": {
                //     "second_layer_biases": [0.0, 0.1, -0.1]
                //   },
                //   "ModelGradients_до_батча": {
                //     "second_layer_biases_gradients": [0.0, 0.0, 0.0]
                //   },
                //   "после_первого_примера": [0.05, -0.15, 0.1],
                //   "вклад_второго_примера": [0.1, 0.15, -0.25],
                //   "после_второго_примера": [0.15, 0.0, -0.15],
                //   "веса_во_время_накопления": "остаются прежними",
                //   "перед_следующим_батчем": "градиенты заново обнуляются; параметры модели сохраняются"
                // }
                // ```
                struct ModelGradients {
                    // Производные среднего loss по координатам символов C.
                    character_embeddings_gradients: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
                    // Производные по W1: [координата объединённых эмбеддингов][компонента первого слоя].
                    first_layer_weights_gradients:
                        [[FiniteF32; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                    // Производные по каждому смещению первого слоя b1.
                    first_layer_biases_gradients: [FiniteF32; FIRST_LAYER_SIZE],
                    // Производные по W2: [индекс символа Vocabulary][компонента первого слоя].
                    second_layer_weights_gradients: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
                    // Производные по каждому смещению оценок символов b2.
                    second_layer_biases_gradients: NonEmptyVec<FiniteF32>,
                }
                // Пример значения и его получения.
                // ```json
                // {
                //   "сумма_NLL": 1.5,
                //   "len_батча": 3,
                //   "loss": 0.5,
                //   "model_gradients": "накопленные средние производные; параметры модели пока не обновлены"
                // }
                // ```
                let (loss, model_gradients): (f32, ModelGradients) = {
                    // Последний батч эпохи может быть короче BATCH_SIZE: используем все примеры.
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "batch_start": 6,
                    //   "BATCH_SIZE": 3,
                    //   "длина_train": 7,
                    //   "формула": "min(6 + 3, 7)",
                    //   "результат": 7,
                    //   "выбранные_позиции": [6]
                    // }
                    // ```
                    let batch_end: usize =
                        (batch_start + BATCH_SIZE).min(training_character_index_pairs.len());
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "самостоятельный_пример": true,
                    //   "batch_start": 3,
                    //   "batch_end": 5,
                    //   "срез_позиций": [3, 4],
                    //   "копирование_примеров": false,
                    //   "проверка": "NonEmptySlice::try_from отклонит пустой срез",
                    //   "назначение": "оба примера используются для одного шага SGD"
                    // }
                    // ```
                    let previous_vocabulary_character_indexes_and_next_character_index_batch: NonEmptySlice<
                        '_,
                        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                    > = NonEmptySlice::try_from(
                        &training_character_index_pairs[batch_start..batch_end],
                    )?;
                    // Пример выбора ветки.
                    // ```json
                    // {
                    //   "если_batch_end_равен_длине_train": {
                    //     "batch_start": 0,
                    //     "следующее": "новый проход в том же порядке"
                    //   },
                    //   "иначе": {
                    //     "batch_start": "batch_end",
                    //     "следующее": "берём следующий непересекающийся срез"
                    //   }
                    // }
                    // ```
                    batch_start = if batch_end == training_character_index_pairs.len() {
                        0
                    } else {
                        batch_end
                    };
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "вход": 0,
                    //   "после_FiniteF32": 0,
                    //   "применение": "начальное значение всех градиентов перед батчем"
                    // }
                    // ```
                    let zero_gradient: FiniteF32 = FiniteF32::try_from(ZERO_F32)?;
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "до_батча": {
                    //     "second_layer_biases_gradients": [0, 0, 0]
                    //   },
                    //   "после_первого_примера": {
                    //     "second_layer_biases_gradients": [0.05, -0.15, 0.1]
                    //   },
                    //   "после_второго": {
                    //     "second_layer_biases_gradients": [0.15, 0, -0.15]
                    //   },
                    //   "параметры_Model": "здесь не обнуляются и не обновляются"
                    // }
                    // ```
                    let mut model_gradients: ModelGradients = ModelGradients {
                        character_embeddings_gradients: model
                            .character_embeddings
                            .map(|_: &[FiniteF32; EMBEDDING_DIMENSIONS]| -> [FiniteF32; EMBEDDING_DIMENSIONS] { [zero_gradient; EMBEDDING_DIMENSIONS] }),
                        first_layer_weights_gradients: [[zero_gradient; FIRST_LAYER_SIZE];
                            CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                        first_layer_biases_gradients: [zero_gradient; FIRST_LAYER_SIZE],
                        second_layer_weights_gradients: model
                            .second_layer_weights
                            .map(|_: &[FiniteF32; FIRST_LAYER_SIZE]| -> [FiniteF32; FIRST_LAYER_SIZE] { [zero_gradient; FIRST_LAYER_SIZE] }),
                        second_layer_biases_gradients: model
                            .second_layer_biases
                            .map(|_: &FiniteF32| -> FiniteF32 { zero_gradient }),
                    };
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "начальное_значение": 0,
                    //   "следующие_NLL": [0.2, 0.8, 0.5],
                    //   "значения_накопителя": [0.2, 1, 1.5],
                    //   "усреднение": "только после обработки всех примеров"
                    // }
                    // ```
                    let mut total_negative_log_likelihood_loss: FiniteF32 = zero_gradient;
                    // Пример итераций этого цикла.
                    // ```json
                    // {
                    //   "батч": [
                    //     "ann → a",
                    //     "ann → n"
                    //   ],
                    //   "итерация_0": "прибавляем вклад первого примера в те же накопители",
                    //   "итерация_1": "прибавляем вклад второго примера",
                    //   "между_примерами": "градиенты не обнуляются"
                    // }
                    // ```
                    for previous_vocabulary_character_indexes_and_next_character_index_batch_element in
                        previous_vocabulary_character_indexes_and_next_character_index_batch.iter()
                    {
                        // Производная по оценке правильного символа: p − 1, затем деление на размер батча.
                        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
                        struct TargetScoreGradientAsF32FromMinus1To0(f32);
                        impl TryFrom<f32> for TargetScoreGradientAsF32FromMinus1To0 {
                            type Error = &'static str;
                            // Проверяет замкнутый диапазон [−1, 0], отклоняя NaN и бесконечности.
                            // Пример входа и результата этой функции.
                            // ```json
                            // {
                            //   "проверки": [
                            //     {
                            //       "value": -0.15,
                            //       "ветка": "Ok"
                            //     },
                            //     {
                            //       "value": -1,
                            //       "ветка": "Ok"
                            //     },
                            //     {
                            //       "value": 0,
                            //       "ветка": "Ok"
                            //     },
                            //     {
                            //       "value": 0.1,
                            //       "ветка": "Err"
                            //     }
                            //   ]
                            // }
                            // ```
                            fn try_from(value: f32) -> Result<Self, Self::Error> {
                                // Пример выбора ветки.
                                // ```json
                                // {
                                //   "проверки": [
                                //     {
                                //       "value": -0.15,
                                //       "ветка": "Ok"
                                //     },
                                //     {
                                //       "value": -1,
                                //       "ветка": "Ok"
                                //     },
                                //     {
                                //       "value": 0,
                                //       "ветка": "Ok"
                                //     },
                                //     {
                                //       "value": 0.1,
                                //       "ветка": "Err"
                                //     }
                                //   ]
                                // }
                                // ```
                                if (-1.0..=ZERO_F32).contains(&value) {
                                    Ok(Self(value))
                                } else {
                                    Err("градиент правильного символа должен быть от −1 до 0")
                                }
                            }
                        }
                        // Различает градиент правильного символа и неотрицательные градиенты остальных.
                        #[derive(Clone, Copy, Debug)]
                        enum NextCharacterScoreGradientAsF32FromMinus1To1 {
                            Target(TargetScoreGradientAsF32FromMinus1To0),
                            // Используем проверенный диапазон [0, 1]; здесь значение уже является градиентом.
                            Other(CharacterProbabilityAsF32From0To1),
                        }
                        impl From<NextCharacterScoreGradientAsF32FromMinus1To1> for f32 {
                            // Извлекает числовую производную для расчёта градиентов параметров модели.
                            // Пример входа и результата этой функции.
                            // ```json
                            // {
                            //   "вход": {
                            //     "Target": -0.15
                            //   },
                            //   "выход_f32": -0.15,
                            //   "для_Other": {
                            //     "вход": {
                            //       "Other": 0.05
                            //     },
                            //     "выход_f32": 0.05
                            //   }
                            // }
                            // ```
                            fn from(
                                gradient: NextCharacterScoreGradientAsF32FromMinus1To1,
                            ) -> Self {
                                // Пример разбора варианта enum.
                                // ```json
                                // {
                                //   "входы_и_выходы": [
                                //     {
                                //       "вход": {
                                //         "Target": -0.15
                                //       },
                                //       "выход_f32": -0.15
                                //     },
                                //     {
                                //       "вход": {
                                //         "Other": 0.05
                                //       },
                                //       "выход_f32": 0.05
                                //     }
                                //   ],
                                //   "что_меняется": "убирается enum-обёртка; само число не меняется"
                                // }
                                // ```
                                match gradient {
                                    NextCharacterScoreGradientAsF32FromMinus1To1::Target(value) => {
                                        value.0
                                    }
                                    NextCharacterScoreGradientAsF32FromMinus1To1::Other(value) => {
                                        value.0
                                    }
                                }
                            }
                        }
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "какой_пример": "текущий элемент батча",
                        //   "данные": [
                        //     "character_embeddings_flattened",
                        //     "first_layer_activations",
                        //     "character_scores"
                        //   ],
                        //   "для_чего": "используются при обратном проходе; не запускаем повторный forward для каждого веса"
                        // }
                        // ```
                        let forward_pass_result: ForwardPassResult = forward_pass(
                            &model,
                            previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                .previous_vocabulary_character_usize_indexes,
                        )?;
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "вероятности": [0.1, 0.7, 0.2],
                        //   "правильный_индекс": 1,
                        //   "размер_батча": 2,
                        //   "результат": [
                        //     {
                        //       "Other": 0.05
                        //     },
                        //     {
                        //       "Target": -0.15
                        //     },
                        //     {
                        //       "Other": 0.1
                        //     }
                        //   ],
                        //   "сумма_примерно": 0
                        // }
                        // ```
                        let next_character_score_gradients_as_f32_from_minus1_to1: NonEmptyVec<
                            NextCharacterScoreGradientAsF32FromMinus1To1,
                        > = {
                            // Пример значения и его получения.
                            // ```json
                            // {
                            //   "самостоятельный_пример": true,
                            //   "character_scores": [0, 2, 1],
                            //   "цель": 1,
                            //   "результат": {
                            //     "next_character_probabilities": [0.090030573, 0.665240956, 0.244728471],
                            //     "negative_log_likelihood_loss": 0.407605964
                            //   }
                            // }
                            // ```
                            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                                NonEmptySlice::from(&forward_pass_result.character_scores),
                                previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                    .next_vocabulary_character_usize_index,
                            )?;
                            // Пример изменения значения.
                            // ```json
                            // {
                            //   "до": 0.4,
                            //   "NLL_текущего_примера": 0.7,
                            //   "сложение": 1.1,
                            //   "после": 1.1,
                            //   "что_ещё_не_сделано": "не делили на размер батча",
                            //   "при_бесконечном_результате": "Err через FiniteF32; в накопитель бесконечность не записывается"
                            // }
                            // ```
                            total_negative_log_likelihood_loss = FiniteF32::try_from(
                                total_negative_log_likelihood_loss.0
                                    + next_character_probabilities_and_negative_log_likelihood
                                        .negative_log_likelihood_loss
                                        .0,
                            )?;
                            // Пример значения и его получения.
                            // ```json
                            // {
                            //   "фактическая_len": 6,
                            //   "после_as_f32": 6.0,
                            //   "BATCH_SIZE": 32,
                            //   "почему_не_константа": "последний батч может быть короче 32; делить нужно на фактическое число примеров"
                            // }
                            // ```
                            let previous_vocabulary_character_indexes_and_next_character_index_batch_len: f32 =
                                previous_vocabulary_character_indexes_and_next_character_index_batch
                                    .len() as f32;
                            // Градиент по оценке символа — (p − целевая_метка) / фактический_размер_батча.
                        // ```json
                        // {
                        //   "next_character_probabilities": [0.3725557, 0.36300234, 0.26444196],
                        //   "правильный_индекс": 1,
                        //   "целевая_метка_one_hot": [0, 1, 0],
                        //   "размер_батча": 2,
                        //   "до_деления": [0.3725557, -0.63699766, 0.26444196],
                        //   "после_деления_и_варианты_enum": [
                        //     {
                        //       "Other": 0.18627785
                        //     },
                        //     {
                        //       "Target": -0.31849883
                        //     },
                        //     {
                        //       "Other": 0.13222098
                        //     }
                        //   ],
                        //   "диапазоны": {
                        //     "Target": [-1, 0],
                        //     "Other": [0, 1]
                        //   },
                        //   "сумма_градиентов_примерно": 0.0
                        // }
                        // ```
                        // Ровно один Target ставим на позицию правильного символа; остальные — Other.
                            // Проверяем диапазон уже усреднённой производной: (p − 1) / B либо p / B.
                            NonEmptyVec::try_from(
                                next_character_probabilities_and_negative_log_likelihood.next_character_probabilities
                                    .iter().enumerate()
                                    .map(|(next_character_probability_index, next_character_probability): (usize, &CharacterProbabilityAsF32From0To1)| -> Result<NextCharacterScoreGradientAsF32FromMinus1To1, &'static str> {
                                        // Пример выбора ветки.
                                        // ```json
                                        // {
                                        //   "размер_батча": 2,
                                        //   "правильный_индекс": 1,
                                        //   "ветка_Target": {
                                        //     "next_character_probability_index": 1,
                                        //     "next_character_probability": 0.7,
                                        //     "вычитание_1": -0.3,
                                        //     "после_деления": -0.15,
                                        //     "результат": {
                                        //       "Target": -0.15
                                        //     }
                                        //   },
                                        //   "ветка_Other": {
                                        //     "next_character_probability_index": 0,
                                        //     "next_character_probability": 0.1,
                                        //     "после_деления": 0.05,
                                        //     "результат": {
                                        //       "Other": 0.05
                                        //     }
                                        //   },
                                        //   "важно": "Target один, но Other тоже может иметь значение 0; различаем по индексу, не по знаку числа"
                                        // }
                                        // ```
                                        if next_character_probability_index == previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index.0 {
                                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Target(
                                                TargetScoreGradientAsF32FromMinus1To0::try_from((next_character_probability.0 - 1.0) / previous_vocabulary_character_indexes_and_next_character_index_batch_len)?,
                                            ))
                                        } else {
                                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Other(
                                                CharacterProbabilityAsF32From0To1::try_from(next_character_probability.0 / previous_vocabulary_character_indexes_and_next_character_index_batch_len)?,
                                            ))
                                        }
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            )?
                        };
                        // Правило цепочки для output = input * weight + bias; общий множитель output_gradient уже известен.
                        // ```json
                        // {
                        //   "input": 2.0,
                        //   "weight": 3.0,
                        //   "bias": 1.0,
                        //   "output": 7.0,
                        //   "output_gradient": 0.1,
                        //   "градиент_веса": {
                        //     "формула": "input * output_gradient",
                        //     "вклад": 0.2
                        //   },
                        //   "градиент_входа": {
                        //     "формула": "weight * output_gradient",
                        //     "вклад": 0.3
                        //   },
                        //   "градиент_смещения": {
                        //     "формула": "output_gradient",
                        //     "вклад": 0.1
                        //   },
                        //   "accumulate_gradient_product": {
                        //     "gradient_до": 0.4,
                        //     "input_or_weight": 2.0,
                        //     "output_gradient": 0.1,
                        //     "contribution": 0.2,
                        //     "gradient_после": 0.6
                        //   },
                        //   "при_переполнении": "Err до присваивания неконечного результата"
                        // }
                        // ```
                        // Добавляет произведение двух множителей: вход × градиент выхода либо вес × градиент выхода.
                        fn accumulate_gradient_product(
                            gradient: &mut FiniteF32,
                            input_or_weight: f32,
                            output_gradient: f32,
                            location: &'static str,
                        ) -> Result<(), &'static str> {
                            // Пример значения и его получения.
                            // ```json
                            // {
                            //   "input_or_weight": 2,
                            //   "output_gradient": 0.1,
                            //   "произведение": 0.2,
                            //   "проверка": "FiniteF32::try_from отклоняет переполнение ещё до прибавления"
                            // }
                            // ```
                            let contribution: FiniteF32 = FiniteF32::try_from(input_or_weight * output_gradient)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            // Пример изменения значения.
                            // ```json
                            // {
                            //   "gradient_до": 0.4,
                            //   "contribution": 0.2,
                            //   "новая_сумма": 0.6,
                            //   "проверка": "новое значение должно быть конечным; затем записываем в gradient"
                            // }
                            // ```
                            *gradient = FiniteF32::try_from(gradient.0 + contribution.0)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            Ok(())
                        }
                        // Как zip и accumulate_bias_gradient складывают вклады двух примеров одного батча.
                        // ```json
                        // {
                        //   "это_самостоятельный_пример": true,
                        //   "порядок_символов": [
                        //     ".",
                        //     "a",
                        //     "n"
                        //   ],
                        //   "B": 2,
                        //   "начальные_second_layer_biases_gradients": [0, 0, 0],
                        //   "первый_пример": {
                        //     "вероятности": [0.1, 0.7, 0.2],
                        //     "правильный_индекс": 1,
                        //     "градиенты_оценок_после_деления_на_B": [0.05, -0.15, 0.1],
                        //     "накопленные_градиенты_после_цикла": [0.05, -0.15, 0.1]
                        //   },
                        //   "второй_пример": {
                        //     "вероятности": [0.2, 0.3, 0.5],
                        //     "правильный_индекс": 2,
                        //     "градиенты_оценок_после_деления_на_B": [0.1, 0.15, -0.25],
                        //     "накопленные_градиенты_после_цикла": [0.15, 0, -0.15]
                        //   },
                        //   "пара_из_zip_для_a_во_втором_примере": {
                        //     "изменяемый_накопитель": -0.15,
                        //     "добавляемое_значение": 0.15,
                        //     "после_сложения": 0
                        //   },
                        //   "границы_суммы": "в точной арифметике средний градиент b2 лежит в [−1, 1]; FiniteF32 проверяет только конечность",
                        //   "параметры_Model": "не меняются этим циклом; меняются только накопители ModelGradients"
                        // }
                        // ```
                        // Производная по смещению равна производной по выходной оценке: множитель 1 не нужен.
                        fn accumulate_bias_gradient(
                            bias_gradient: &mut FiniteF32,
                            output_gradient: f32,
                            location: &'static str,
                        ) -> Result<(), &'static str> {
                            // Пример изменения значения.
                            // ```json
                            // {
                            //   "bias_gradient_до": -0.15,
                            //   "output_gradient": 0.15,
                            //   "после": 0,
                            //   "параметр_bias_модели": "не изменяется; это производная по нему"
                            // }
                            // ```
                            *bias_gradient = FiniteF32::try_from(bias_gradient.0 + output_gradient)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            Ok(())
                        }
                        // Пример итераций этого цикла.
                        // ```json
                        // {
                        //   "порядок": [
                        //     ".",
                        //     "a",
                        //     "n"
                        //   ],
                        //   "накопители_до": [0, 0, 0],
                        //   "добавляемые_градиенты": [0.05, -0.15, 0.1],
                        //   "накопители_после": [0.05, -0.15, 0.1],
                        //   "zip": "первая пара относится к точке, вторая к a, третья к n"
                        // }
                        // ```
                        for (
                            second_layer_bias_gradient,
                            &next_character_score_gradient_as_f32_from_minus1_to1,
                        ) in model_gradients
                            .second_layer_biases_gradients
                            .0
                            .iter_mut()
                            .zip(next_character_score_gradients_as_f32_from_minus1_to1.iter())
                        {
                            // Пример именно этого вклада в градиент.
                            // ```json
                            // {
                            //   "самостоятельный_пример": true,
                            //   "second_layer_bias_gradient_до": -0.15,
                            //   "next_character_score_gradient_as_f32_from_minus1_to1": {
                            //     "Other": 0.15
                            //   },
                            //   "после_f32_from": 0.15,
                            //   "после_накопления": 0,
                            //   "что_не_меняется": "second_layer_biases в Model пока прежние"
                            // }
                            // ```
                            accumulate_bias_gradient(
                                second_layer_bias_gradient,
                                f32::from(next_character_score_gradient_as_f32_from_minus1_to1),
                                "переполнение градиента second_layer_biases",
                            )?;
                        }
                        // Обратный проход через W2 и tanh для первого примера малого батча.
                        // ```json
                        // {
                        //   "градиенты_оценок": [0.18627785, -0.31849883, 0.13222098],
                        //   "вклады_W2_по_символам_для_компоненты_0": [0.018627785, -0.095549649, -0.06611049],
                        //   "суммы_до_производной_tanh": [-0.143032354, -0.138210906],
                        //   "активации_tanh": [0.216518061, -0.282134813],
                        //   "производная_tanh": [0.953119929, 0.920399947],
                        //   "first_layer_pre_activation_gradients": [-0.136326987, -0.127209311],
                        //   "градиент_веса_W2_a_0": {
                        //     "input": 0.216518061,
                        //     "output_gradient": -0.31849883,
                        //     "вклад": -0.068960749
                        //   }
                        // }
                        // ```
                        // Производные ошибки по суммам первого слоя ДО применения tanh.
                        let first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = {
                            // Пример значения и его получения.
                            // ```json
                            // {
                            //   "самостоятельный_пример": true,
                            //   "компонента_0": {
                            //     "веса_W2": [0.1, 0.3, -0.5],
                            //     "градиенты_оценок": [0.05, -0.15, 0.1],
                            //     "вклады": [0.005, -0.045, -0.05],
                            //     "сумма_до_tanh": -0.09,
                            //     "активация_tanh": 0.2,
                            //     "производная_tanh": 0.96,
                            //     "результат": -0.0864
                            //   },
                            //   "назначение": "градиенты по суммам первого слоя, до tanh"
                            // }
                            // ```
                            let mut first_layer_pre_activation_gradients: [FiniteF32;
                                FIRST_LAYER_SIZE] = [zero_gradient; FIRST_LAYER_SIZE];
                            // Пример итераций этого цикла.
                            // ```json
                            // {
                            //   "учебный_FIRST_LAYER_SIZE": 2,
                            //   "first_layer_index": [0, 1],
                            //   "смысл": "каждую компоненту первого слоя обрабатываем отдельно",
                            //   "внутри_обратного_прохода": "суммируем вклады связанных символов или координат"
                            // }
                            // ```
                            for first_layer_index in 0..FIRST_LAYER_SIZE {
                                // Пример итераций этого цикла.
                                // ```json
                                // {
                                //   "Vocabulary": [
                                //     ".",
                                //     "a",
                                //     "n"
                                //   ],
                                //   "индексы": [0, 1, 2],
                                //   "за_итерацию": "вес W2 и градиент оценки одного и того же символа"
                                // }
                                // ```
                                for iteration_value in 0..vocabulary_len {
                                    // Пример значения и его получения.
                                    // ```json
                                    // {
                                    //   "Vocabulary": [
                                    //     ".",
                                    //     "a",
                                    //     "n"
                                    //   ],
                                    //   "значения_цикла": [0, 1, 2],
                                    //   "текущий_пример": 1,
                                    //   "что_выбирается": "строка W2 и градиент оценки того же символа a"
                                    // }
                                    // ```
                                    let vocabulary_character_index: usize = iteration_value;
                                    // Пример именно этого вклада в градиент.
                                    // ```json
                                    // {
                                    //   "самостоятельный_пример": true,
                                    //   "куда": "second_layer_weights_gradients[a][0]",
                                    //   "градиент_до": 0,
                                    //   "input_or_weight": "активация первого слоя, 0.2",
                                    //   "output_gradient": -0.15,
                                    //   "вклад": -0.03,
                                    //   "градиент_после": -0.03
                                    // }
                                    // ```
                                    accumulate_gradient_product(
                                        &mut model_gradients.second_layer_weights_gradients.0
                                            [vocabulary_character_index][first_layer_index],
                                        forward_pass_result.first_layer_activations
                                            [first_layer_index]
                                            .0,
                                        f32::from(
                                            next_character_score_gradients_as_f32_from_minus1_to1
                                                [vocabulary_character_index],
                                        ),
                                        "переполнение градиента second_layer_weights",
                                    )?;
                                    // Пример именно этого вклада в градиент.
                                    // ```json
                                    // {
                                    //   "самостоятельный_пример": true,
                                    //   "куда": "first_layer_pre_activation_gradients[0], ещё до умножения на производную tanh",
                                    //   "накоплено_до": 0.005,
                                    //   "input_or_weight": "вес W2[a][0] = 0.3",
                                    //   "output_gradient": -0.15,
                                    //   "вклад": -0.045,
                                    //   "после": -0.04,
                                    //   "далее": "добавляем вклады остальных символов, затем применяем производную tanh"
                                    // }
                                    // ```
                                    accumulate_gradient_product(
                                        &mut first_layer_pre_activation_gradients
                                            [first_layer_index],
                                        model.second_layer_weights[vocabulary_character_index]
                                            [first_layer_index]
                                            .0,
                                        f32::from(
                                            next_character_score_gradients_as_f32_from_minus1_to1
                                                [vocabulary_character_index],
                                        ),
                                        "переполнение градиента входа первого слоя",
                                    )?;
                                }
                                // Производная tanh равна 1 - tanh².
                                // Пример изменения значения.
                                // ```json
                                // {
                                //   "до_производной_tanh": -0.09,
                                //   "активация": 0.2,
                                //   "множитель": "1 - 0.2 * 0.2 = 0.96",
                                //   "после": -0.0864
                                // }
                                // ```
                                first_layer_pre_activation_gradients[first_layer_index] =
                                    FiniteF32::try_from(
                                        first_layer_pre_activation_gradients[first_layer_index].0
                                            * (1.0
                                                - forward_pass_result.first_layer_activations
                                                    [first_layer_index]
                                                    .0
                                                    * forward_pass_result.first_layer_activations
                                                        [first_layer_index]
                                                        .0),
                                    )?;
                                // Пример именно этого вклада в градиент.
                                // ```json
                                // {
                                //   "самостоятельный_пример": true,
                                //   "куда": "first_layer_biases_gradients[0]",
                                //   "накоплено_до": 0,
                                //   "output_gradient": -0.0864,
                                //   "после": -0.0864,
                                //   "источник": "производная по сумме первого слоя после обратного прохода через tanh"
                                // }
                                // ```
                                accumulate_bias_gradient(
                                    &mut model_gradients.first_layer_biases_gradients
                                        [first_layer_index],
                                    first_layer_pre_activation_gradients[first_layer_index].0,
                                    "переполнение градиента first_layer_biases",
                                )?;
                            }
                            first_layer_pre_activation_gradients
                        };
                        // Пример итераций этого цикла.
                        // ```json
                        // {
                        //   "EMBEDDING_DIMENSIONS": 2,
                        //   "CONTEXT_LENGTH": 3,
                        //   "индексы": [0, 1, 2, 3, 4, 5],
                        //   "для_индекса_4": {
                        //     "позиция_символа_в_контексте": 2,
                        //     "координата_эмбеддинга": 0
                        //   }
                        // }
                        // ```
                        for character_embeddings_flattened_index in
                            0..forward_pass_result.character_embeddings_flattened.len()
                        {
                            // Пример значения и его получения.
                            // ```json
                            // {
                            //   "самостоятельный_пример": true,
                            //   "веса_из_одной_координаты_входа": [0.5, -0.2],
                            //   "градиенты_первого_слоя": [-0.08, 0.03],
                            //   "вклады": [-0.04, -0.006],
                            //   "сумма": -0.046,
                            //   "назначение": "передать производную обратно в соответствующую координату C"
                            // }
                            // ```
                            let input_coordinate_gradient: FiniteF32 = {
                                // Пример значения и его получения.
                                // ```json
                                // {
                                //   "самостоятельный_пример": true,
                                //   "веса_из_одной_координаты_входа": [0.5, -0.2],
                                //   "градиенты_первого_слоя": [-0.08, 0.03],
                                //   "вклады": [-0.04, -0.006],
                                //   "сумма": -0.046,
                                //   "назначение": "передать производную обратно в соответствующую координату C"
                                // }
                                // ```
                                let mut input_coordinate_gradient: FiniteF32 = zero_gradient;
                                // Пример итераций этого цикла.
                                // ```json
                                // {
                                //   "учебный_FIRST_LAYER_SIZE": 2,
                                //   "first_layer_index": [0, 1],
                                //   "смысл": "каждую компоненту первого слоя обрабатываем отдельно",
                                //   "внутри_обратного_прохода": "суммируем вклады связанных символов или координат"
                                // }
                                // ```
                                for first_layer_index in 0..FIRST_LAYER_SIZE {
                                    // Пример именно этого вклада в градиент.
                                    // ```json
                                    // {
                                    //   "самостоятельный_пример": true,
                                    //   "куда": "first_layer_weights_gradients[0][0]",
                                    //   "накоплено_до": 0,
                                    //   "input_or_weight": "координата эмбеддинга 0.2",
                                    //   "output_gradient": -0.08,
                                    //   "вклад": -0.016,
                                    //   "после": -0.016
                                    // }
                                    // ```
                                    accumulate_gradient_product(
                                        &mut model_gradients.first_layer_weights_gradients
                                            [character_embeddings_flattened_index]
                                            [first_layer_index],
                                        forward_pass_result.character_embeddings_flattened
                                            [character_embeddings_flattened_index]
                                            .0,
                                        first_layer_pre_activation_gradients[first_layer_index].0,
                                        "переполнение градиента first_layer_weights",
                                    )?;
                                    // Пример именно этого вклада в градиент.
                                    // ```json
                                    // {
                                    //   "самостоятельный_пример": true,
                                    //   "куда": "input_coordinate_gradient",
                                    //   "накоплено_до": 0,
                                    //   "input_or_weight": "вес W1[0][0] = 0.5",
                                    //   "output_gradient": -0.08,
                                    //   "вклад": -0.04,
                                    //   "после": -0.04,
                                    //   "далее": "добавляем вклад следующей компоненты первого слоя"
                                    // }
                                    // ```
                                    accumulate_gradient_product(
                                        &mut input_coordinate_gradient,
                                        model.first_layer_weights
                                            [character_embeddings_flattened_index]
                                            [first_layer_index]
                                            .0,
                                        first_layer_pre_activation_gradients[first_layer_index].0,
                                        "переполнение градиента координаты эмбеддинга",
                                    )?;
                                }
                                input_coordinate_gradient
                            };
                            // Scatter-add возвращает производные координат в строки C; две позиции n складываются.
                            // ```json
                            // {
                            //   "контекст_индексы": [1, 2, 2],
                            //   "input_coordinate_gradients": [-0.042721631, -0.051795492, 0.029088933, -0.014544466, -0.09178182, -0.036339258],
                            //   "градиент_C_a": [-0.042721631, -0.051795492],
                            //   "вклад_первой_n": [0.029088933, -0.014544466],
                            //   "вклад_второй_n": [-0.09178182, -0.036339258],
                            //   "градиент_C_n": [-0.062692888, -0.050883724],
                            //   "градиент_C_точка": [0.0, 0.0],
                            //   "индексация": {
                            //     "flattened_index": 4,
                            //     "позиция_в_контексте": 2,
                            //     "координата_эмбеддинга": 0,
                            //     "строка_C": 2
                            //   }
                            // }
                            // ```
                            // Повторяющиеся символы используют одну строку C: складываем вклады всех их появлений.
                            let embedding_gradient: &mut FiniteF32 = &mut model_gradients.character_embeddings_gradients.0
                                    [previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                        .previous_vocabulary_character_usize_indexes
                                        [character_embeddings_flattened_index
                                            / EMBEDDING_DIMENSIONS]
                                        .0]
                                    [character_embeddings_flattened_index % EMBEDDING_DIMENSIONS];
                            // Пример изменения значения.
                            // ```json
                            // {
                            //   "flattened_index": 4,
                            //   "EMBEDDING_DIMENSIONS": 2,
                            //   "контекст_индексы": [1, 2, 2],
                            //   "позиция_в_контексте": 2,
                            //   "строка_C": 2,
                            //   "координата_C": 0,
                            //   "значение_до": 0.01,
                            //   "добавляемый_градиент": -0.046,
                            //   "после": -0.036
                            // }
                            // ```
                            *embedding_gradient = FiniteF32::try_from(
                                embedding_gradient.0 + input_coordinate_gradient.0,
                            )
                            .map_err(
                                |_: &'static str| -> &'static str {
                                    "переполнение градиента character_embeddings"
                                },
                            )?;
                        }
                    }
                    (
                        total_negative_log_likelihood_loss.0
                            / previous_vocabulary_character_indexes_and_next_character_index_batch
                                .len() as f32,
                        model_gradients,
                    )
                };
                // Шаг градиентного спуска выполняется после обработки всего батча.
                // ```json
                // {
                //   "параметр_до": 0.5,
                //   "средний_градиент": 0.2,
                //   "learning_rate": 0.1,
                //   "формула": "параметр - learning_rate * градиент",
                //   "параметр_после": 0.48,
                //   "применяется_к": [
                //     "C",
                //     "W1",
                //     "b1",
                //     "W2",
                //     "b2"
                //   ],
                //   "важно": "градиент уже усреднён по батчу; второй раз делить на размер батча не нужно"
                // }
                // ```
                for iteration_value in [
                    (
                        model.character_embeddings.0.as_flattened_mut(),
                        model_gradients
                            .character_embeddings_gradients
                            .as_flattened(),
                    ),
                    (
                        model.first_layer_weights.as_flattened_mut(),
                        model_gradients.first_layer_weights_gradients.as_flattened(),
                    ),
                    (
                        &mut model.first_layer_biases[..],
                        &model_gradients.first_layer_biases_gradients[..],
                    ),
                    (
                        model.second_layer_weights.0.as_flattened_mut(),
                        model_gradients
                            .second_layer_weights_gradients
                            .as_flattened(),
                    ),
                    (
                        &mut model.second_layer_biases.0[..],
                        &model_gradients.second_layer_biases_gradients[..],
                    ),
                ] {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "parameter_tensor": [0.5, -0.2],
                    //   "gradient_tensor": [0.2, -0.1],
                    //   "learning_rate": 0.1,
                    //   "параметры_после": [0.48, -0.19],
                    //   "размеры": "оба среза соответствуют одному полю Model"
                    // }
                    // ```
                    let (parameter_tensor, gradient_tensor): (&mut [FiniteF32], &[FiniteF32]) =
                        iteration_value;
                    // Пример итераций этого цикла.
                    // ```json
                    // {
                    //   "параметры_до": [0.5, -0.2],
                    //   "градиенты": [0.2, -0.1],
                    //   "learning_rate": 0.1,
                    //   "параметры_после": [0.48, -0.19],
                    //   "пары_из_zip": [
                    //     [0.5, 0.2],
                    //     [-0.2, -0.1]
                    //   ]
                    // }
                    // ```
                    for iteration_value in parameter_tensor.iter_mut().zip(gradient_tensor) {
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "parameter_до": 0.5,
                        //   "gradient": 0.2,
                        //   "learning_rate": 0.1,
                        //   "parameter_после": 0.48
                        // }
                        // ```
                        let (parameter, &gradient): (&mut FiniteF32, &FiniteF32) = iteration_value;
                        // Пример изменения значения.
                        // ```json
                        // {
                        //   "parameter_до": 0.5,
                        //   "learning_rate": 0.1,
                        //   "gradient": 0.2,
                        //   "шаг": 0.02,
                        //   "parameter_после": 0.48,
                        //   "проверка": "FiniteF32 до записи нового параметра"
                        // }
                        // ```
                        *parameter =
                            FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
                    }
                }
                CrossEntropyLossAsNonNegativeF32::try_from(loss)?
            };
            // Пример выбора ветки.
            // ```json
            // {
            //   "training_steps": 200000,
            //   "печатаем_на_шагах": [0, 10000, 20000, 199999],
            //   "не_печатаем_например": [1, 9999],
            //   "почему_199999": "последний шаг печатается даже без кратности 10000"
            // }
            // ```
            if training_step % 10_000 == 0 || training_step + 1 == training_steps.get() {
                println!(
                    "step={training_step:6} lr={learning_rate} loss={loss:.4} elapsed={:.1}s iteration_ms={:.3}",
                    training_started_at.elapsed().as_secs_f32(),
                    iteration_started_at.elapsed().as_secs_f64() * 1000.0,
                    loss = loss.0,
                    learning_rate = learning_rate.0,
                );
            }
        }
        model
    };
    // Итоговые числа ниже иллюстративные: train/dev/test не участвуют в одном общем среднем.
    // ```json
    // {
    //   "пример_лога": {
    //     "train_loss": 2.1,
    //     "dev_loss": 2.16,
    //     "test_loss": 2.18
    //   },
    //   "train": "набор, по которому меняли параметры",
    //   "dev": "отложенные имена для оценки и выбора настроек",
    //   "test": "отложенные имена для окончательной оценки",
    //   "параметры_при_оценке": "не меняются",
    //   "вывод": "только консоль, выходные файлы не создаются"
    // }
    // ```
    // 4. Считаем средние loss на train/dev/test без обновления параметров и печатаем метрики.
    // Dev служит для выбора настроек, test — для окончательной оценки после их выбора.
    {
        // Пример значения и его получения.
        // ```json
        // {
        //   "что_собирается": [
        //     "число шагов",
        //     "число параметров",
        //     "dev до обучения",
        //     "средние NLL на train/dev/test"
        //   ],
        //   "пример_строки": "train_loss=2.100000\ndev_loss=2.160000\n",
        //   "куда": "print! в консоль; файлов нет"
        // }
        // ```
        let metrics_text: String = format!(
            "steps={training_steps}\nparameters={trainable_scalar_count}\ninitial_dev_loss={initial_validation_loss:.6}\ntrain_loss={training_loss:.6}\ndev_loss={validation_loss:.6}\ntest_loss={test_loss:.6}\n",
            initial_validation_loss = initial_validation_loss.0,
            training_loss = loss(&model, NonEmptySlice::from(&training_character_index_pairs))?.0,
            validation_loss = loss(
                &model,
                NonEmptySlice::from(&validation_character_index_pairs)
            )?
            .0,
            test_loss = loss(&model, NonEmptySlice::from(&test_character_index_pairs))?.0,
        );
        print!("{metrics_text}");
    }
    // 5. Печатаем 20 имён: выбираем символ по вероятностям, завершаем на точке или после 100 символов.
    // Пример итераций этого цикла.
    // ```json
    // {
    //   "количество_имён": 20,
    //   "перед_каждым": "контекст [0,0,0] и пустая строка",
    //   "после_каждого": "печатаем одну строку имени"
    // }
    // ```
    for _ in 0..20 {
        // Пример значения и его получения.
        // ```json
        // {
        //   "контекст_сначала": [0, 0, 0],
        //   "выбранные_символы": [
        //     "a",
        //     "n",
        //     "."
        //   ],
        //   "name": "an.",
        //   "пометка_лимита": "только если точка не выбрана за 100 символов"
        // }
        // ```
        let name: String = 'sample: {
            // Пример значения и его получения.
            // ```json
            // {
            //   "CONTEXT_LENGTH": 3,
            //   "dot_index": 0,
            //   "начало": [0, 0, 0],
            //   "после_a": [0, 0, 1],
            //   "после_n": [0, 1, 2]
            // }
            // ```
            let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                CONTEXT_LENGTH] = [vocabulary.dot_index(); CONTEXT_LENGTH];
            // Пример значения и его получения.
            // ```json
            // {
            //   "начало": "",
            //   "после_первой_буквы": "a",
            //   "после_второй": "an",
            //   "после_точки": "an.",
            //   "при_лимите": "an... [лимит 100 символов]"
            // }
            // ```
            let mut generated_name: String = String::new();
            // Пример итераций этого цикла.
            // ```json
            // {
            //   "максимум_символов": 100,
            //   "пример_выборов": [
            //     "a",
            //     "n",
            //     "."
            //   ],
            //   "число_итераций_примера": 3,
            //   "причина_остановки": "точка; остальные 97 итераций не выполняются"
            // }
            // ```
            for _ in 0..100 {
                // Пример значения и его получения.
                // ```json
                // {
                //   "самостоятельный_пример": true,
                //   "character_scores": [0, 2, 1],
                //   "цель": 1,
                //   "результат": {
                //     "next_character_probabilities": [0.090030573, 0.665240956, 0.244728471],
                //     "negative_log_likelihood_loss": 0.407605964
                //   }
                // }
                // ```
                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                    NonEmptySlice::from(
                        &forward_pass(&model, previous_vocabulary_character_usize_indexes)?
                            .character_scores,
                    ),
                    vocabulary.dot_index(),
                )?;
                // Пример значения и его получения.
                // ```json
                // {
                //   "диапазон": "[0, 1)",
                //   "пример": 0.8,
                //   "использование": "выбирает интервал накопленной вероятности",
                //   "при_вероятностях": [0.1, 0.6, 0.3],
                //   "выбранный_индекс": 2
                // }
                // ```
                let random_probability: CharacterProbabilityAsF32From0To1 =
                    CharacterProbabilityAsF32From0To1::try_from(rand::RngExt::random::<f32>(
                        &mut random_generator,
                    ))?;
                // Генерация: выбираем символ по случайному числу, а не обязательно самый вероятный.
                // ```json
                // {
                //   "Vocabulary": [
                //     ".",
                //     "a",
                //     "n"
                //   ],
                //   "next_character_probabilities": [0.1, 0.6, 0.3],
                //   "интервалы": [
                //     {
                //       "индекс": 0,
                //       "символ": ".",
                //       "от_включительно": 0.0,
                //       "до_исключительно": 0.1
                //     },
                //     {
                //       "индекс": 1,
                //       "символ": "a",
                //       "от_включительно": 0.1,
                //       "до_исключительно": 0.7
                //     },
                //     {
                //       "индекс": 2,
                //       "символ": "n",
                //       "от_включительно": 0.7,
                //       "до_исключительно": 1.0
                //     }
                //   ],
                //   "random_probability": 0.8,
                //   "выбранный_индекс": 2,
                //   "выбранный_символ": "n",
                //   "контекст_до": [0, 0, 1],
                //   "контекст_после": [0, 1, 2],
                //   "если_выбрана_точка": "добавляем точку в печатаемое имя и завершаем его",
                //   "ограничение": "после 100 символов без точки добавляется пометка о лимите",
                //   "NLL_при_генерации": "cross_entropy получает dot_index как техническую цель; её NLL здесь игнорируется, нужны только вероятности"
                // }
                // ```
                // Выбираем интервал накопленной вероятности; последний индекс — запасной из-за округления суммы.
                let sampled_character_index: usize = {
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "начало": 0,
                    //   "вероятности": [0.1, 0.6, 0.3],
                    //   "после_каждого_сложения": [0.1, 0.7, 1.0],
                    //   "использование": "сравниваем с одним random_probability, общим для этого выбора символа"
                    // }
                    // ```
                    let mut cumulative_probability: f32 = ZERO_F32;
                    // Пример значения и его получения.
                    // ```json
                    // {
                    //   "вероятности": [0.1, 0.6, 0.3],
                    //   "random_probability": 0.8,
                    //   "накопленные_суммы": [0.1, 0.7, 1.0],
                    //   "результат": 2,
                    //   "резервное_значение": "последний индекс на случай округления суммы вниз от 1"
                    // }
                    // ```
                    let mut sampled_character_index: usize = vocabulary_len - 1;
                    // Пример итераций этого цикла.
                    // ```json
                    // {
                    //   "вероятности": [0.1, 0.6, 0.3],
                    //   "пары_enumerate": [
                    //     [0, 0.1],
                    //     [1, 0.6],
                    //     [2, 0.3]
                    //   ],
                    //   "random_probability": 0.8,
                    //   "остановка": "когда сумма впервые превысит 0.8, выбираем индекс 2"
                    // }
                    // ```
                    for iteration_value in next_character_probabilities_and_negative_log_likelihood
                        .next_character_probabilities
                        .iter()
                        .enumerate()
                    {
                        // Пример значения и его получения.
                        // ```json
                        // {
                        //   "iteration_value": [1, 0.6],
                        //   "character_index": 1,
                        //   "character_probability": 0.6,
                        //   "порядок": "позиция в Vocabulary и вероятность символа на этой позиции"
                        // }
                        // ```
                        let (character_index, character_probability): (
                            usize,
                            &CharacterProbabilityAsF32From0To1,
                        ) = iteration_value;
                        // Пример изменения значения.
                        // ```json
                        // {
                        //   "до": 0.1,
                        //   "вероятность_текущего_символа": 0.6,
                        //   "после": 0.7
                        // }
                        // ```
                        cumulative_probability += character_probability.0;
                        // Пример выбора ветки.
                        // ```json
                        // {
                        //   "random_probability": 0.65,
                        //   "проверки": [
                        //     {
                        //       "накоплено": 0.1,
                        //       "условие": false
                        //     },
                        //     {
                        //       "накоплено": 0.7,
                        //       "условие": true
                        //     }
                        //   ],
                        //   "результат": "выбираем второй символ и прекращаем поиск"
                        // }
                        // ```
                        if random_probability.0 < cumulative_probability {
                            // Пример изменения значения.
                            // ```json
                            // {
                            //   "условие": "0.65 < накопленная сумма 0.7",
                            //   "выбранная_позиция": 1,
                            //   "после_присваивания": "break завершает поиск; это индекс a, если Vocabulary=[.,a,n]"
                            // }
                            // ```
                            sampled_character_index = character_index;
                            break;
                        }
                    }
                    sampled_character_index
                };
                // Пример значения и его получения.
                // ```json
                // {
                //   "вероятности": [0.1, 0.6, 0.3],
                //   "random_probability": 0.8,
                //   "накопленные_суммы": [0.1, 0.7, 1.0],
                //   "результат": 2,
                //   "резервное_значение": "последний индекс на случай округления суммы вниз от 1"
                // }
                // ```
                let sampled_character_index: VocabularyCharacterIndexAsUsize =
                    VocabularyCharacterIndexAsUsize::try_from((
                        sampled_character_index,
                        &vocabulary,
                    ))?;
                // Точку конца имени тоже включаем в консольную строку.
                generated_name.push(vocabulary.character(sampled_character_index)?);
                // Пример выбора ветки.
                // ```json
                // {
                //   "Vocabulary": [
                //     ".",
                //     "a",
                //     "n"
                //   ],
                //   "индекс_0": {
                //     "символ": ".",
                //     "действие": "break sample; имя завершено"
                //   },
                //   "индекс_1": {
                //     "символ": "a",
                //     "действие": "продолжаем генерацию с обновлённым контекстом"
                //   }
                // }
                // ```
                if sampled_character_index.0 == 0 {
                    break 'sample generated_name;
                }
                previous_vocabulary_character_usize_indexes.rotate_left(1);
                // Пример изменения значения.
                // ```json
                // {
                //   "до_замены_последнего": [1, 2, 0],
                //   "новый_индекс": 2,
                //   "после": [1, 2, 2],
                //   "последний_индекс_массива": 2
                // }
                // ```
                previous_vocabulary_character_usize_indexes[CONTEXT_LENGTH - 1] =
                    sampled_character_index;
            }
            generated_name.push_str(" [лимит 100 символов]");
            generated_name
        };
        println!("{name}");
    }
    println!(
        "Общее время работы программы: {:.2} с",
        program_started_at.elapsed().as_secs_f64()
    );
    Ok(())
}
