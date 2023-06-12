#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "cgroup_skb.h"

SEC("cgroup_skb/ingress")
int cgroup_skb_ingress(struct __sk_buff *ctx)
{
        // drop all icmp packets
        if (ctx->protocol == IPPROTO_ICMP)
                return 0;
        return 1;
}


char _license[] SEC("license") = "GPL";
