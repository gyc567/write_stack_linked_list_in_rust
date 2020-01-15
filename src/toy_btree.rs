use std::cmp::Ordering;

#[derive(Debug)]
pub enum BTree<T: Ord + Clone> {
    Node {
        v: T,
        l: Box<BTree<T>>,
        r: Box<BTree<T>>,
    },
    Empty,
}

impl<T: Ord + Clone> BTree<T> {
    fn new() -> BTree<T> {
        BTree::Empty
    }

    fn insert(&mut self, nv: T) {
        match self {
            &mut BTree::Node {
                ref v,
                ref mut l,
                ref mut r,
            } => {
                let c = nv.cmp(v);
                match c {
                    Ordering::Less => {
                        l.insert(nv);
                    }
                    Ordering::Greater => {
                        r.insert(nv);
                    }
                    _ => return, //Already in the tree
                };
            }
            &mut BTree::Empty => {
                /*Found an empty leaf -- insert*/
                *self = BTree::Node {
                    v: nv,
                    l: Box::new(BTree::Empty),
                    r: Box::new(BTree::Empty),
                };
            }
        };
    }

    fn is_empty(&self) -> bool {
        match self {
            &BTree::Empty => true,
            _ => false,
        }
    }

    fn find(&self, fv: T) -> bool {
        match self {
            &BTree::Node {
                ref v,
                ref l,
                ref r,
            } => {
                let c = fv.cmp(v);
                match c {
                    Ordering::Less => l.find(fv),
                    Ordering::Greater => r.find(fv),
                    _ => true,
                }
            }
            &BTree::Empty => false,
        }
    }
    fn delete(&mut self, fv: T) -> bool {
        match self {
            BTree::Node {
                ref v,
                ref mut l,
                ref mut r,
            } => {
                let c = fv.cmp(v);
                match c {
                    Ordering::Less => l.delete(fv),
                    Ordering::Greater => r.delete(fv),
                    _ => {
                        *self = BTree::Empty;
                        true
                    }
                }
            }
            BTree::Empty => false,
        }
    }

    fn min(&self) -> Option<T> {
        match self {
            &BTree::Empty => None,
            &BTree::Node {
                ref v,
                ref l,
                ref r,
            } => match &**l {
                &BTree::Empty => Some(v.clone()),
                _ => l.min(),
            },
        }
    }
    fn max(&self) -> Option<T> {
        match self {
            &BTree::Empty => None,
            &BTree::Node {
                ref v,
                ref l,
                ref r,
            } => match &**r {
                &BTree::Empty => Some(v.clone()),
                _ => r.max(),
            },
        }
    }
    fn findNode(&self, fv: T) -> Option<T> {
        match self {
            &BTree::Node {
                ref v,
                ref l,
                ref r,
            } => {
                let c = fv.cmp(v);
                match c {
                    Ordering::Less => l.findNode(fv),
                    Ordering::Greater => r.findNode(fv),
                    _ => Some(fv),
                }
            }
            &BTree::Empty => None,
        }
    }
}

fn main() {
    let mut t: BTree<u64> = BTree::new();
    println!("Is Empty? {}", t.is_empty());
    t.insert(128);
    t.insert(256);
    t.insert(4500000001);
    println!("256: {}", t.find(256));
    println!("128: {}", t.find(128));
    println!("12: {}", t.find(12));
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut t: BTree<u64> = BTree::new();
        println!("Is Empty? {}", t.is_empty());
        assert_eq!(true, t.is_empty());
        t.insert(128);
        t.insert(129);
        t.insert(256);
        t.insert(4500000001);
        println!("256: {:?}", t.findNode(256));
        assert_eq!(Some(256), t.findNode(256));
        assert_eq!(true, t.find(256));
        assert_eq!(false, t.find(2));
        assert_eq!(true, t.find(128));
        assert_eq!(true, t.find(4500000001));
        assert_eq!(Some(128), t.min());
        assert_eq!(Some(4500000001), t.max());
        assert_eq!(true, t.find(129));
        assert_eq!(true, t.delete(129));
        assert_eq!(false, t.find(129));
    }
}
