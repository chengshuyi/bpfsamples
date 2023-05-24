


## 01 BPF_PROG_TYPE_SOCKET_FILTER

演示socket filter截取报文的功能。


运行流程如下：

- `cargo run -p sk_lookup` 启动该程序，该程序做了：
  - 监听5201端口
  - 加载socket filter程序，截断收到的"hello world"

- `nc -4 127.0.0.1 5201` 发起连接请求

- 输入`hello world`

- 会间断的回复`hello`，然后再回复` world`，这里主要是因为是tcp协议，会进行重传

```shell
18:24:07.186661 IP 127.0.0.1.33176 > 127.0.0.1.5201: Flags [P.], seq 637392362:637392374, ack 3775574401, win 128, options [nop,nop,TS val 593094786 ecr 592952575], length 12
18:24:07.186725 IP 127.0.0.1.5201 > 127.0.0.1.33176: Flags [P.], seq 1:6, ack 5, win 128, options [nop,nop,TS val 593094786 ecr 593094786], length 5
18:24:07.186736 IP 127.0.0.1.33176 > 127.0.0.1.5201: Flags [.], ack 6, win 128, options [nop,nop,TS val 593094786 ecr 593094786], length 0
18:24:07.391230 IP 127.0.0.1.33176 > 127.0.0.1.5201: Flags [P.], seq 5:12, ack 6, win 128, options [nop,nop,TS val 593094991 ecr 593094786], length 7
18:24:07.391301 IP 127.0.0.1.5201 > 127.0.0.1.33176: Flags [P.], seq 6:13, ack 12, win 128, options [nop,nop,TS val 593094991 ecr 593094991], length 7
18:24:07.391314 IP 127.0.0.1.33176 > 127.0.0.1.5201: Flags [.], ack 13, win 128, options [nop,nop,TS val 593094991 ecr 593094991], length 0
```



## BPF_PROG_TYPE_SK_LOOKUP

可以将指定报文理由给指定socket，即使该socket没有监听该端口。

运行流程如下：

- `cargo run -p sk_lookup` 启动该程序，该程序做了：
  - 监听5201端口
  - 加载sk_lookup程序，劫持5202端口的报文

- `nc -4 127.0.0.1 5202` 发起连接请求，可以发现链接成功建立



