# zuo-algo

跟着左程云学算法

## 二进制

[code](./examples/1-bit.rs)

### 二进制表示数字

每一位（bit）表示`0`和`1`，计算一个二进制数的十进制表示就是从低位*2的0次方累加的到结果

比如`1001`，计算十进制数为`1*2^0+0*2^1+0*2^2+1*2^3=9`

### 二进制表示非负数和负数

一个n位的二进制数表示的无符号数字范围是`2^n`个

一个n位的二进制数表示的有符号数字范围是`-2^(n-1)~2^(n-1)-1`。每个数字的首位表示符号，0为正、1为负

一个正数的二进制数的负数如何计算？

将所有位减1再取反。`5`的二进制表示`0101`，它的负数计算过程为

1. `0101` -> `0100`
2. `1010` -> `1011`

`1001`就是`-5`的二进制表示，相反，根据一个二进制数来判断它是负几，则将整个数取反再加1

1. `1011` -> `0100`
2. `0100` -> `0101`

由于最后的`0101`是5，所以`1011`表示`-5`

### 二进制对正负数的表示原因

这种表示方式可以在负数和非负数以任何出现顺序做加法的同时不需要加分支判断即可进行，例如`-5+2`

1. `1011+0010`
2. `1101`
3. `1101` -> `0010`
4. `0010` -> `0011`

`0100`是3，结果正确

> **计算机不处理数字计算的溢出情况，需要程序员自己保证**

由于没有分支判断，加法的计算速度很快，这样会导致其它的计算`-`/`*`/`/`的计算性能提高，因为这些计算都是基于加法实现的

### 位计算

位计算没有短路计算方式，逻辑计算存在短路计算

#### `|`

当存在一个位为`1`时，计算结果为`1`，否则为`0`

#### `&`

当存在至少一个位为`0`时，计算结果为`0`，否则为`1`

#### `^`

当两个位相同时为`0`，否则为`1`

#### `<</>>/<<</>>>`

带符号左移/右移，无符号左移/右移（即右移用0补位）

对于**非负数**，左移n位等同于`*2^(n)`，右移n位等同于`/2^(n)`

### 取反

`~`是取反符号，每个数的正反都等于对它取反再加1

### 打印二进制数

要打印二进制的某一位，可以将其它位与`0`做`&`操作，要打印的位与`1`做`&`操作，如果剩下的数不为0，那么打印`1`，否则打印`0`

## 三种简单排序

[code](./examples/2-simple-sort.rs)

⚠️注意边界条件，数组元素如果是`0`或`1`个，那么本身有序无需进算法步骤

### 选择排序

一句话总结：将i～n-1位置上的最小值找到，然后与i位置的数交换位置，直到`i==n-1`

### 冒泡排序

一句话总结：在0~n-1位置范围内，通过两两比较将最大值移到n-1位置上，然后缩短范围0～n-2继续

### 插入排序

一句话总计：每次都保证前i位有序，将i+1位认为无序，然后和前i位倒序比较，如果无序则交换顺序

## 对数器

[code](./examples/3-test-method.rs)

对数器是一种验证算法的方法，它的实现步骤大概为

1. 对于一个要解决的问题A
2. 实现一个不考虑复杂度的暴力解法S1，最简单的思路基本都是正确的，要保证这里正确
3. 实现几个考虑复杂度的优雅解法S2、S3...
4. 开始创建测试用例，即利用随机数生成测试数据
5. 使用生成的测试数据对几个解法（方法）进行测试，小数据量测试
6. 如果小数据量没有问题，那么逐渐增加数据，直到暴力解法达到基本无法计算的程度，如果还没有问题则基本确定后续的最优解是正确的
7. 如果过程中出现了问题，那么减少数据量，打印/debug方法，开始找出错误，这个过程可以通过打印原始测试数据，对比不同方法的结果来判断

## 二分搜索

[code](./examples/4-binary-search.rs)

### 有序数列

1. 在有序数列中找到一个指定数字，如果存在返回它的索引，否则返回`-1`
2. 找到一个大于数字`n`的最靠左的数字
3. 找到一个小于数字`n`的最靠右的数字

解决方法：一个长度为n的数列的开头总是`0`，最后一个位置是`n-1`，那么中点是`0+(n-1)/2`，另外一种计算中点的方式是`0+((n-1-0)>>1)`，这样写的作用是防止两个索引位置直接相加之后会超过索引值类型的最大值造成溢出

### 无序数列

峰值问题

前提，一个数列中，峰值的含义是，在位置i上，它的值大于左右两边的值，且相邻数不允许相同，认为在数列左端和右端的数的值为最小

找到数列中任意一个峰值的位置，如果没有则返回`-1`

按照题中的指示，先判断第一个数和最后一个数是否为峰值，如果两个数都不是峰值，那么可以判断峰值一定在中间，此时认为二分搜索的起点为`1`，终点为`n-2`，判断中点是否为峰值，如果不是，如果小于左边的数，那么终点变为左移一位的数继续，否则起点变成右移一位的数继续

启发：如果可以确定问题范围缩小的规律，就可以使用“二分”解决同类问题

### 时间复杂度

二分搜索的时间复杂度是`O(logN)`，N表示数列的长度，因为每一次循环会将搜索范围缩小一半，所以问题的总长度就是2的搜索次数的次方，例如一个长度为`i32.Max`的数列，它的长度为`2^31-1`，所以最慢31次搜索就可以解决问题