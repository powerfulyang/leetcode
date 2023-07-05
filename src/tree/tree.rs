#![allow(dead_code)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

#[allow(dead_code)]
impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Tree {
    root: Option<Box<TreeNode>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    // 前序遍历 迭代版
    fn preorder_traversal(&self) -> Vec<i32> {
        let mut stack = vec![];
        let mut p = self.root.as_ref();
        let mut ans = vec![];

        while !stack.is_empty() || p.is_some() {
            if let Some(node) = p {
                ans.push(node.val);
                stack.push(node);
                p = node.left.as_ref();
            } else if let Some(node) = stack.pop() {
                p = node.right.as_ref();
            }
        }

        ans
    }

    // 中序遍历 迭代版
    fn inorder_traversal(&self) -> Vec<i32> {
        let mut stack = vec![];
        let mut p = self.root.as_ref();
        let mut ans = vec![];

        while p.is_some() || !stack.is_empty() {
            while let Some(node) = p {
                stack.push(node);
                p = node.left.as_ref();
            }
            if let Some(node) = stack.pop() {
                ans.push(node.val);
                p = node.right.as_ref();
            }
        }

        ans
    }

    // 中序遍历 递归版
    fn inorder_traversal_recursive(&self) -> Vec<i32> {
        let mut ans = vec![];
        fn inorder_traversal_recursive_helper(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder_traversal_recursive_helper(&node.left, ans);
                ans.push(node.val);
                inorder_traversal_recursive_helper(&node.right, ans);
            }
        }
        inorder_traversal_recursive_helper(&self.root, &mut ans);
        ans
    }

    // 后序遍历 迭代版
    fn postorder_traversal(&self) -> Vec<i32> {
        let mut ans = vec![];
        let root = self.root.as_ref().unwrap();
        let mut stack1 = vec![root];
        let mut stack2 = vec![];

        while !stack1.is_empty() {
            let node = stack1.pop().unwrap();
            stack2.push(node.val);
            if node.left.is_some() {
                stack1.push(node.left.as_ref().unwrap());
            }
            if node.right.is_some() {
                stack1.push(node.right.as_ref().unwrap());
            }
        }

        while !stack2.is_empty() {
            ans.push(stack2.pop().unwrap());
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tree = Tree::new();
        tree.root = Some(Box::new(TreeNode::new(1)));
        let root = tree.root.as_mut().unwrap();
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        let left = root.left.as_mut().unwrap();
        left.left = Some(Box::new(TreeNode::new(4)));
        left.right = Some(Box::new(TreeNode::new(5)));
        let right = root.right.as_mut().unwrap();
        right.left = Some(Box::new(TreeNode::new(6)));
        right.right = Some(Box::new(TreeNode::new(7)));
        let ans = tree.inorder_traversal();
        assert_eq!(ans, vec![4, 2, 5, 1, 6, 3, 7]);
        let ans_recursive = tree.inorder_traversal_recursive();
        assert_eq!(ans_recursive, ans);
        let ans = tree.preorder_traversal();
        assert_eq!(ans, vec![1, 2, 4, 5, 3, 6, 7]);
        let ans = tree.postorder_traversal();
        assert_eq!(ans, vec![4, 5, 2, 6, 7, 3, 1]);
    }
}
