#include <stdio.h>
#include "../include/hello.h"

void hello(void)
{
	printf("hello from c 2x\n");
	return;
}

void hello_name(const char *name)
{
	printf("hello %s from C\n", name);
	return;
}

int give_back(int num)
{
	printf("C> got %d\n", num);
	return num + 1;
}