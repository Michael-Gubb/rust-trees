use std::collections::VecDeque;

#[derive(Default, Debug)]
struct TreeNode {
    name: String,
    left_child: Option<Box<TreeNode>>,
    right_child: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(name: String) -> Self {
        TreeNode {
            name,
            left_child: None,
            right_child: None,
        }
    }
    fn new_left_child(&mut self, child_name: String) {
        self.left_child = Some(Box::new(TreeNode::new(child_name)));
    }
    fn new_right_child(&mut self, child_name: String) {
        self.right_child = Some(Box::new(TreeNode::new(child_name)));
    }
    fn dfs(&self) {
        let mut node_stack: Vec<&TreeNode> = Vec::new();
        node_stack.push(self);
        while let Some(popped_node) = node_stack.pop() {
            println!("{}", popped_node.name);
            popped_node
                .right_child
                .as_ref()
                .inspect(|n| node_stack.push(n));
            popped_node
                .left_child
                .as_ref()
                .inspect(|n| node_stack.push(n));
        }
    }
    fn bfs(&self) {
        let mut node_queue: VecDeque<&TreeNode> = VecDeque::new();
        node_queue.push_front(self);
        while let Some(popped_node) = node_queue.pop_back() {
            println!("{}", popped_node.name);
            popped_node
                .left_child
                .as_ref()
                .inspect(|n| node_queue.push_front(n));
            popped_node
                .right_child
                .as_ref()
                .inspect(|n| node_queue.push_front(n));
        }
    }
    fn bf_iter(&self) -> TreeNodeBFIterator {
        let node_queue: VecDeque<&TreeNode> = VecDeque::from([self]);
        TreeNodeBFIterator { node_queue }
    }
    fn preorder_iter(&self) -> TreeNodePreorderIterator {
        let node_stack: Vec<&TreeNode> = vec![self];
        TreeNodePreorderIterator { node_stack }
    }
}

struct TreeNodeBFIterator<'a> {
    node_queue: VecDeque<&'a TreeNode>,
}

impl<'it> Iterator for TreeNodeBFIterator<'it> {
    type Item = &'it TreeNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.node_queue.pop_back().inspect(|node| {
            node.left_child
                .as_ref()
                .inspect(|left_child| self.node_queue.push_front(left_child));
            node.right_child
                .as_ref()
                .inspect(|right_child| self.node_queue.push_front(right_child));
        })
    }
}

struct TreeNodePreorderIterator<'iter> {
    node_stack: Vec<&'iter TreeNode>,
}

impl<'iter> Iterator for TreeNodePreorderIterator<'iter> {
    type Item = &'iter TreeNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.node_stack.pop().inspect(|node| {
            node.right_child
                .as_ref()
                .inspect(|right_child| self.node_stack.push(right_child));
            node.left_child
                .as_ref()
                .inspect(|left_child| self.node_stack.push(left_child));
        })
    }
}

fn main() {
    let mut root = TreeNode::new("Root".to_owned());
    root.new_left_child("l".to_owned());
    root.new_right_child("r".to_owned());
    root.left_child
        .as_mut()
        .unwrap()
        .new_left_child("ll".to_owned());
    root.left_child
        .as_mut()
        .unwrap()
        .new_right_child("lr".to_owned());
    println!("Preorder iterator:");
    for node in root.preorder_iter() {
        println!("{}", node.name);
    }
    println!("Preorder iterator from l:");
    root.left_child.as_ref().inspect(|lc| {
        for n in lc.preorder_iter() {
            println!("{}", n.name);
        }
    });
    for node in root.bf_iter() {
        println!("{}", node.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
