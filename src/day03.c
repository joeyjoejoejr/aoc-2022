#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef TEST_INPUT
char input[] = "vJrwpWtwJgWrhcsFMMfFFhFp\n"
               "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n"
               "PmmdzqPrVvPwwTWBwg\n"
               "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n"
               "ttgJtRGJQctTZtZT\n"
               "CrZsJsPPZsGzwwsLwLmpwMDw";
#endif
#include "shared/input.c"

void splitRucksack(char *rucksack, char *firstCompartment, char *secondCompartment) {
  size_t len = strlen(rucksack);
  char *end = stpncpy(firstCompartment, rucksack, len / 2);
  *end = '\0';
  strcpy(secondCompartment, rucksack + len / 2);
}

void getIntersection(char *intersection, char *str1, char *str2) {
  size_t counter = 0,
         firstLen = strlen(str1),
         secondLen = strlen(str2);
  char firstSparse['z' + 1] = {0},
       secondSparse['z' + 1] = {0};

  for(size_t i = 0; i < firstLen; ++i)
    firstSparse[(size_t)str1[i]] = 1;
  for(size_t i = 0; i < secondLen; ++i)
    secondSparse[(size_t)str2[i]] = 1;

  for(char i = 'A'; i <= 'z'; ++i) {
    if(firstSparse[(size_t)i] && secondSparse[(size_t)i]) {
      intersection[counter++] = i;
    }
  }

  intersection[counter] = '\0';
}

size_t charToPriority(char item) {
  if(item <= 'Z') {
    return item - 'A' + 27;
  } else {
    return item - 'a' + 1;
  }
}

int main(void) {
  size_t counter = 0,
         total = 0,
         part2Total = 0;
  char *buffer,
       *rucksack,
       *sep = "\n";
  char intersection[256],
       firstCompartment[256],
       secondCompartment[256];
  char group[3][256];


  if(getInput(&buffer, "day03") <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  rucksack = strtok(buffer, sep);
  while(rucksack) {
    strcpy(group[counter % 3], rucksack);
    if (counter % 3 == 2) {
      getIntersection(intersection, group[0], group[1]);
      getIntersection(intersection, intersection, group[2]);
      part2Total += charToPriority(intersection[0]);
    }

    splitRucksack(rucksack, firstCompartment, secondCompartment);
    getIntersection(intersection, firstCompartment, secondCompartment);
    total += charToPriority(intersection[0]);

    rucksack = strtok(NULL, sep);
    counter++;
  }


  printf("Part 1: %zu\n", total);
  printf("Part 2: %zu\n", part2Total);

  if (buffer) free(buffer);
  return 0;
}
