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
    return 1;
  };
  *value = (int)strtol(localBuff, &endPtr, 10);

  if(errno) return(-1);

  localBuff = endPtr;
  return 0;
}

void insertResult(int value, int topThree[3]) {
  int currentValue = value;
  int tmp;

  for(size_t i = 0; i < 3; ++i) {
    if(currentValue > topThree[i]) {
      tmp = currentValue;
      currentValue = topThree[i];
      topThree[i] = tmp;
    }
  }
}

int main(void) {
  char *buffer = NULL;
  int value;
  int conversionResult;
  int tmpResult = 0;
  int topThree[3] = { 0, 0, 0};

  if(getInput(&buffer) <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  while ((conversionResult = strToInt(buffer, &value)) != -1) {
    if(conversionResult) {
      insertResult(tmpResult, topThree);
      tmpResult = 0;
    }
    else
      tmpResult += value;
  };

  printf("Part 1: %d\n", topThree[0]);
  printf("Part 2: %d\n", topThree[0] + topThree[1] + topThree[2]);
  return 0;
}
