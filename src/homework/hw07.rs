// Функція для зміни регістру букв
fn invert_the_case(s: String) -> String {
    // Перебір кожного символу рядка
    s.chars()
    // Якщо символ - мала буква, перетворюємо в велику
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                // Якщо символ - велика буква, перетворюємо в малу
                c.to_lowercase().to_string()
            }
        })
        // Збираємо символи в новий рядок
        .collect()
}

#[test]
fn test() {
    // Тести з різними значеннями для перевірки функції
    let data = [
        ("Hello", "hELLO"), // Тестуємо на слові з англійськими літерами
        ("Привіт", "пРИВІТ"), // Тестуємо на слові з українськими літерами
    ];

    // Для кожної пари тестових значень виконуємо перевірку
    data.iter().for_each(|(a, b)| {
        // Перевіряємо, що зміна регістру працює правильно
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}
