// TODO: 将必要的 trait bounds 添加到 'min' 中，以便成功编译。
//   请参阅“std：：cmp”模块的文档，以了解有关您可能需要的特征的更多信息。
//
// Note: 有不同的 trait bounds 会让编译器满意，但它们带有不同的 _semantics_。
//  我们将在本课程后面讨论有序集合（例如 BTreeMap）时介绍这些差异。
//

/// 返回两个值的最小值。
pub fn min<T>(left: T, right: T) -> T
where
    T: Ord,
{
    std::cmp::min(left, right)
}
