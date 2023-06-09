#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "denyicmp.h"

#define ETH_P_IP 0x0800 /* Internet Protocol packet	*/

SEC("xdp")
int xdp_deny_icmp(struct xdp_md *ctx)
{
        void *data_end = (void *)(long)ctx->data_end;
        void *data = (void *)(long)ctx->data;
        struct ethhdr *eth;
        struct iphdr *iph;

        eth = data;
        if (data + sizeof(struct ethhdr) <= data_end && eth->h_proto != bpf_htons(ETH_P_IP))
                goto out;

        iph = data + sizeof(struct ethhdr);
        if (data + sizeof(struct ethhdr) + sizeof(struct iphdr) <= data_end && iph->protocol != IPPROTO_ICMP)
                goto out;

        return XDP_DROP;
out:
        return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
