

#ifndef TRACING_FETNRY_H
#define TRACING_FETNRY_H

struct event
{
    unsigned int saddr;
    unsigned int daddr;
    unsigned short sport;
    unsigned short dport;
    long unsigned int size;
};

#endif