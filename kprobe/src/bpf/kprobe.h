#ifndef KPROBE_H
#define KPROBE_H

enum
{
    IRQ,
    SOFTIRQ,
    SKB,
    ERROR,
};

struct event
{
    unsigned long long softirq_ts;
    unsigned long long ts;
    unsigned char comm[16];
    unsigned short type;
};

#endif
