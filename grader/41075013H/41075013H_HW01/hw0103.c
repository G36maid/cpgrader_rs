#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main()
{
    uint16_t num;
    printf("Please enter an unsigned 16-bits number:");
    scanf("%hd", &num);
    if (num > (uint16_t)65536)
    {
        printf("輸入錯誤。超出範圍。\n");
        return 0;
    }
    int32_t before = 0, flip = 0, n = 0;
    printf("Before Flip:\n");
    printf("%hd_10 = ", num);
    if (num >= 32768)
    {
        before += 100000 * (num / 32768);
        num = num % 32768;
    }
    if (num >= 4096)
    {
        before += 10000 * (num / 4096);
        num = num % 4096;
    }
    if (num >= 512)
    {
        before += 1000 * (num / 512);
        num = num % 512;
    }
    if (num >= 64)
    {
        before += 100 * (num / 64);
        num = num % 64;
    }
    if (num >= 8)
    {
        before += 10 * (num / 8);
        num = num % 8;
    }
    if (num > 0)
        before += num;
    printf("%d_8\n", before);
    if ((before / 10 != 0) || (before % 10 != 0))
    {
        flip = (flip * 10) + (before % 10);
        n = (n * 8) + (before % 10);
        before /= 10;
    }
    if ((before / 10 != 0) || (before % 10 != 0))
    {
        flip = (flip * 10) + (before % 10);
        n = (n * 8) + (before % 10);
        before /= 10;
    }
    if ((before / 10 != 0) || (before % 10 != 0))
    {
        flip = (flip * 10) + (before % 10);
        n = (n * 8) + (before % 10);
        before /= 10;
    }
    if ((before / 10 != 0) || (before % 10 != 0))
    {
        flip = (flip * 10) + (before % 10);
        n = (n * 8) + (before % 10);
        before /= 10;
    }
    if ((before / 10 != 0) || (before % 10 != 0))
    {
        flip = (flip * 10) + (before % 10);
        n = (n * 8) + (before % 10);
        before /= 10;
    }
    printf("After Flip:\n");
    printf("%d_8 = %d_10\n", flip, n);
    return 0;
}