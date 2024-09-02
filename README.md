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

### 二分搜索的时间复杂度

二分搜索的时间复杂度是`O(logN)`，N表示数列的长度，因为每一次循环会将搜索范围缩小一半，所以问题的总长度就是2的搜索次数的次方，例如一个长度为`i32.Max`的数列，它的长度为`2^31-1`，所以最慢31次搜索就可以解决问题

## 时间复杂度和空间复杂度

常数操作：常数时间内操作，和操作数大小没有关系。例如数字运算操作，因为位数固定，所以同类型运算消耗的时间差不多。例如数组寻址，由于是计算索引获取数组数值，所以读取任意位置元素时间差不多。哈希函数的访问也是常数操作

非常数操作：例如链表，由于链表在内存中不是连续存放，所以根据节点数量不同，获取不同位置的节点也是不同的

### 时间复杂度

表示运行时间和数据量的关系

使用问题规模来计算，将每一次计算的数据量/规模相加，然后去掉非最高阶的式子，留下的最高阶式子去掉常数项，就是`O(x^n)`的复杂度

常数操作是`O(1)`的时间复杂度

和数据量相关的变量的阶数对于运行时间的意义最大，低阶项和常数项相比都不重要

对于插入排序，如果是最差情况（倒序排列），那么时间复杂度就是`O(n^2)`，但是最好情况时间复杂度变成了`O(n)`，要确定时间复杂度需要用最差情况

严格**固定流程**（没有随机性）的算法必须评估在最差情况下的性能

对于随机流程的算法，即每一个运算单元可能有不确定性的执行次数，那么最差情况就是算法无法执行完成，即随机性无法满足算法向下执行的概率。所以需要使用平均复杂度或者概率上的期望复杂度来评价

### 空间复杂度

额外空间复杂度

入参和出参的空间不占空间复杂度计算项。算法流程中为了支持实现开辟的新的空间就是空间复杂度。如果没有利用额外空间，那么空间复杂度为`O(1)`

### 最优解

各个方法中哪个方法的时间复杂度更低阶就更好，这种方法下使用更低的空间复杂度

### 复杂度的均摊

以动态数组为例，如果动态数组每次扩容都是加倍，那么在向长度为1动态数组插入n个数时，在插入第2*n个数时问题规模变为2*n+1，那么插入8个数，就是`1+1+2+1+4+1+1`，小于`2*8`，推知插入n个数，扩容的时间复杂度也小于2n，那么总的时间复杂度就是说`O(n)`，均摊到n次插入数据，扩容对于插入的时间复杂度就是`O(1)`

均摊时间复杂度主要是为了好估计算法的整体时间复杂度

### 估计复杂度的误区

- ⚠️估计时间复杂度不能简单根据循环的个数/代码结构判断量级
  - 一个循环完成的冒泡排序
  - 两个循环达成的`O(nlogn)`的复杂度，`for i = 0; i < N; i++ {for j = i; j < N; j+=i}`。`1+1/2+1/3+...+1/n`的结果收敛于`O(logn)`（调和级数）

### 时间复杂度比较

`O(1)` `O(logN)` `O(N)` `O(N*logN)` `O(N^2)` `O(N^3)` ... `O(N^k)` `O(2^N)` `O(k^N)` `O(N!)`

## 算法和数据结构分类

- 硬计算类算法：精确求解
- 软计算类算法：最优解，注重逼近解决问题

- （物理）连续结构：在底层是连续存储的，只需要知道首地址和偏移量就能直接查找节点。例如数组、一个单独数字（`i32`）
- （物理）跳转结构：在底层是不连续存储的，需要保存不同节点的地址才能查找。例如链表、树、图

任何一种数据结构都是通过这两种结构组合起来

## 单双链表及其反转，堆栈诠释

[code](./examples/5-linked-list.rs)

### 按值/引用传递参数

Java除了基本类型以外都是按引用传递，但无论是按值还是按引用传递，都是对原类型的拷贝，如果是基本类型就直接拷贝值传递，而引用类型是拷贝了引用（内存地址）本身然后传递。引用类型就是值在堆上保存，栈上只保存内存地址，传递也只用地址传递

### 单链表

内存中定义的结构，一个结构，它包含一个值和指向下一个节点的指针。最后一个节点的下一个节点指向空（`null`）

```rust
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
```

### 双链表

和单链表类似，只不过除了指向下一个节点的指针还有指向上一个节点的指针

```rust
struct DoubleListNode {
    val: i32,
    last: Option<Box<DoubleListNode>>,
    next: Option<Box<DoubleListNode>>,
}
```

