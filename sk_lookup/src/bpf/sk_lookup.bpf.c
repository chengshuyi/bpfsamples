#include "vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_endian.h>

#include "sk_lookup.h"


struct
{
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10240);
    __type(key, u16);
    __type(value, u8);
} ports SEC(".maps");

struct
{
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
} sockets SEC(".maps");


// https://github.com/jsitnicki/ebpf-summit-2020
SEC("sk_lookup")
int sk_lookup_example(struct bpf_sk_lookup *ctx)
{

    const __u32 zero = 0;
	struct bpf_sock *sk;
	__u16 port;
	__u8 *open;
	long err;

	port = ctx->local_port;
	bpf_printk("port is %u\n", port);

	open = bpf_map_lookup_elem(&ports, &port);
	if (!open)
		return SK_PASS;

	sk = bpf_map_lookup_elem(&sockets, &zero);
	if (!sk)
		return SK_DROP;

    // set skb->sk
	err = bpf_sk_assign(ctx, sk, 0);
	bpf_sk_release(sk);
	return err ? SK_DROP : SK_PASS;
}


char _license[] SEC("license") = "GPL";
