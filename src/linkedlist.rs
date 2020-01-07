enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIterator<'a, T:'a> {
    cur: &'a List<T>
}

impl<'a, T> Iterator<&'a T> for ListIterator<'a, T> {
    fn next(&mut self) -> Option<&'a T>{
        match self.cur {
            &Cons(ref val, box ref next) => {self.cur = next; Some(val)},
            &Nil => None
        }
    }
}

impl<'a, T> List<T> {
    fn iter(&'a self) -> ListIterator<'a, T> {
        ListIterator{cur: self}
    }
    fn add(self, elem: T) -> List<T> {
        Cons(elem, box self)
    }
}

impl<T:std::fmt::Show> std::fmt::Show for List<T> {
    fn fmt(&self, ft: &mut std::fmt::Formatter) -> Result<(), std::fmt::FormatError> {
        for (i,elem) in self.iter().enumerate() {
            let _ = ft.write_str(if i==0 {""} else {"->"});
            match format_args!(|x| ft.write_fmt(x), "({})", elem) {
                Ok(()) => {}
                Err(_) => {return Err(std::fmt::WriteError);}
            };
        }
        Ok(())
    }
}

fn main(){
    let l = Nil.add(1i ).add(2).add(3);
    println!("{}",l);
    for i in l.iter() {let mut c=*i; c+=1; println!("{}", c)}
}