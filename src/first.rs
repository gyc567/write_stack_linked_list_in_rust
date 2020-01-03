// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
