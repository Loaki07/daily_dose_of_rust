use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    height: i8, // could be bool for Red and Black, but math is easier with a small int.
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.height,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.height = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                } else {
                    bd.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    height: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
            }
        }
        self.set_height();
    }
}

impl <T: Debug> BinTree<T> {
    pub fn print_l_first(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_l_first(dp + 1);
            let mut spc = String::new();

            for _ in 0..dp {
                spc.push('.');
            }

            println!("{}:{}{:?}", bd.height, spc, bd.data);
            bd.right.print_l_first(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);

    t.print_l_first(0);

    // println!(" t = {:?} ", t);
}
