


## 01 BPF_PROG_TYPE_SOCKET_FILTER

演示socket filter截断报文的功能。


运行流程如下：

- `cargo run -p socket` 启动该程序，该程序做了：
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

# 06-BPF_PROG_TYPE_XDP

denycimp: 丢掉lo口的所有icmp包。

## 13-BPF_PROG_TYPE_SOCK_OPS

```shell
cargo run -p sockops
cat /sys/kernel/debug/tracing/trace_pipe
```

## 14-BPF_PROG_TYPE_SK_SKB

```
client -> proxy:5201 -> backend
完整的路径是：
client -> proxy:5201 -> proxy -> backend: 5202
也就是客户端将报文先发给代理，代理收到报文后再发送给backend
```

```
cargo run -p sk_skb 
```
可以看到如下输出

```
client send: i'm client
proxy server receive: i'm client
proxy client send: i'm client
backend server receive: i'm client
client send: i'm client
proxy server receive: i'm client
proxy client send: i'm client
backend server receive: i'm client
insert client-to-proxy socketfd[7] into sockets map with key 0
insert client socketfd[4] into sockets map with key 1
client send: i'm client
backend server receive: i'm client
client send: i'm client
backend server receive: i'm client
client send: i'm client
backend server receive: i'm client
client send: i'm client
backend server receive: i'm client
```
发现，proxy server和proxy client已经开始收不到报文了，因为全在内核态进行转发。

## 16-BPF_PROG_TYPE_SK_MSG

```shell
cargo run -p sk_msg
tcpdump -i lo port 5201
```

## BPF_PROG_TYPE_SK_LOOKUP

可以将指定报文理由给指定socket，即使该socket没有监听该端口。

运行流程如下：

- `cargo run -p sk_lookup` 启动该程序，该程序做了：
  - 监听5201端口
  - 加载sk_lookup程序，劫持5202端口的报文

- `nc -4 127.0.0.1 5202` 发起连接请求，可以发现链接成功建立



