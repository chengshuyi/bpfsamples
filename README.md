





## BPF_PROG_TYPE_SK_LOOKUP

可以将指定报文理由给指定socket，即使该socket没有监听该端口。

运行流程如下：

- `cargo run -p sk_lookup` 启动该程序，该程序做了：
  - 监听5201端口
  - 加载sk_lookup程序，劫持5202端口的报文

- `nc -4 127.0.0.1 5202` 发起连接请求，可以发现链接成功建立



