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

void splitRucksack(char *rucksack, char **firstCompartment, char **secondCompartment) {
  size_t len = strlen(rucksack);
  *firstCompartment = strndup(rucksack, len / 2);
  *secondCompartment = strdup(rucksack + len / 2);
}

char getIntersection(char *firstCompartment, char *secondCompartment) {
  size_t len = strlen(firstCompartment);
  char firstSparse['z' + 1] = {0},
       secondSparse['z' + 1] = {0};

  for(size_t i = 0; i < len; ++i) {
    firstSparse[(size_t)firstCompartment[i]] = 1;
    secondSparse[(size_t)secondCompartment[i]] = 1;
  }

  for(char i = 'A'; i <= 'z'; ++i) {
    if(firstSparse[(size_t)i] && secondSparse[(size_t)i]) return i;
  }

  return 0;
}

size_t charToPriority(char item) {
  if(item <= 'Z') {
    return item - 'A' + 27;
  } else {
    return item - 'a' + 1;
  }
}

int main(void) {
  size_t total = 0;
  char intersection,
       *buffer,
       *rucksack,
       *firstCompartment,
       *secondCompartment,
       *sep = "\n";


  if(getInput(&buffer, "day03") <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  rucksack = strtok(buffer, sep);
  while(rucksack) {
    splitRucksack(rucksack, &firstCompartment, &secondCompartment);
    intersection = getIntersection(firstCompartment, secondCompartment);
    total += charToPriority(intersection);

    rucksack = strtok(NULL, sep);
  }


  printf("Part 1: %zu\n", total);

  if (buffer) free(buffer);
  if (firstCompartment) free(firstCompartment);
  if (secondCompartment) free(secondCompartment);
  return 0;
}
