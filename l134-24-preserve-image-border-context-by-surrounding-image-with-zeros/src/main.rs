// Урок 134. Помещать изображение внутрь большей таблицы с нулевой рамкой.
// Дополнение позволяет применять фильтр у границ, где иначе не хватает соседних пикселей.

fn main() {
    let image: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    let mut image_with_zero_border: [[i32; 4]; 4] = [[0; 4]; 4];
    for row in 0..2 {
        for column in 0..2 {
            image_with_zero_border[row + 1][column + 1] = image[row][column];
        }
    }

    // Выполняем вычисления из примера.
    let _ = image_with_zero_border;

    println!("До={image:?}; с рамкой={image_with_zero_border:?}");
    assert_eq!(image_with_zero_border[0], [0; 4]);
    assert_eq!(image_with_zero_border[1][1], image[0][0]);
    let windows_without = (image.len() - 2 + 1).pow(2);
    let windows_with = (image_with_zero_border.len() - 2 + 1).pow(2);
    println!("Положений фильтра 2x2: без рамки={windows_without}, с рамкой={windows_with}");
    assert!(windows_with > windows_without);
}

// Чему учит этот урок:
// Учимся помещать изображение внутрь большей таблицы с нулевой рамкой.
// Дополнение позволяет применять фильтр у границ, где иначе не хватает соседних пикселей.
