    // 这是一个 'main.rs' 文件，因此 'cargo' 将其解释为二进制目标的根。

// TODO: 修复这个损坏的导入。在 'src' 目录中创建一个新的库目标。
//   库目标应公开一个名为“你好世界”的公共函数，该函数不接受任何参数，也不返回任何内容。
//
use packages::hello_world;

// This is the entrypoint of the binary.
fn main() {
    hello_world();
}
