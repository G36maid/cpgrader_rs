#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main()
{
    int32_t a, b, c;
    int32_t sum;
    int32_t x = 0, y = 0, z = 0, carry = 0;
    char temp, xc, yc, zc;
    printf("Please enter the first operand\t:");
    scanf("%d", &a);
    if (a >= 10)
    {
        printf("輸入錯誤。\n");
        return 0;
    }
    scanf("%c", &xc);
    if (xc != 'x')
    {
        printf("輸入錯誤。此位置應輸入x\n");
        return 0;
    }
    scanf("%d", &b);
    if (b >= 10)
    {
        printf("輸入錯誤。\n");
        return 0;
    }
    scanf("%c", &temp);
    printf("Please enter the second operand\t:");
    scanf("%c", &yc);
    if (yc != 'y')
    {
        printf("輸入錯誤。此位置應輸入y\n");
        return 0;
    }
    scanf("%d", &c);
    if (c >= 10)
    {
        printf("輸入錯誤。\n");
        return 0;
    }
    scanf("%c", &zc);
    if (zc != 'z')
    {
        printf("輸入錯誤。此位置應輸入z\n");
        return 0;
    }
    scanf("%c", &temp);
    printf("Please enter the sum\t\t:");
    scanf("%d", &sum);
    int32_t u = sum % 10;
    int32_t t = (sum % 100) / 10;
    int32_t h = sum / 100;
    if (u < b)
    {
        z = 10 - b + u;
        t--;
    }
    else
        z = u - b;

    if (t < c)
    {
        x = 10 - c + t;
        h--;
    }
    else
        x = t - c;
    y = h - a;
    printf("Ans: x = %d, y = %d, z = %d\n", x, y, z);
    return 0;
}
