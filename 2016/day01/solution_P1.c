#include <stdio.h>
#include <stdlib.h>

int main()
{
    int x = 0;
    int y = 0;
    int facing = 0;

    FILE * fp;
    fp = fopen("input.txt", "rb");
    fseek(fp, 0, SEEK_END);
    long fsize = ftell(fp);
    fseek(fp, 0, SEEK_SET);

	char * line = malloc(fsize+1);

    fread(line, fsize, 1, fp);
    line[fsize] = 0;
    fclose(fp);

    char *tok = strtok(line, ", ");
    while (tok != NULL) {
        char dir = tok[0];
        int len = atoi(tok+1);
        if (dir == 'R') {
            facing++;
        } else {
            facing --;
        }
        if (facing > 3) {
            facing = 0;
        } else if (facing < 0) {
            facing = 3;
        }
        if (facing == 0) {
            y = y + len;
        } else if (facing == 1) {
            x = x + len;
        } else if (facing == 2) {
            y = y - len;
        } else if (facing == 3) {
            x = x - len;
        }
        tok = strtok(NULL, ", ");
    }
    printf("%d", x+y);
    return 0;
} 