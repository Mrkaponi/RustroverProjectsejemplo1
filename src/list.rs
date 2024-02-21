    use crate::print;

    #[derive(Debug)] // Agrega esta línea para derivar automáticamente el trait Debug
    pub(crate) struct Book {
        title_name: String,
    }

    impl Book {
        pub fn new(title_name: String) -> Self {
            Self { title_name }
        }
    }

    #[derive(Debug)] // Agrega esta línea para derivar automáticamente el trait Debug
    pub(crate) struct Node {
        next: Option<Box<Node>>,
        book: Book,
        position: Option<usize>,
    }

    impl Node {
        fn new_node(b: Book) -> Node {
            Node {
                next: None,
                book: b,
                position: None,
            }
        }


        pub fn new(book: Book) -> Self {
            Self { next: (None), book, position: (None) }
        }
        pub fn set_position(&mut self, position: usize) {
            self.position = Option::from(position);
        }
        pub fn position(&self) -> Option<usize> {
            self.position
        }
    }

    pub(crate) struct List {
        head: Option<Box<Node>>,
        size: usize,
    }

    impl List {
        pub fn insert_first_position(&mut self, book: Book) {
            let mut new_node = Box::new(Node::new(book));
            new_node.next = self.head.take();
            self.head = Some(new_node);


            let mut current_node = &mut self.head;
            let mut position = 0;
            while let Some(node) = current_node {
                node.set_position(position);
                position += 1;
                current_node = &mut node.next;
            }

            self.size += 1;
        }

        pub fn insert_last_position(&mut self, book: Book) {
            let new_node = Box::new(Node::new(book));

            if let Some(mut puntero) = self.head.as_mut() {
                let mut position = 0;
                while let Some(ref mut next_node) = puntero.next {
                    puntero = next_node;
                    position += 1;
                }
                puntero.next = Some(new_node);
                puntero.next.as_mut().unwrap().set_position(position); // Asigna la posición al nuevo nodo
            } else {
                self.head = Some(new_node);
            }
            self.size += 1;
        }



        pub fn get_book_at_position(&self, position: usize) -> Option<&Book> {
            let mut current_node = &self.head;
            let mut cont = 0;

            while let Some(node) = current_node {
                if position == cont {
                    return Some(&node.book);
                }

                current_node = &node.next;
                cont += 1;
            }

            None
        }


        pub fn size(&self) -> usize {

            return self.size;
        }




        pub fn insert_to_specified_position(&mut self, book: Book, position: usize) {
            let mut current_node = &mut self.head;
            let mut cont = 0;
            while let Some(node) = current_node {
                if cont == position {
                    let mut new_node = Box::new(Node::new(book));
                    new_node.next = node.next.take();
                    node.next = Some(new_node);

                    let mut current_position = position + 1;
                    let mut current_next = &mut node.next;
                    while let Some(next_node) = current_next {
                        next_node.set_position(current_position);
                        current_position += 1;
                        current_next = &mut next_node.next;
                    }

                    self.size += 1;
                    return;
                }
                current_node = &mut node.next;
                cont += 1;
            }
            if cont == self.size {
                self.insert_last_position(book);
            }
        }

        pub fn print_list(&self) {
            let mut current_node = &self.head;
            let mut cont = 0;

            while let Some(node) = current_node {
                println!("psicion numero {:?}", cont);
                print(&node.book);
                current_node = &node.next;

                if self.head.is_none() {
                    print("No hay valores para imprimir");
                }
                cont = cont + 1
            }

            if self.head.is_none() {
                print("No hay valores para imprimir");
            }
        }

        pub fn new(head: Option<Box<Node>>) -> Self {
            Self { head, size: 0 }
        }
    }










