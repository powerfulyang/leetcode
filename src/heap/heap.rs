// heap is a complete binary tree
// parent(i) = i/2
// left(i) = 2i
// right(i) = 2i + 1
// min-heap: parent <= left && parent <= right
// max-heap: parent >= left && parent >= right
// heapify: O(logN) log default base 2

// MinHeap struct，它有一个 Vec<i32> 类型的字段，它是堆的内部存储
pub struct MinHeap {
    data: Vec<i32>,
}

#[allow(dead_code)]
impl MinHeap {
    // 新建一个空的 MinHeap
    pub fn new() -> Self {
        MinHeap { data: vec![] }
    }

    // 在堆中插入一个元素，然后进行上浮操作保持堆的性质。
    // 我们将新元素添加到数组的末尾以维持完全二叉树的特性，然后进行上浮操作以维持堆的特性。
    pub fn push(&mut self, item: i32) {
        self.data.push(item);
        self.bubble_up(self.data.len() - 1);
    }

    // 从堆中弹出最小元素。如果堆为空，返回 None。
    // 否则，我们将堆的最后一个元素交换到堆顶（这保持了完全二叉树的特性），然后进行下沉操作以维持堆的特性。
    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let min = self.data[0];
        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            self.bubble_down(0);
        }
        Some(min)
    }

    // 上浮操作。从当前节点开始，如果当前节点的值小于其父节点的值，则交换这两个节点。
    // 然后递归地对父节点进行上浮操作，直到当前节点的值不再小于其父节点，或者已经到达根节点。
    // 这个操作用于维护插入新元素后的堆的性质。
    fn bubble_up(&mut self, idx: usize) {
        if idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[parent] > self.data[idx] {
                self.data.swap(parent, idx);
                self.bubble_up(parent);
            }
        }
    }

    // 下沉操作。从当前节点开始，如果当前节点的值大于其任一子节点的值，则交换这两个节点。
    // 然后递归地对新的子节点进行下沉操作，直到当前节点的值不再大于其子节点，或者已经到达叶子节点。
    // 这个操作用于维护移除最小元素后的堆的性质。
    fn bubble_down(&mut self, idx: usize) {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut min_idx = idx;

        // 找到当前节点，左子节点和右子节点中最小的那个
        if left < self.data.len() && self.data[left] < self.data[min_idx] {
            min_idx = left;
        }
        if right < self.data.len() && self.data[right] < self.data[min_idx] {
            min_idx = right;
        }

        // 如果当前节点不是最小的，那么它就需要下沉
        if min_idx != idx {
            self.data.swap(min_idx, idx);
            self.bubble_down(min_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap(){
        let mut heap = MinHeap::new();
        heap.push(1);
        heap.push(4);
        heap.push(5);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }
}
