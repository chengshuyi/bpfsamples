#ifndef KPROBE_H
#define KPROBE_H

enum
{
    IRQ,
    SOFTIRQ,
    SKB,
};

struct event
{
    unsigned long long ts;
    unsigned char comm[16];
    unsigned short type;
};

#endif