### 链表的反转

函数签名`fn reverse(head ListNode) -> ListNode`

## 合并两个有序链表

[code](./examples/6-merge-sorted-list.rs)

问题描述：有两个有序链表，合并两个链表为一个有序链表

解法：边界条件是两个链表任意为空则返回另一个链表，然后先找到最小的节点作为最终返回的结果，然后两个遍历节点分别为头节点的下一个和另一个链表的头节点，开始比较两个遍历节点，并使用一个头节点的引用来修改指向下一个节点的指针，等到两个遍历节点任一为空时，将剩余的节点拼到引用的指向下一个节点的指针上即完成算法

⚠️rust中，由于不能在一个节点上同时存在可变引用和不可变引用，所以利用一个dummy节点作为头节点，使用它的一个可变引用来逐次拼接两个链表中的最小值，最终拼接剩余的节点，然后范围dummy节点指向的下一个节点即可完成算法

## 两个链表相加

[code](./examples/7-add-two-list.rs)

问题描述：两个链表表示两个数字，链表头从个位开始，结果是一个新链表

解法：两个遍历指针，如果非空就是本值，否则为0，每次相加，注意保存进位，每次生成一个新节点保存到最终结果中

## 划分链表

[code](./examples/8-divide-list.rs)

问题描述：给定一个链表，再给一个特定值x，将链表重排，小于x在左，大于x在右，但是不能改变原链表元素的相对次序

解法：四个裸指针初始为null，分别表示小于子链表的头尾和大于等于子链表的头尾，然后开始遍历链表，如果当前节点小于x，那么小头赋值为当前节点，小头围指向这一节点，如果非首次则小头尾的下一个指针指向这个节点，更新小头尾节点。对于大头相同逻辑。最后要判断，如果小头头为空，那么直接返回大头头，否则返回小头尾拼接大头头后的小头头

## 队列和栈

[code](./examples/9-queue-stack.rs)

队列和栈是一种逻辑结构，物理实现可以使用链表或者数组

- 队列：先进先出（FIFO），新来元素从尾部进，从头部出
- 栈：后进先出（LIFO）

队列实现

用链表实现，一条单链表就可以实现队列，保持两个指针，头指针总是指向头节点，尾指针指向最近被插入的数据，当移除数据的时候，头指针返回节点前移动到下一个节点

用数组实现，做题过程中元素个数确定，可以初始化一个长度为N的数组，头尾指针初始化为`(0, 0)`，队列的范围是`[0, n)`，当`l < r`时，认为队列非空，否则队列为空

```java
int peek()
int tail()
void offer()
int poll()
int size()
bool empty()
```

栈的实现

用数组实现，一个指针记录栈顶元素即可

```java
bool empty()
void push()
int pop()
int peek()
int size()
```

### 环形队列

数组实现的队列，被释放的元素（位置）可以重复利用，只要同时在队列里的元素不超过整个数组大小，就可以无限利用

不使用头尾指针来控制循环队列，而是使用一个size来控制当前的队列长度

## 栈和队列相互实现

[code](./examples/10-queue-stack-converse.rs)

### 用栈实现一个队列

问题描述：只有标准栈，实现一个队列结构

解法：使用两个栈，一个in栈，一个out栈。当取数时，只有out栈空时，把in栈的内容全部导入out栈，每次导入数据的时候需要将in栈清空

⚠️时间复杂度，对于栈实现的队列，对元素的每个操作的均摊时间复杂度是`O(1)`，因为每个元素在这个数据结构中的生命周期就是in栈的进出和out栈的进出

### 用队列实现栈

问题描述：只有标准队列，实现一个栈结构

解法：每次队列进一个元素，记录当这个元素弹出时，它之前有几个元素需要弹出再插入队列

假设队列为空，先插入1，则为1，然后插入2，那么1要先出再进变为2<-1，对于每个新进的元素，已有的元素都已经是倒序，所以只需要处理当前元素，将其放到开头即可

⚠️时间复杂度，插入元素操作是`O(N)`，其余方法是`O(1)`

## 最小栈

问题描述：设计一个栈，额外提供一个方法`getMin()`来在常数时间内获取栈中的最小值

解法：使用两个栈，数据栈之外提供给一个最小栈，如果新入元素是第一个元素或者是最小的元素，那么在两个栈同步压入元素，否则数据栈压数据，最小栈压栈顶

空间换时间

## 双端队列

定义：在队列两端都可以进出元素，且进出顺序为FIFO

实现：

