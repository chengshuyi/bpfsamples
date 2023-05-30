#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "sockops.h"

SEC("sockops")
int sockops_example(struct bpf_sock_ops *skops)
{
    switch (skops->op)
    {
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
        bpf_printk("active established: %d -> %d\n", skops->local_port, bpf_ntohl(skops->remote_port));
        break;

    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
        bpf_printk("passive established: %d -> %d\n", skops->local_port, bpf_ntohl(skops->remote_port));
        break;

    default:
        break;
    }
    return 0;
}
char _license[] SEC("license") = "GPL";