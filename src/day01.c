#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#ifdef TEST_INPUT
char input[] = "1000\n\
2000\n\
3000\n\
\n\
4000\n\
\n\
5000\n\
6000\n\
\n\
7000\n\
8000\n\
9000\n\
\n\
10000\n";

ssize_t getInput(char **buffer) {
  ssize_t len = strlen(input);
  *buffer = input;
  return len;
}
#else

ssize_t getInput(char **buffer) {
  ssize_t len;
  FILE *fd = fopen("inputs/day01input.txt", "r");
  if(!fd) return -1;

  fseek(fd, 0, SEEK_END);
  len = ftell(fd);
  fseek(fd, 0, SEEK_SET);
  *buffer = malloc(sizeof(char) * len + 1);

  fread(*buffer, sizeof(char), len, fd);
  fclose(fd);
  (*buffer)[len] = '\0';
  return len;
}
#endif

int strToInt(char *buffer, int *value) {
  static char *localBuff = NULL;
  char *endPtr;
  if(!localBuff) localBuff = strdup(buffer);
  if(localBuff[0] == '\n' && localBuff[1] == '\n') {
    localBuff += 2;
    printf("blank\n");
    return 1;
  };
  *value = (int)strtol(localBuff, &endPtr, 10);
  printf("%d\n", *value);

  if(errno) return(-1);

  localBuff = endPtr;
  return 0;
}

int main(void) {
  char *buffer = NULL;
  int value;
  int conversionResult;
  int result = 0;
  int tmpResult = 0;

  if(getInput(&buffer) <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  while ((conversionResult = strToInt(buffer, &value)) != -1) {
    if(conversionResult) {
      if (tmpResult > result) result = tmpResult;
      printf("result: %d\n", result);
      tmpResult = 0;
    }
    else
      tmpResult += value;
    printf("tmp result: %d\n", tmpResult);
  };

  printf("Part 1: %d\n", result);
  return 0;
}
