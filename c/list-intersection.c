/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode
{
    int val;
    struct ListNode *next;
};

#include <math.h>

struct ListNode *getIntersectionNode(struct ListNode *headA, struct ListNode *headB)
{
    if (headA == 0 && headB == 0)
    {
        return 0;
    }

    int diff = 0;
    struct ListNode *a = headA;
    struct ListNode *b = headB;

    while (a != 0 && a->next != 0)
    {
        diff++;
        a = a->next;
    }

    while (b != 0 && b->next != 0)
    {
        diff--;
        b = b->next;
    }

    if (a != b)
    {
        return 0;
    }

    if (diff > 0)
    {
        a = headA;
        b = headB;
    }
    else
    {
        a = headB;
        b = headA;
    }

    diff = (int)fabsf((float)diff);

    for (; diff > 0; --diff)
    {
        a = a->next;
    }

    while (1)
    {
        if (a == b)
        {
            return a;
        }
        else
        {
            a = a->next;
            b = b->next;
        }
    }
}