#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main()
{
    uint16_t hex;
    int32_t type, flag = 0, exp = 0;
    printf("Please input a hex: ");
    scanf("%hx", &hex);
    printf("Please choose the output type(1:integer ,2:unsigned integer ,3:float): ");
    scanf("%d", &type);
    printf("Binary of %hx is: ", hex);
    printf("%u", (hex / 32768) % 2);
    printf("%u", (hex / 16384) % 2);
    printf("%u", (hex / 8192) % 2);
    printf("%u ", (hex / 4096) % 2);
    printf("%u", (hex / 2048) % 2);
    printf("%u", (hex / 1024) % 2);
    printf("%u", (hex / 512) % 2);
    printf("%u ", (hex / 256) % 2);
    printf("%u", (hex / 128) % 2);
    printf("%u", (hex / 64) % 2);
    printf("%u", (hex / 32) % 2);
    printf("%u ", (hex / 16) % 2);
    printf("%u", (hex / 8) % 2);
    printf("%u", (hex / 4) % 2);
    printf("%u", (hex / 2) % 2);
    printf("%u\n", hex % 2);
    if (type == 1)
    {
        int16_t sign = (int16_t)hex;
        printf("Converted integer is: %d\n", sign);
    }
    else if (type == 2)
    {
        uint16_t usign = hex;
        printf("Converted usigned integer is: %u\n", usign);
    }
    else if (type == 3)
    {
        float f = 1;
        if ((hex / 32768) % 2 == 1)
            flag = 1;
        exp += ((hex / 16384) % 2) * 16;
        exp += ((hex / 8192) % 2) * 8;
        exp += ((hex / 4096) % 2) * 4;
        exp += ((hex / 2048) % 2) * 2;
        exp += (hex / 1024) % 2;
        exp -= 15;
        f += (float)((hex / 512) % 2) / 2;
        f += (float)((hex / 256) % 2) / 4;
        f += (float)((hex / 128) % 2) / 8;
        f += (float)((hex / 64) % 2) / 16;
        f += (float)((hex / 32) % 2) / 32;
        f += (float)((hex / 16) % 2) / 64;
        f += (float)((hex / 8) % 2) / 128;
        f += (float)((hex / 4) % 2) / 256;
        f += (float)((hex / 2) % 2) / 512;
        f += (float)(hex % 2) / 1024;
        if (flag == 1)
            f = f * (-1);
        printf("Converted float is: %f*2^%d\n", f, exp);
    }
    else
        printf("Wrong output type.\n");
    return 0;
}