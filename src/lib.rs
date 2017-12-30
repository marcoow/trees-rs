#![feature(match_default_bindings, nll)]

use std::mem;

pub trait Key {
    type Key: Ord;

    fn key(&self) -> &Self::Key;
}

impl<T: Ord> Key for T {
    type Key = T;

    fn key(&self) -> &T {
        self
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

impl<T: Key> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn find(&mut self, key: &T::Key) -> Option<&mut T> {
        let link = self.link(key);

        match link {
            None => None,
            Some(node) => Some(&mut node.data),
        }
    }

    pub fn insert(&mut self, data: T) {
        let link = self.link(data.key());

        match link {
            None => *link = Some(Box::new(Node::new(data))),
            Some(node) => node.data = data,
        }
    }

    pub fn delete(&mut self, key: &T::Key) {
        let link = self.link(key);

        if let Some(node) = link {
            let mut successor = &mut node.right;

            match successor {
                None => *link = node.left.take(),
                Some(right_node) => {
                    let mut successor_node = right_node;

                    while let Some(left) = &mut successor_node.left {
                        successor = &mut successor_node.left;
                        successor_node = left;
                    }

                    mem::swap(&mut node.data, &mut successor_node.data);
                    *successor = successor_node.right.take();
                }
            }
        }
    }

    fn link(&mut self, key: &T::Key) -> &mut Link<T> {
        let mut link = &mut self.root;

        loop {
            link = match link {
                Some(node) if key < node.data.key() => &mut node.left,
                Some(node) if key > node.data.key() => &mut node.right,
                _ => return link,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BST;

    #[test]
    fn insert() {
        let mut bst = BST::new();

        assert_eq!(bst.find(&5), None);

        bst.insert(5);

        assert_eq!(bst.find(&5), Some(&mut 5));

        bst.insert(9);

        assert_eq!(bst.find(&5), Some(&mut 5));
        assert_eq!(bst.find(&9), Some(&mut 9));

        bst.insert(9);

        assert_eq!(bst.find(&5), Some(&mut 5));
        assert_eq!(bst.find(&9), Some(&mut 9));

        bst.insert(8);

        assert_eq!(bst.find(&5), Some(&mut 5));
        assert_eq!(bst.find(&8), Some(&mut 8));
        assert_eq!(bst.find(&9), Some(&mut 9));
    }

    #[test]
    fn delete() {
        for j in 0..7 {
            //      2
            //    /   \
            //   1     5
            //  /     /  \
            // 0     3    6
            //        \
            //         4

            let mut bst = BST::new();

            bst.insert(2);
            bst.insert(1);
            bst.insert(0);
            bst.insert(5);
            bst.insert(3);
            bst.insert(4);
            bst.insert(6);

            for mut i in 0..7 {
                assert_eq!(bst.find(&i), Some(&mut i));
            }

            bst.delete(&j);

            for mut i in 0..7 {
                assert_eq!(bst.find(&i), if i == j { None } else { Some(&mut i) });
            }
        }
    }
}
