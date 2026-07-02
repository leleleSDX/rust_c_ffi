#include <stdio.h>
#include <stdlib.h>
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

void modify_num(int *num)
{
	printf("C> modifying %d\n", *num);
	*num = *num + 1;
}

char* modify_name(char *name)
{
	printf("C> modifying %s\n", name);
	int i = 0;
	while (*name != '\0')
	{
		*name = *name - 2;
		name++;
		i++;
	}
	name = name - i;
	printf("C> %s is modified\n", name);
	return name;
}
void shorten_str(char *str)
{
	printf("C> modifying %s\n", str);
	*str = '\0';
	printf("C> %s is modified\n", str);
}
void free_str(char *str)
{
	printf("C> dropping %s \n", str);
	free(str);
}

void display_str(const char *str)
{
	printf("%s\n", str);
}
