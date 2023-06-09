// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/897181/rust-4ms-100/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

///
/// get "Pointer" of a Tree Node
fn to_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => Some(Rc::clone(node)),
        None => None,
    }
}

///
/// get value of a Tree Node
fn val_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            Some(node.val)
        }
        None => None,
    }
}

///
/// get left of a Tree Node
fn left_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.left {
                Some(l) => Some(Rc::clone(l)),
                None => None,
            }
        }
        None => None,
    }
}

///
/// get right of a Tree Node
fn right_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.right {
                Some(r) => Some(Rc::clone(r)),
                None => None,
            }
        }
        None => None,
    }
}

/// root is not null
/// left is none or left has no child
fn append_to_left(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.left = to_rc(&ans);
        }
        None => {}
    }
    ans
}

/// root is not null
/// right is none or right has no child
fn append_to_right(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.right = to_rc(&ans);
        }
        None => {}
    }
    ans
}

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();
        ans.push('[');
        let mut q = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        if root.is_some() {
            q.push_back(to_rc(&root));
            ans.push_str(&val_of(&root).unwrap().to_string());
            ans.push(',');
        }
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            let left = left_of(&node);
            let right = right_of(&node);
            if left.is_none() {
                ans.push_str("null,");
            } else {
                ans.push_str(&val_of(&left).unwrap().to_string());
                ans.push(',');
                q.push_back(left);
            }
            if right.is_none() {
                ans.push_str("null,");
            } else {
                ans.push_str(&val_of(&right).unwrap().to_string());
                ans.push(',');
                q.push_back(right);
            }
        }
        while ans.len() > 5 && &ans[ans.len() - 5..] == "null," {
            ans.truncate(ans.len() - 5);
        }
        match ans.pop() {
            Some(ch) if ch == ',' => {}
            Some(ch) => ans.push(ch),
            None => {}
        }
        ans.push(']');
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() < 3 {
            return None;
        }
        let strs = data[1..data.len() - 1]
            .split(',')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let n = strs.len();
        // println!("{:?}", node);
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            strs[0].parse::<i32>().unwrap(),
        ))));

        let mut q = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        q.push_back(to_rc(&root));
        let mut i = 1;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                if i < n && strs[i] != "null" {
                    let left = append_to_left(&node, strs[i].parse::<i32>().unwrap());
                    q.push_back(left);
                };
                i += 1;
                if i < n && strs[i] != "null" {
                    let right = append_to_right(&node, strs[i].parse::<i32>().unwrap());
                    q.push_back(right);
                }
                i += 1;
            }
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let codec = Codec::new();
        let s = "[1,2,3,null,null,4,5]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_serde_02() {
        let codec = Codec::new();
        let s = "[]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_serde_03() {
        let codec = Codec::new();
        let s = "[1]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_serde_04() {
        let codec = Codec::new();
        let s = "[1,2]".to_owned();
        let root = codec.deserialize(s.clone());
        //        println!("{:?}", root);
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }
}