#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>

#ifdef TEST_INPUT
char input[] = "2-4,6-8\n"
               "2-3,4-5\n"
               "5-7,7-9\n"
               "2-8,3-7\n"
               "6-6,4-6\n"
               "2-6,4-8\n";
#endif
#include "shared/input.c"

typedef struct Range {
  size_t start, end;
} Range;

void initRange(Range* range, size_t start, size_t end) {
  range->start = start;
  range->end = end;
}

bool rangeContainedIn(Range* range, Range* other) {
  return range->start >= other->start && range->end <= other->end;
}

bool rangesOverlap(Range* range1, Range* range2) {
  return (range1->start <= range2->end && range1->end >= range2->start);
}

ssize_t parseRange(char* buffer, Range* elf) {
  size_t start, end;
  char *startStr, *endStr, *sep = "-,\n";
  static bool initialized = false;

  if(!initialized) {
    startStr = strtok(buffer, sep);
    initialized = true;
  } else {
    startStr = strtok(NULL, sep);
  }
  endStr = strtok(NULL, sep);
  if(!startStr || !endStr) return 0;

  start = (size_t)strtol(startStr, NULL, 10);
  if(errno) return -1;
  end = (size_t)strtol(endStr, NULL, 10);
  if(errno) return -1;

  elf->start = start;
  elf->end = end;
  return 1;
}

int main(void) {
  Range elf1, elf2;
  char *buffer;
  size_t part1Counter = 0,
         part2Counter = 0;
  ssize_t parseResult;

  if(getInput(&buffer, "day04") <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  while(true) {
    parseResult = parseRange(buffer, &elf1);
    if (parseResult < 0) {
      fprintf(stderr, "Error: failed to parse range\n");
      return 1;
    }

    parseResult = parseRange(buffer, &elf2);
    if (parseResult < 0) {
      fprintf(stderr, "Error: failed to parse range\n");
      return 1;
    } else if (parseResult == 0) break;

    if(rangeContainedIn(&elf1, &elf2) || rangeContainedIn(&elf2, &elf1))
      part1Counter++;
    if(rangesOverlap(&elf1, &elf2))
      part2Counter++;
  }

  printf("Part 1: %zu\n", part1Counter);
  printf("Part 2: %zu\n", part2Counter);

  if(buffer) free(buffer);
}
