mod list;

use list::{List, Book};
use std::any::type_name;


fn main() {
    // Crear una instancia de la estructura List
    let mut mi_lista = List::new(None);

    // Crear instancias de la estructura Book
    let libro1 = Book::new("Libro 1".to_string());
    let libro2 = Book::new("Libro 2".to_string());
    let libro3 = Book::new("Libro 3".to_string());
    let libro4 = Book::new("Libro 4".to_string());
    let libro5 = Book::new("Libro 5".to_string());


    // Insertar libros en la primera posición de la lista
    mi_lista.insert_last_position(libro4);
    mi_lista.insert_first_position(libro1);
    mi_lista.insert_first_position(libro5);
    mi_lista.insert_last_position(libro3);
    mi_lista.insert_last_position(libro2);

    mi_lista.print_list();
    // Imprimir la lista
    mi_lista.get_book_at_position(0);

    println!("el tamaño de la lista es {:?}", mi_lista.size())
}


fn print_type<T>(valor: T) {
    println!("El tipo de dato es: {}", type_name::<T>());
}

fn print<T: std::fmt::Debug>(valor: T) {
    println!("{:?}", valor);
}

