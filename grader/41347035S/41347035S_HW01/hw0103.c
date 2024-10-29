#include<stdio.h>
#include<stdint.h>

int main()
{
	uint16_t n=0;
	printf("Please enter an unsigned 16-bits number: ");
	scanf("%hu", &n);
	
	printf("Before Flip:\n%hu_10 = %ho_8\n", n, n);

	int32_t a=n/32768; //8^5
	int32_t b=(n%32768)/4096; //8^4	
	int32_t c=(n%32768%4096)/512; //8^3
	int32_t d=(n%32768%4096%512)/64; //8^2
	int32_t e=(n%32768%4096%512%64)/8; //8^1
	int32_t f=(n%32768%4096%512%64%8);
	int32_t r; //reverse_8
	
	if(n>=32768)
	{
		r=f*100000+e*10000+d*1000+c*100+b*10+a;
	}
	else if(n>=4096)
	{
		r=f*10000+e*1000+d*100+c*10+b;
	}
	else if(n>=512)
	{
		r=f*1000+e*100+d*10+c;
	}
	else if(n>=64)
	{
		r=f*100+e*10+d;
	}
	else if(n>=8)
	{
		r=f*10+e;
	}
 	else
	{
		r=f;
	}
	
	int32_t fr= r/100000%10;
	int32_t er= r/10000%10;
	int32_t dr= r/1000%10;
	int32_t cr= r/100%10;
	int32_t br= r/10%10;
	int32_t ar= r/1%10;

	int32_t t; //_10
	if(r>=100000)
	{
		t=fr*32768+er*4096+dr*512+cr*64+br*8+ar;
	}
	else if(r>=10000)
	{
		t=er*4096+dr*512+cr*64+br*8+ar;
	}
	else if(r>=1000)
	{
		t=dr*512+cr*64+br*8+ar;
	}
	else if(r>=100)
	{
		t=cr*64+br*8+ar;
	}
	else if(r>=10)
	{
		t=br*8+ar;
	}
	else
	{
		t=ar;
	}
	//printf("%d, %d, %d, %d, %d, %d\n", f, e, d, c, b, a);
	//printf("%d, %d, %d, %d, %d, %d\n", fr, er, dr, cr, br, ar);
	
	printf("After Flip:\n%d_8 = %d_10\n", r,t);			
	
	return 0;

}
