#include "pthread_impl.h"

void __wait(volatile int *addr, volatile int *waiters, int val, int priv)
{
	int spins=100;
	while (spins-- && (!waiters || !*waiters)) {
		if (*addr==val) a_spin();
		else return;
	}
	if (waiters) a_inc(waiters);
	while (*addr==val) {
		/* No futex or equivalent syscall on SVSM. */
		a_spin();
	}
	if (waiters) a_dec(waiters);
}
