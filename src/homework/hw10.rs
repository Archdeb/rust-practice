// Функція перевіряє, чи є число паліндромом
fn is_palindrome(x: u32) -> bool {
    let original = x; // Зберігаємо початкове значення числа
    let mut reversed = 0; // Ініціалізуємо змінну для зворотного числа

    // Проходимо через кожну цифру числа
    let mut num = x;
    while num > 0 {
        // Отримуємо останню цифру числа
        let digit = num % 10;
        reversed = reversed * 10 + digit; // Додаємо цифру до зворотного числа
        num /= 10; // Видаляємо останню цифру з числа
    }

    // Порівнюємо зворотнє число з оригінальним
    original == reversed
}

#[test]
fn test() {
    let data = [
        (123, false),   // 123 не є паліндромом
        (121, true),    // 121 є паліндромом
        (1221, true),   // 1221 є паліндромом
    ];

    // Перевіряємо кожен тестовий випадок
    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp); // Порівнюємо результат з очікуваним
    });
}
