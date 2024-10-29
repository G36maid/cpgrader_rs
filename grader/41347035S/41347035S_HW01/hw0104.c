#include<stdio.h>
#include<stdint.h>

int main()
{
	int32_t a=0, b=0, c=0, d=0, e=0, temp=0;
	
	printf("Please enter the 5 cards: ");
	scanf("%d %d %d %d %d", &a, &b, &c, &d, &e);

	if(a<=0||a>52||b<=0||b>52||c<=0||c>52||d<=0||d>52||e<=0||e>52)
	{
		printf("Error\n");
	}
	else
	{	//ranking high to low
		if(a%13<b%13)
		{
			temp=a;
			a=b;
			b=temp;
		}
		if(a%13<c%13)
		{
			temp=a;
			a=c;
			c=temp;
		}
		if(a%13<d%13)
		{
			temp=a;
			a=d;
			d=temp;
		}
		if(a%13<e%13)
		{
			temp=a;
			a=e;
			e=temp;
		}
		if(b%13<c%13)
		{
			temp=b;
			b=c;
			c=temp;
		}
		if(b%13<d%13)
		{
			temp=b;
			b=d;
			d=temp;
		}
		if(b%13<e%13)
		{
			temp=b;
			b=e;
			e=temp;
		}
		if(c%13<d%13)
		{
			temp=c;
			c=d;
			d=temp;
		}
		if(c%13<e%13)
		{
			temp=c;
			c=e;
			e=temp;
		}
		if(d%13<e%13)
		{
			temp=d;
			d=e;
			e=temp;
		}
		//printf("%d %d %d %d %d\n", a, b, c, d, e);

		//ranting category
		//Straight flush
		if((a-1)/13 == (b-1)/13 && (b-1)/13 == (c-1)/13 && (c-1)/13 == (d-1)/13 && (d-1)/13 == (e-1)/13 && a==b+1 && b==c+1 && c==d+1 && d==e+1)
		{
			printf("Straight flush\n");
		}
		else if((a-1)/13 == (b-1)/13 && (b-1)/13 == (c-1)/13 && (c-1)/13 == (d-1)/13 && (d-1)/13 == (e-1)/13 && a%13==12 && b%13==11 && c%13==10 && d%13==1 && e%13==0)
		{
			printf("Straight flush\n");
		}
		else if((a-1)/13 == (b-1)/13 && (b-1)/13 == (c-1)/13 && (c-1)/13 == (d-1)/13 && (d-1)/13 == (e-1)/13 && a%13==12 && b%13==11 && c%13==10 && d%13==9 && e%13==0)
		{
			printf("Straight flush\n");
		}
		//Four of kind
		else if(a%13 == b%13 && b%13 == c%13 && c%13 == d%13)
		{
			printf("Four of a kind\n");
		}
		else if(b%13 == c%13 && c%13 == d%13 && d%13 == e%13)
		{
			printf("Four of a kind\n");
		}
		//Full house
		else if(a%13 == b%13 && b%13 == c%13 && d%13 == e%13)
		{
			printf("Full house\n");
		}
		else if(a%13 == b%13 && c%13 == d%13 && d%13 == e%13)
		{
			printf("Full house\n");
		}
		//Flush
		else if((a-1)/13 == (b-1)/13 && (b-1)/13 == (c-1)/13 && (c-1)/13 == (d-1)/13 && (d-1)/13 == (e-1)/13)
		{
			printf("Flush\n");
		}
		//Straight
		else if(a%13 == b%13+1 && b%13 == c%13+1 && c%13 == d%13+1 && d%13 == e%13+1)
		{
			printf("Straight\n");
		}
		else if(e%13 == 0 && d%13 == 1 && a%13 == 12 && b%13 == 11 && c%13 == 10)
		{
			printf("Straight\n");
		}
		else if(a%13 == 12 && b%13 == 11 && c%13 == 10 && d%13 == 9 && e%13 == 0)
		{
			printf("Straight\n");
		}
		//Three of a kind
		else if(a%13 == b%13 && b%13 == c%13)
		{
			printf("Three of a kind\n");
		}
		else if(b%13 == c%13 && c%13 == d%13)
		{
			printf("Three of a kind\n");
		}
		else if(c%13 == d%13 && d%13 == e%13)
		{
			printf("Three of a kind\n");
		}
		//Two pair
		else if(a%13 == b%13 && c%13 == d%13)
		{
			printf("Two pair\n");
		}
		else if(a%13 == b%13 && d%13 == e%13)
		{
			printf("Two pair\n");
		}
		else if(d%13 == e%13 && b%13 == c%13)
		{
			printf("Two pair\n");
		}
		//One pair
		else if(a%13 == b%13)
		{
			printf("One pair\n");
		}
		else if(b%13 == c%13)
		{
			printf("One pair\n");
		}
		else if(c%13 == d%13)
		{
			printf("One pair\n");
		}
		else if(d%13 == e%13)
		{
			printf("One pair\n");
		}
		//High card
		else 
		{
			printf("High card\n");
		}
	}
	return 0;
}
