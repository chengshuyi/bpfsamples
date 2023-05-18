

#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "iter.h"


struct
{
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
} events SEC(".maps");

SEC("iter/task_file")
int dump_task_file(struct bpf_iter__task_file *ctx)
{
    struct event event = {0};
    struct task_struct *task = ctx->task;
    if (task)
    {
        event.pid = task->mm->pgd->pgd;
        bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    }

    return 0;
}

char _license[] SEC("license") = "GPL";