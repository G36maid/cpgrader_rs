#include<stdio.h>
#include<stdint.h>

int main()
{
	uint16_t h;
	int32_t n, r;
	printf("Please input a hex:");
	r=scanf("%hx", &h);
	printf("Please choose the output type(1:integer ,2:unsigned integer ,3:float):");
	scanf("%d", &n);
	
	if(r != 1 || n < 1 || n>3)
	{
		printf("Error\n");
	}
	else
	{
	//hex --> decimal
	uint16_t h1=h/4096;
	uint16_t h2=(h%4096)/256;
	uint16_t h3=(h%4096%256)/16;
	uint16_t h4=(h%4096%256%16);
	//decimal --> binary
	uint16_t b1=(h1/8), b2=(h1%8)/4, b3=(h1%8%4)/2, b4=(h1%8%4%2);
	uint16_t b5=(h2/8), b6=(h2%8)/4, b7=(h2%8%4)/2, b8=(h2%8%4%2);
	uint16_t b9=(h3/8), b10=(h3%8)/4, b11=(h3%8%4)/2, b12=(h3%8%4%2);
	uint16_t b13=(h4/8), b14=(h4%8)/4, b15=(h4%8%4)/2, b16=(h4%8%4%2);
	printf("Binary of %hx is: %hu%hu%hu%hu %hu%hu%hu%hu %hu%hu%hu%hu %hu%hu%hu%hu\n", h, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16);
	uint16_t b=b1*32768+b2*16384+b3*8192+b4*4096+b5*2048+b6*1024+b7*512+b8*256+b9*128+b10*64+b11*32+b12*16+b13*8+b14*4+b15*2+b16;

	if(n==1)
	{
		printf("Converted integer is: %hd\n ", b);
	}
	else if(n==2)
	{
		printf("Converted unsigned integer is: %hu\n", b);
	}
	else if(n==3)
	{
		int32_t exp=b2*16+b3*8+b4*4+b5*2+b6;
		float f=b7*(1.0/2.0)+b8*(1.0/4.0)+b9*(1.0/8.0)+b10*(1.0/16.0)+b11*(1.0/32.0)+b12*(1.0/64.0)+b13*(1.0/128.0)+b14*(1.0/256.0)+b15*(1.0/512.0)+b16*(1.0/1024.0);
	
		if(exp == 0 && f == 0 && b1 == 0)//+0.0
		{
			printf("Convered float is: +0.0\n");
		}
		else if(exp == 0 && f == 0 && b1 == 1)//-0/0
		{
			printf("Converted float is: -0.0\n");
		}
		else if(exp == 31 && f == 0 && b1 == 0)//+inf
		{
			printf("Converted float is: +Inf\n");
		}
		else if(exp == 31 && f == 0 && b1 ==1)//-inf
		{
			printf("Converted float is: -Inf\n");
		}
		else if(exp == 31 && f != 0)//nan
		{
			printf("Converted float is: Nan\n");
		}
		else if(b1==0)
		{
			printf("Converted float is: %f*2^%d\n", f+1, exp-15);
		}
		else if(b1==1)
		{
			printf("Converted float is: -%f*2^%d\n", f+1, exp-15);
		}


	}		
	}
	return 0;
}
	
