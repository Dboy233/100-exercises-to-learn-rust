#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // 达到最大容量
        assert_eq!(v.capacity(), 2);

        v.push(3); // 超出容量，需要调整大小

        // 您能猜出新的容量是多少吗？
        // 请注意，标准库不保证用于调整向量大小的算法，因此将来可能会更改。

        assert_eq!(v.capacity(), 4);
        v.push(4);
        assert_eq!(v.capacity(), 4);
        v.push(5);
        assert_eq!(v.capacity(), 8);
    }
}
