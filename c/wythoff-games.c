#include <math.h>
#include <stdio.h>

#define MAX(a, b) a > b ? a : b
#define MIN(a, b) a < b ? a : b

int main() {
    long double golden_ratio = (1 + sqrtl(5)) / 2;
    long double n1, n2;
    scanf("%Lf %Lf", &n1, &n2);
    long double small = MIN(n1, n2);
    long double big = MAX(n1, n2);
    if (small != floorl((big - small) * golden_ratio)) {
        printf("1\n");
    } else {
        printf("0\n");
    }
    return 0;
}