#ifdef TEST_INPUT
#include <string.h>

ssize_t getInput(char **buffer, char *fileBase) {
  (void)fileBase;
  ssize_t len = strlen(input);
  *buffer = strdup(input);
  return len;
}
#else
#include <stdlib.h>

ssize_t getInput(char **buffer, char *fileBase) {
  ssize_t len;
  char filepath[255];
  snprintf(filepath, sizeof(filepath), "inputs/%sinput.txt", fileBase);

  FILE *fd = fopen(filepath, "r");
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
