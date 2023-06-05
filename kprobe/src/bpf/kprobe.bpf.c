

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

struct softirq_key
{
    u32 pid;
    u32 cpu;
};

struct
{
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 102400);
    __uint(key_size, sizeof(struct softirq_key));
    __uint(value_size, sizeof(u64));
} softirq_map SEC(".maps");

struct
{
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1024);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
} hardirq_map SEC(".maps");

// skb_recv_done
static void __always_inline push_event(void *ctx, int type)
{
    struct event event = {};
    event.ts = bpf_ktime_get_ns();
    event.type = type;
    bpf_get_current_comm(event.comm, sizeof(event.comm));
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
}

// // hardware irq
// SEC("kprobe/skb_recv_done")
// int BPF_KPROBE(skb_recv_done, int x)
// {
//     push_event(ctx, IRQ);
//     return 0;
// }

// // software irq
// SEC("kprobe/net_rx_action")
// int BPF_KPROBE(net_rx_action, int x)
// {
//     push_event(ctx, SOFTIRQ);
//     return 0;
// }

// struct netif_receive_skb_args
// {
//     u32 pad[2];
//     struct sk_buff *skb;
// };
// SEC("tracepoint/net/netif_receive_skb")
// int tp_netif_receive_skb(struct netif_receive_skb_args *args)
// {
//     u16 network_header, transport_header, source;
//     char *head;
//     struct iphdr ih = {};
//     struct tcphdr th = {};
//     struct sk_buff *skb = args->skb;

//     bpf_probe_read(&head, sizeof(head), &skb->head);
//     bpf_probe_read(&network_header, sizeof(network_header), &skb->network_header);
//     if (network_header != 0)
//     {
//         bpf_probe_read(&ih, sizeof(ih), head + network_header);
//         if (ih.protocol == IPPROTO_TCP)
//         {
//             transport_header = network_header + (ih.ihl << 2);
//             bpf_probe_read(&th, sizeof(th), head + transport_header);
//             source = bpf_ntohs(th.source);

//             if (source == 2031)
//             {
//                 push_event(args, SKB);
//             }
//         }
//     }
//     return 0;
// }

struct napi_gro_receive_entry_args
{
    u32 pad[2];
    u32 pad2[4];
    struct sk_buff *skb;
};

SEC("tracepoint/net/napi_gro_receive_entry")
int tp_napi_gro_receive_entry(struct napi_gro_receive_entry_args *args)
{
    u16 mac_header, network_header, transport_header, source, protocol;
    char *head;
    struct ethhdr eh = {};
    struct iphdr ih = {};
    struct tcphdr th = {};
    struct sk_buff *skb = args->skb;
    bpf_probe_read(&head, sizeof(head), &skb->head);
    bpf_probe_read(&mac_header, sizeof(mac_header), &skb->mac_header);

    if (mac_header != 0)
    {
        bpf_probe_read(&eh, sizeof(eh), head + mac_header);
        protocol = bpf_ntohs(eh.h_proto);
        if (protocol == 0x0800)
        {
            network_header = mac_header + 14;
            bpf_probe_read(&ih, sizeof(ih), head + network_header);
            if (ih.protocol == IPPROTO_TCP)
            {
                transport_header = network_header + (ih.ihl << 2);
                bpf_probe_read(&th, sizeof(th), head + transport_header);
                source = bpf_ntohs(th.source);

                if (source == 2031)
                {
                    struct softirq_key key = {};
                    key.cpu = bpf_get_smp_processor_id();
                    key.pid = bpf_get_current_pid_tgid();
                    u64 *res = bpf_map_lookup_elem(&softirq_map, &key);
                    struct event event = {};
                    event.type = ERROR;
                    if (res)
                    {
                        event.ts = bpf_ktime_get_ns();
                        event.softirq_ts = *res;
                        event.type = SKB;
                    }
                    res = bpf_map_lookup_elem(&hardirq_map, &key.cpu);
                    if (res)
                        event.hardirq_ts = *res;
                    event.sip = ih.saddr;
                    event.dip = ih.daddr;
                    bpf_get_current_comm(event.comm, sizeof(event.comm));
                    bpf_perf_event_output(args, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
                }
            }
        }
    }
    return 0;
}

struct softirq_entry_args
{
    u32 pad[2];
    u32 vec_nr;
};

SEC("tracepoint/irq/softirq_entry")
int tp_softirq_entry(struct softirq_entry_args *args)
{
    if (args->vec_nr == NET_RX_SOFTIRQ)
    {
        u64 ts = bpf_ktime_get_ns();
        struct softirq_key key = {};
        key.cpu = bpf_get_smp_processor_id();
        key.pid = bpf_get_current_pid_tgid();
        bpf_map_update_elem(&softirq_map, &key, &ts, BPF_ANY);
    }
    return 0;
}

SEC("tracepoint/irq/softirq_exit")
int tp_softirq_exit(struct softirq_entry_args *args)
{
    if (args->vec_nr == NET_RX_SOFTIRQ)
    {
        struct softirq_key key = {};
        key.cpu = bpf_get_smp_processor_id();
        key.pid = bpf_get_current_pid_tgid();
        bpf_map_delete_elem(&softirq_map, &key);
    }
    return 0;
}

SEC("kprobe/skb_recv_done")
int BPF_KPROBE(skb_recv_done)
{
    int cpu = bpf_get_smp_processor_id();

    u64 *ts = bpf_map_lookup_elem(&hardirq_map, &cpu);
    if (ts)
        *ts = bpf_ktime_get_ns();
    return 0;
}
char _license[] SEC("license") = "GPL";
