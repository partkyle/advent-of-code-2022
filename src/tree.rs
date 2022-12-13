pub mod tree {
    use std::fmt::{self, Debug};
    use std::{cell::RefCell, rc::Rc};

    pub struct Tree<T: Debug> {
        pub parent: Option<Rc<RefCell<Tree<T>>>>,
        pub children: Vec<Rc<RefCell<Tree<T>>>>,
        pub data: T,
    }

    impl<T: Debug> fmt::Debug for Tree<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut s = f.debug_struct("Tree");
            s.field("data", &self.data);
            if self.children.len() > 0 {
                s.field("children", &self.children);
            }
            s.finish()
        }
    }

    impl<T: Debug> Tree<T> {
        pub fn new(data: T) -> Rc<RefCell<Tree<T>>> {
            Rc::new(RefCell::new(Tree {
                parent: None,
                children: vec![],
                data: data,
            }))
        }

        pub fn add_child(parent: &Rc<RefCell<Tree<T>>>, data: T) -> Rc<RefCell<Tree<T>>> {
            let child = Rc::new(RefCell::new(Tree {
                parent: Some(Rc::clone(parent)),
                children: vec![],
                data: data,
            }));

            let mut real_parent = parent.as_ref().borrow_mut();
            real_parent.children.push(Rc::clone(&child));

            child
        }
    }
}

#[cfg(test)]
mod test {

    use super::tree::*;

    #[test]
    fn test() {
        let root = Tree::new("testing");

        let doot = Tree::add_child(&root, "doot");
        Tree::add_child(&doot, "woot");
        Tree::add_child(&doot, "toor");

        println!("{:?}", root);
    }
}
