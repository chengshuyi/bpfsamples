

#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "kprobe.h"

struct
{
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
} events SEC(".maps");

// skb_recv_done
static void __always_inline push_event(void *ctx, int type)
{
    struct event event = {};
    event.ts = bpf_ktime_get_boot_ns();
    event.type = type;
    bpf_get_current_comm(event.comm, sizeof(event.comm));
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
}

// hardware irq
SEC("kprobe/skb_recv_done")
int BPF_KPROBE(skb_recv_done, int x)
{
    push_event(ctx, IRQ);
    return 0;
}

// software irq
SEC("kprobe/net_rx_action")
int BPF_KPROBE(net_rx_action, int x)
{
    push_event(ctx, SOFTIRQ);
    return 0;
}

struct napi_gro_receive_entry_args
{
    u32 pad[2];
    struct sk_buff *skb;
};
SEC("tracepoint/net/netif_receive_skb")
int tp_netif_receive_skb(struct napi_gro_receive_entry_args *args)
{
    u16 network_header, transport_header, source;
    char *head;
    struct iphdr ih = {};
    struct tcphdr th = {};
    struct sk_buff *skb = args->skb;

    bpf_probe_read(&head, sizeof(head), &skb->head);
    bpf_probe_read(&network_header, sizeof(network_header), &skb->network_header);
    if (network_header != 0)
    {
        bpf_probe_read(&ih, sizeof(ih), head + network_header);
        if (ih.protocol == IPPROTO_TCP)
        {
            transport_header = network_header + (ih.ihl << 2);
            bpf_probe_read(&th, sizeof(th), head + transport_header);
            source = bpf_ntohs(th.source);

            if (source == 2031)
            {
                push_event(args, SKB);
            }
        }
    }
    return 0;
}
char _license[] SEC("license") = "GPL";
