struct Vector<T> {
    data: Vec<T>,
}

impl<T: Clone + Default> Vector<T> {
    // Создание нового вектора
    fn new() -> Self {
        Vector { data: Vec::new() }
    }

    // Создание вектора с заданным размером
    fn with_capacity(capacity: usize) -> Self {
        Vector { data: Vec::with_capacity(capacity) }
    }

    // Внесение значения в вектор
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    // Возвращение последнего элемента и его удаление
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    // Удаление заданного индексом элемента и его возвращение
    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    // Получение элемента по индексу
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // Изменение размера вектора и заполнение новых элементов нулём
    fn resize(&mut self, new_size: usize) {
        let default = Default::default();
        self.data.resize(new_size, default);
    }
}

fn main() {
    let mut vector: Vector<i32> = Vector::new();
    vector.push(1);
    vector.push(3);
    vector.push(7);
    println!("{:?}", vector.get(1)); // Вывод: Some(3)
    println!("{:?}", vector.pop()); // Вывод: Some(7)
    println!("{:?}", vector.remove(0)); // Вывод: Some(1)
    println!("{:?}", vector.get(0)); // Вывод: Some(3)
    vector.resize(5);
    println!("{:?}", vector.get(4)); // Вывод: Some(0)
}