use tokio::net::TcpListener;

// TODO:编写一个回显服务器，它接受传入的 TCP 连接并将接收到的数据回显给客户端。
//  `echo` 完成连接处理后不应返回，但应继续接受新连接。
// Hint: 您应该依赖“tokio”的结构和方法来实现回显服务器。
// 尤其：
// - `tokio::net::TcpListener::accept` 处理下一个传入连接
// - `tokio::net::TcpStream::split` 从套接字获取读取器和写入器
// - `tokio::io::copy` 将数据从读取器复制到写入器
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
