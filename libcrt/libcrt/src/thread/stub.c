#include "pthread_impl.h"

int pthread_key_create(pthread_key_t *, void (*)(void *))
{
	return ENOSYS;
}

int pthread_key_delete(pthread_key_t)
{
	return ENOSYS;
}
void *pthread_getspecific(pthread_key_t)
{
	return 0;
}

int pthread_setspecific(pthread_key_t, const void *)
{
	return ENOSYS;
}
