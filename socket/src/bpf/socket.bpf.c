#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "socket.h"



SEC("socket")
int socket_filter_example(struct __sk_buff *skb)
{

    struct tcphdr tcph;
    bpf_skb_load_bytes(skb, 0, &tcph, sizeof(tcph));

    bpf_printk("skb len is %d, tcphdr len %d\n", skb->len, tcph.doff * 4);
    if (tcph.doff * 4 + sizeof("hello world") == skb->len) {
        // we got "hello world"
        return skb->len - sizeof(" world");
    }
    return skb->len;
}


char _license[] SEC("license") = "GPL";
