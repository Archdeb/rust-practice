fn main() {
    // Виклик функції малювання ялинки
    draw_tree(6); // Малюємо ялинку з 5 трикутниками як у документі
}

// Функція для малювання ялинки
pub fn draw_tree(triangles: u32) {
    for i in 1..=triangles {
        // Обчислюємо ширину найширшого рівня
        let max_width = 2 * triangles + (triangles - 1) * 2 + 1;

        for j in 1..=i {
            let width = 2 * j - 1; // Ширина поточного рівня
            let spaces = (max_width - width) / 2; // Кількість пробілів для центрованого вирівнювання

            let line = " ".repeat(spaces as usize) + &"*".repeat(width as usize);
            println!("{}", line);
        }
    }
}