- 链表实现：使用双向链表可以直接实现双端队列，只需要保证链表的长度为指定的固定值即可
- 数组实现：固定数组实现一个双端队列，头尾索引总是指向头尾元素，使用`size`来确定当前队列的总元素个数，尾部索引是`+1`后移，头部索引是`-1`前移，当达到最大/小值时要回到数组的头/尾索引继续。⚠️两个头尾索引最开始并没有指向某个值，所以插入第一个元素的时候，这两个索引不能变化，即两个索引总是指向头尾元素

⚠️这道题主要问题是对边界条件的处理

另一种解法：还可以从头尾各自插入，这样计算索引可以省去对边界条件的判断，单线程内速度更快（leetcode无区别）

## 二叉树及其三种序的遍历

简单定义：一个节点只有左右两个节点的树

- 先序遍历：中->左->右
- 中序遍历：左->中->右
- 后序遍历：左->右->中

遍历的递归序，可以派生出三种序的遍历顺序

```java
// 递归基本样子，用来理解递归序
public static void f(TreeNode head) {
  if (head == null) {
    return;
  }
  // 1
  f(head.left);
  // 2
  f(head.right);
  // 3
}
```

递归序就是每一个非空节点，在这个函数中都会有`1/2/3`个位置执行当前节点的操作，不同的遍历序就是在不同的位置去打印了节点

## 二叉树的非递归遍历及其时间复杂度

[code](./examples/14-binary-tree-noniter.rs)

用栈实现二叉树的先序遍历：先将头节点压入栈中，然后重复过程，弹出栈顶元素，然后将该节点的右子树压入栈中，再将该节点的做子树压入栈中，这是因为要先弹出左子树，再弹出右子树

用栈实现二叉树中序遍历：先将头节点压入栈中，然后将它的左边界（它自己的左子树和后续所有的左子树）压入栈中，然后重复过程，弹出一个节点之后，将右子树及其左边界压入栈中。弹出过程就是左中右的中序遍历

用栈实现二叉树后续遍历（两个栈）：由于后续遍历是左右中，所以利用先序遍历改为中右左，然后利用另一个栈保存所有遍历元素，最后再依次弹出就是正确结果

用栈实现二叉树后续遍历（一个栈）：记录当前弹出的节点，也就是下一个要弹出的节点的上一个弹出节点，第一次压入头节点，如果左右子树未处理，那么压入左子树，然后看左子树，如果左右子树为空，那么弹出并且记录此为弹出节点，开始继续查看栈顶节点，因为左子树处理过，所以压入右子树处理

时间复杂度

- 递归做法：由于每个节点在递归过程中会被访问三次，所以是`O(N)`
- 非递归做法：每个节点入栈次数可能是多次（引用），但是属于有限次数，时间复杂度也是`O(N)`

空间复杂度

都是`O(H)`，H是树的高度，因为无论是递归还是非递归（栈），它们的最大占用空间都是树的最高高度，对于递归来说，一条边界递归完成后的空间可以被回收利用，对于栈的空间也是可以在弹出之后循环利用

### Rust实现一个栈的后序遍历二叉树

![image](./picture/same-node.png)

在上图中的情况，如果使用`Rc::eq()`，会使得值相同的左右叶子节点被认为是同一个节点，所以在比较上一个弹出的节点和当前节点的左右子节点时应该使用`Rc::ptr_eq()`方法

## 算法笔试中的输入输出处理

### 填函数风格

给定一个问题，然后提供方法/类的签名（类名、方法名、参数类型/个数），只需要把解决问题的逻辑代码填充好，提交即可。后台会有两部分（测试数据和答案），它会根据测试数据生成指定类型的参数，然后调用写好的函数，最后对比答案。这种形式比较简单清晰

### ACM风格

判题后台依旧是同样的结构，但是它会要求考生自己从标准输入读取测试数据，并且在代码中讲结果打印到标准输出。这里推荐使用`BufferReader`、`StreamTokenizer`、`PrintWriter`来对标准输入、输出做一层包装，目的是减少IO次数，不同的语言使用对应的API。`StreamTokenizer`可以从一个输入流中一次获取一个非空字符

⚠️除答案以外不要随便往标准输出打印东西

### 临时动态空间和全局静态空间

每个测试用例都需要分配的临时数组，当知道最大的数据规模后，可以只创建一个静态的最大空间（数组），然后每个测试用例都复用这个空间，由于后台只算累积的内存使用量，这种方式可以跳过每次动态创建内存可能会碰到的内存使用量限制

### 按行读

对于一些问题，它不告诉数据规模，只说每一个问题的数据是一行，以x分隔符划分，此时应该一次读取一行，然后自行分隔内容，再组装数据

