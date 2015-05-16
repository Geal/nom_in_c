#include "include/embed.h"

#include <stdio.h>
#include <string.h>

int main() {
  printf("return: %d\n", test());

  char * a = "omkthxbye";
  char * b = "omnomkthxbye";
  char * c = "omnomnomnomkthxbye";
  char * d = "pouet";

  printf("counting noms:\n");
  printf("%d\t->\t%s\n", count_noms(a, strlen(a)), a);
  printf("%d\t->\t%s\n", count_noms(b, strlen(b)), b);
  printf("%d\t->\t%s\n", count_noms(c, strlen(c)), c);
  printf("%d\t->\t%s\n", count_noms(d, strlen(d)), d);
  return 0;
}
