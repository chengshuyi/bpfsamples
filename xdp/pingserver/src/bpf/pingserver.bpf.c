#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "pingserver.h"

#define ETH_P_IP 0x0800  /* Internet Protocol packet	*/
#define ICMP_ECHOREPLY 0 /* Echo Reply			*/
#define ICMP_ECHO 8      /* Echo Request			*/

static __always_inline __u16 csum_fold_helper(__u32 csum)
{
        __u32 sum;
        sum = (csum >> 16) + (csum & 0xffff);
        sum += (sum >> 16);
        return ~sum;
}

SEC("xdp")
int ping_server(struct xdp_md *ctx)
{
        void *data_end = (void *)(long)ctx->data_end;
        void *data = (void *)(long)ctx->data;
        struct ethhdr *eth;
        struct iphdr *iph;
        struct icmphdr *ich;
        struct icmphdr ich_old;

        eth = data;
        if (data + sizeof(struct ethhdr) <= data_end && eth->h_proto == bpf_htons(ETH_P_IP))
        {
                data = data + sizeof(struct ethhdr);
                iph = data;
                if (data + sizeof(struct iphdr) <= data_end && iph->protocol == IPPROTO_ICMP)
                {
                        data = data + iph->ihl * 4;
                        ich = data;
                        if (data + sizeof(struct icmphdr) <= data_end && ich->type == ICMP_ECHO)
                        {
                                u16 old_csum;
                                old_csum = ich->checksum;
                                ich->checksum = 0;
                                ich_old = *ich;
                                ich->type = ICMP_ECHOREPLY;

                                u32 csum = bpf_csum_diff((__be32 *)&ich_old, sizeof(struct icmphdr), (__be32 *)ich, sizeof(struct icmphdr), ~old_csum);
                                ich->checksum = csum_fold_helper(csum);
                                return XDP_TX;
                        }
                }
        }
        return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
