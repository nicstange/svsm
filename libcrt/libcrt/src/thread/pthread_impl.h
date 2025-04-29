#ifndef _PTHREAD_IMPL_H
#define _PTHREAD_IMPL_H

#include <libcrt.h>
#include <pthread/pthread.h>
#include "atomic_arch-x86_64.h"

#define hidden __attribute__((__visibility__("hidden")))

#define _m_type __u.__i[0]
#define _m_lock __u.__vi[1]
#define _m_waiters __u.__vi[2]
#define _m_prev __u.__p[3]
#define _m_next __u.__p[4]
#define _m_count __u.__i[5]
#define _c_shared __u.__p[0]
#define _c_seq __u.__vi[2]
#define _c_waiters __u.__vi[3]
#define _c_clock __u.__i[4]
#define _c_lock __u.__vi[8]
#define _c_head __u.__p[1]
#define _c_tail __u.__p[5]
#define _rw_lock __u.__vi[0]
#define _rw_waiters __u.__vi[1]
#define _rw_shared __u.__i[2]
#define _b_lock __u.__vi[0]
#define _b_waiters __u.__vi[1]
#define _b_limit __u.__i[2]
#define _b_count __u.__vi[3]
#define _b_waiters2 __u.__vi[4]
#define _b_inst __u.__p[3]

hidden void __do_cleanup_push(struct __ptcb *);
hidden void __do_cleanup_pop(struct __ptcb *);

hidden int __timedwait(volatile int *, int, clockid_t, const struct timespec *, int);
hidden int __timedwait_cp(volatile int *, int, clockid_t, const struct timespec *, int);
hidden void __wait(volatile int *, volatile int *, int, int);
static inline void __wake(volatile void *addr, int cnt, int priv)
{
	/* No FUTEX_WAKE, the __wait has been reimplemented to spin forever. */
}

hidden int __pthread_mutex_trylock(pthread_mutex_t *m);
hidden int __pthread_mutex_timedlock(pthread_mutex_t *restrict m, const struct timespec *restrict at);


hidden int __pthread_rwlock_rdlock(pthread_rwlock_t *);
hidden int __pthread_rwlock_tryrdlock(pthread_rwlock_t *);
hidden int __pthread_rwlock_timedrdlock(pthread_rwlock_t *__restrict, const struct timespec *__restrict);
hidden int __pthread_rwlock_wrlock(pthread_rwlock_t *);
hidden int __pthread_rwlock_trywrlock(pthread_rwlock_t *);
hidden int __pthread_rwlock_timedwrlock(pthread_rwlock_t *__restrict, const struct timespec *__restrict);
hidden int __pthread_rwlock_unlock(pthread_rwlock_t *);

#endif
