#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "sk_msg.h"

struct
{
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
} sockets SEC(".maps");

SEC("sk_msg")
int sk_msg_example(struct sk_msg_md *msg)
{
    int verdict = bpf_msg_redirect_map(msg, &sockets, 0, BPF_F_INGRESS);
    return verdict;
}

char _license[] SEC("license") = "GPL";
