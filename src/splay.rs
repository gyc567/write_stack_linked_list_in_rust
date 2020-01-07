extern crate slab;

use slab::Slab;

/// The null index, akin to null pointers.
///
/// Just like a null pointer indicates an address no object is ever stored at,
/// the null index indicates an index no object is ever stored at.
///
/// Number `!0` is the largest possible value representable by `usize`.
const NULL: usize = !0;

struct Node<T> {
    /// Parent node.
    parent: usize,

    /// Left and right child.
    children: [usize; 2],

    /// Actual value stored in node.
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            parent: NULL,
            children: [NULL, NULL],
            value: value,
        }
    }
}

struct Splay<T> {
    /// This is where nodes are stored.
    slab: Slab<Node<T>>,

    /// The root node.
    root: usize,
}

impl<T> Splay<T>
where
    T: Ord,
{
    /// Constructs a new, empty splay tree.
    fn new() -> Splay<T> {
        Splay {
            slab: Slab::new(),
            root: NULL,
        }
    }

    /// Links nodes `p` and `c` as parent and child with the specified direction.
    #[inline(always)]
    fn link(&mut self, p: usize, c: usize, dir: usize) {
        self.slab[p].children[dir] = c;
        if c != NULL {
            self.slab[c].parent = p;
        }
    }

    /// Performs a rotation on node `c`, whose parent is node `p`.
    #[inline(always)]
    fn rotate(&mut self, p: usize, c: usize) {
        // Variables:
        // - `c` is the child node
        // - `p` is it's parent
        // - `g` is it's grandparent

        // Find the grandparent.
        let g = self.slab[p].parent;

        // The direction of p-c relationship.
        let dir = if self.slab[p].children[0] == c { 0 } else { 1 };

        // This is the child of `c` that needs to be reassigned to `p`.
        let t = self.slab[c].children[dir ^ 1];

        self.link(p, t, dir);
        self.link(c, p, dir ^ 1);

        if g == NULL {
            // There is no grandparent, so `c` becomes the root.
            self.root = c;
            self.slab[c].parent = NULL;
        } else {
            // Link `g` and `c` together.
            let dir = if self.slab[g].children[0] == p { 0 } else { 1 };
            self.link(g, c, dir);
        }
    }

    /// Splays a node, rebalancing the tree in process.
    fn splay(&mut self, c: usize) {
        loop {
            // Variables:
            // - `c` is the current node
            // - `p` is it's parent
            // - `g` is it's grandparent

            // Find the parent.
            let p = self.slab[c].parent;
            if p == NULL {
                // There is no parent. That means `c` is the root.
                break;
            }

            // Find the grandparent.
            let g = self.slab[p].parent;
            if g == NULL {
                // There is no grandparent. Just one more rotation is left.
                // Zig step.
                self.rotate(p, c);
                break;
            }

            if (self.slab[g].children[0] == p) == (self.slab[p].children[0] == c) {
                // Zig-zig step.
                self.rotate(g, p);
                self.rotate(p, c);
            } else {
                // Zig-zag step.
                self.rotate(p, c);
                self.rotate(g, c);
            }
        }
    }

    /// Inserts a new node with specified `value`.
    fn insert(&mut self, value: T) {
        // Variables:
        // - `n` is the new node
        // - `p` will be it's parent
        // - `c` is the present child of `p`

        let n = self.slab.insert(Node::new(value));

        if self.root == NULL {
            self.root = n;
        } else {
            let mut p = self.root;
            loop {
                // Decide whether to go left or right.
                let dir = if self.slab[n].value < self.slab[p].value {
                    0
                } else {
                    1
                };
                let c = self.slab[p].children[dir];

                if c == NULL {
                    self.link(p, n, dir);
                    self.splay(n);
                    break;
                }
                p = c;
            }
        }
    }
}

fn main() {
    let mut splay = Splay::new();

    // Insert a bunch of pseudorandom numbers.
    let mut num = 1u32;
    for _ in 0..1000000 {
        num = num.wrapping_mul(17).wrapping_add(255);
        splay.insert(num);
    }
}
