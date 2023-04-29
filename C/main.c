#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
  char line[6];
  int maxCalories[4] = {0};
  int currentCalories = 0;

  FILE *file = fopen("elves.txt", "r");

  while (fgets(line, 6, file) != NULL)
  {

    if (line[0] != '\n')
    {
      currentCalories += atoi(line);
    }
    else
    {
      if (maxCalories[0] < currentCalories)
      {
        maxCalories[2] = maxCalories[1];
        maxCalories[1] = maxCalories[0];
        maxCalories[0] = currentCalories;
      }
      else if (maxCalories[1] < currentCalories)
      {
        maxCalories[2] = maxCalories[1];
        maxCalories[1] = currentCalories;
      }
      else if (maxCalories[2] < currentCalories)
      {
        maxCalories[2] = currentCalories;
      }

      currentCalories = 0;

      continue;
    }
  }

  if (maxCalories[0] < currentCalories)
  {
    maxCalories[2] = maxCalories[1];
    maxCalories[1] = maxCalories[0];
    maxCalories[0] = currentCalories;
  }
  else if (maxCalories[1] < currentCalories)
  {
    maxCalories[2] = maxCalories[1];
    maxCalories[1] = currentCalories;
  }
  else if (maxCalories[2] < currentCalories)
  {
    maxCalories[2] = currentCalories;
  }
  currentCalories = 0;

  for (int i = 0; i < 3; i++)
  {
    printf("%d\n", maxCalories[i]);
    currentCalories += maxCalories[i];
  }

  printf("%d\n", currentCalories);

  fclose(file);

  return 0;
}