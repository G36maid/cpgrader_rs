#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main()
{
    int32_t c1, c2, c3, c4, c5;
    int32_t f1, f2, f3, f4, f5;
    int32_t temp;
    printf("Please enter 5 cards: ");
    scanf("%d %d %d %d %d", &c1, &c2, &c3, &c4, &c5);
    if ((c1 < 1 || c1 > 52) || (c2 < 1 || c2 > 52) || (c3 < 1 || c3 > 52) || (c4 < 1 || c4 > 52) || (c5 < 1 || c5 > 52))
    {
        printf("輸入錯誤。卡片編號超出範圍。\n");
        return 0;
    }
    else if (c1 == c2 || c2 == c3 || c3 == c4 || c4 == c5)
    {
        printf("輸入錯誤。有卡片重複。\n");
        return 0;
    }
    f1 = (c1 - 1) / 13;
    f2 = (c2 - 1) / 13;
    f3 = (c3 - 1) / 13;
    f4 = (c4 - 1) / 13;
    f5 = (c5 - 1) / 13;
    c1 = c1 % 13;
    if (c1 == 0)
        c1 = 13;
    c2 = c2 % 13;
    if (c2 == 0)
        c2 = 13;
    c3 = c3 % 13;
    if (c3 == 0)
        c3 = 13;
    c4 = c4 % 13;
    if (c4 == 0)
        c4 = 13;
    c5 = c5 % 13;
    if (c5 == 0)
        c5 = 13;

    if (c1 > c2)
    {
        temp = c1;
        c1 = c2;
        c2 = temp;
        temp = f1;
        f1 = f2;
        f2 = temp;
    }
    if (c2 > c3)
    {
        temp = c2;
        c2 = c3;
        c3 = temp;
        temp = f2;
        f2 = f3;
        f3 = temp;
    }
    if (c3 > c4)
    {
        temp = c3;
        c3 = c4;
        c4 = temp;
        temp = f3;
        f3 = f4;
        f4 = temp;
    }
    if (c4 > c5)
    {
        temp = c4;
        c4 = c5;
        c5 = temp;
        temp = f4;
        f4 = f5;
        f5 = temp;
    }
    if (c1 > c2)
    {
        temp = c1;
        c1 = c2;
        c2 = temp;
        temp = f1;
        f1 = f2;
        f2 = temp;
    }
    if (c2 > c3)
    {
        temp = c2;
        c2 = c3;
        c3 = temp;
        temp = f2;
        f2 = f3;
        f3 = temp;
    }
    if (c3 > c4)
    {
        temp = c3;
        c3 = c4;
        c4 = temp;
        temp = f3;
        f3 = f4;
        f4 = temp;
    }
    if (c1 > c2)
    {
        temp = c1;
        c1 = c2;
        c2 = temp;
        temp = f1;
        f1 = f2;
        f2 = temp;
    }
    if (c2 > c3)
    {
        temp = c2;
        c2 = c3;
        c3 = temp;
        temp = f2;
        f2 = f3;
        f3 = temp;
    }
    if (c1 > c2)
    {
        temp = c1;
        c1 = c2;
        c2 = temp;
        temp = f1;
        f1 = f2;
        f2 = temp;
    }

    if ((f1 == f2) && (f2 == f3) && (f3 == f4) && (f4 == f5))
    {
        if (c2 == (c1 + 1) && c3 == (c2 + 1) && c4 == (c3 + 1) && c5 == (c4 + 1))
            printf("Straight Flush\n");
        else if (c1 == 1 && c3 == (c2 + 1) && c4 == (c3 + 1) && c5 == (c4 + 1) && c5 == 13)
            printf("Straight Flush\n");
        else
            printf("Flush\n");
    }
    else if ((c1 == c2 && c2 == c3 && c3 == c4) || (c2 == c3 && c3 == c4 && c4 == c5))
        printf("Four of a Kind\n");
    else if ((c1 == c2 && c2 == c3 && c4 == c5) || (c1 == c2 && c3 == c4 && c4 == c5))
        printf("Full house\n");
    else if (c2 == (c1 + 1) && c3 == (c2 + 1) && c4 == (c3 + 1) && c5 == (c4 + 1))
        printf("Straight\n");
    else if (c1 == 1 && c3 == (c2 + 1) && c4 == (c3 + 1) && c5 == (c4 + 1) && c5 == 13)
        printf("Straight\n");
    else if ((c1 == c2 && c2 == c3) || (c2 == c3 && c3 == c4) || (c3 == c4 && c4 == c5))
        printf("Three of a kind\n");
    else if ((c1 == c2 && c3 == c4) || (c1 == c2 && c4 == c5) || (c2 == c3 && c4 == c5))
        printf("Two Pairs\n");
    else if (c1 == c2 || c2 == c3 || c3 == c4 || c4 == c5)
        printf("One Pair\n");
    else
        printf("High card\n");
    return 0;
}