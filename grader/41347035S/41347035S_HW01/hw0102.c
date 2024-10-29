#include<stdio.h>
#include<stdint.h>

int main()
{
	int32_t a=10, b=10, c=10, sum=0;
	int32_t x=10, y=0, z=0;

	printf("Please enter the first  operand: ");
	scanf("%dx%d", &a, &b);
	printf("Please enter the second operand: ");
	scanf(" y%dz", &c);
	printf("Please enter the sum           : ");
	scanf(" %d", &sum);
	
	
	if(sum<a*100+c*10+b||sum>a*100+c*10+b+999||a<0||a>9||b<0||b>9||c<0||c>9)
	{
		printf("Error\n");
	}
	else
	{
		
		int32_t d1=sum/100%10;
		int32_t d2=sum/10%10;
		int32_t d3=sum/1%10;
		if(d3>=b)
		{
			z=d3-b;
		}
		else if(d3<b)
		{
			z=d3+10-b;
			d2-=1;
		}
		if(d2>=c)
		{
			x=d2-c;
		}
		else if(d2<c)
		{
			x=d2+10-c;
			d1-=1;
		}
		if(d1>=a)
		{
			y=d1-a;
		}
		else if(d1<a)
		{
			y=d1+10-a;
		}
		
		printf("Ans: x = %d, y = %d, z = %d\n", x, y, z);
		

	}
	return 0;
}
	
