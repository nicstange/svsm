#include "pthread_impl.h"

int __pthread_mutex_trylock(pthread_mutex_t *m)
{
	if ((m->_m_type&15) == PTHREAD_MUTEX_NORMAL)
		return a_cas(&m->_m_lock, 0, EBUSY) & EBUSY;
	return ENOSYS;
}

weak_alias(__pthread_mutex_trylock, pthread_mutex_trylock);
