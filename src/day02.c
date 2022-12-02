#include <stdio.h>
#include <string.h>

#ifdef TEST_INPUT
char input[] = "A Y\n"
               "B X\n"
               "C Z\n";
#endif
#include "shared/input.c"

size_t calculatePoints(size_t opponentMove, size_t myMove) {
  // rock 1 paper 2 scissors 3
  // 1 1 - tie 0
  // 1 2 - win -1
  // 1 3 - lose -2
  // 2 2 - tie 0
  // 2 3 - win -1
  // 2 1 - lose 1
  // 3 3 - tie 0
  // 3 1 - win 2
  // 3 2 - lose 1
  size_t winValue = 0;

  switch(opponentMove - myMove) {
    case 0:
      winValue = 3;
      break;
    case -1:
    case 2:
      winValue = 6;
      break;
  }

  return winValue + myMove;
}

size_t decideMove(size_t opponentMove, size_t myMove) {
  ssize_t modValue = ((ssize_t)opponentMove - 1 + (ssize_t)myMove - 2) % 3;
  if (modValue < 0) modValue = 3 + modValue;
  return modValue + 1;
}

int main(void) {
  char *buffer;
  size_t opponentMoveValue,
         myMoveValue,
         decidedMove,
         points = 0,
         part2Points = 0;
  char *opponentMove, *myMove;
  char *sep = " \n";

  if(getInput(&buffer, "day02") <= 0) {
    fprintf(stderr, "Error: failed to read file\n");
    return 1;
  };

  opponentMove = strtok(buffer, sep);
  myMove = strtok(NULL, sep);

  if (!opponentMove || !myMove) {
    fprintf(stderr, "Error: invalid file format");
    return 1;
  }

  while (opponentMove && myMove) {
    opponentMoveValue = *opponentMove - 'A' + 1;
    myMoveValue = *myMove - 'X' + 1;

    points += calculatePoints(opponentMoveValue, myMoveValue);

    decidedMove = decideMove(opponentMoveValue, myMoveValue);
    part2Points += calculatePoints(opponentMoveValue, decidedMove);

    opponentMove = strtok(NULL, sep);
    myMove = strtok(NULL, sep);
  }

  printf("Part 1: %zu\n", points);
  printf("Part 2: %zu\n", part2Points);
  return 0;
}
