#include "pthread_impl.h"

int __timedwait_cp(volatile int *addr, int val,
	clockid_t clk, const struct timespec *at, int priv)
{
	__wait(addr, 0, val, priv);
	return 0;
}

int __timedwait(volatile int *addr, int val,
	clockid_t clk, const struct timespec *at, int priv)
{
	int r;
	r = __timedwait_cp(addr, val, clk, at, priv);
	return r;
}
