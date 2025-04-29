#include "pthread_impl.h"

int __pthread_mutex_unlock(pthread_mutex_t *m)
{
	pthread_t self;
	int waiters = m->_m_waiters;
	int cont;
	int type = m->_m_type & 15;
	int priv = (m->_m_type & 128) ^ 128;
	int new = 0;
	int old;

	if (type != PTHREAD_MUTEX_NORMAL) {
		return ENOSYS;
	}
	if (type&8) {
		return ENOSYS;
	} else {
		cont = a_swap(&m->_m_lock, new);
	}
	if (type != PTHREAD_MUTEX_NORMAL && !priv) {
		return ENOSYS;
	}
	if (waiters || cont<0)
		__wake(&m->_m_lock, 1, priv);
	return 0;
}

weak_alias(__pthread_mutex_unlock, pthread_mutex_unlock);
