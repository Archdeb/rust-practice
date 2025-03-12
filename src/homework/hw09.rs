// Функція для виконання зсуву рядка на n позицій
fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s; // Якщо рядок порожній, то нічого не зсуваємо
    }

    // Перетворюємо зсув у коректне значення в межах довжини рядка
    let n = (n % len as isize + len as isize) % len as isize; // Забезпечуємо правильний зсув

    let (left, right) = s.split_at((len as isize - n) as usize);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| 
            assert_eq!(
                rotate(s.to_string(), *n), 
                exp.to_string()
            )
        );
}
