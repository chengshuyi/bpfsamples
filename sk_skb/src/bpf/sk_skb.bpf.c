

#include "vmlinux.h"
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "sk_skb.h"

struct
{
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
} sockmap SEC(".maps");

SEC("sk_skb/stream_verdict")
int stream_verdict(struct __sk_buff *skb)
{
    // redirect to egress
    return bpf_sk_redirect_map(skb, &sockmap, 1, 0);
}

SEC("sk_skb/stream_parser")
int stream_parser(struct __sk_buff *skb)
{
    return skb->len;
}

char _license[] SEC("license") = "GPL";