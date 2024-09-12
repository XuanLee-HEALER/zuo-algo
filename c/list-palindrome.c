#include <stdio.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

bool isPalindrome(struct ListNode *head)
{
    if (head == 0)
    {
        return 1;
    }

    struct ListNode *slow = head;
    struct ListNode *fast = head;
    while (fast->next != 0 && fast->next->next != 0)
    {
        slow = slow->next;
        fast = fast->next->next;
    }

    struct ListNode *cur = slow;
    struct ListNode *last = 0;
    while (cur != 0)
    {
        struct ListNode *nxt = cur->next;
        cur->next = last;
        last = cur;
        if (nxt == 0)
            break;
        else
            cur = nxt;
    }

    bool res = 1;
    struct ListNode *restore_ptr = cur;
    slow = head;
    while (slow != 0)
    {
        if (cur->val != slow->val)
        {
            res = 0;
            break;
        }
        slow = slow->next;
        cur = cur->next;
        printf("%d %d\n", slow == 0, cur == 0);
    }

    last = 0;
    while (restore_ptr != 0)
    {
        struct ListNode *nxt = restore_ptr->next;
        restore_ptr->next = last;
        last = restore_ptr;
        restore_ptr = nxt;
    }

    return res;
}

void print_linkedlist(struct ListNode *head)
{
    for (; head != 0; head = head->next)
    {
        printf("%d ", head->val);
    }
    printf("\n");
}

int main()
{
    struct ListNode node4 = {1, 0};
    struct ListNode node3 = {2, 0};
    struct ListNode node2 = {2, 0};
    struct ListNode node1 = {1, 0};
    node1.next = &node2;
    node2.next = &node3;
    node3.next = &node4;
    printf("%s\n", isPalindrome(&node1) == 1 ? "true" : "false");
    print_linkedlist(&node1);

    node2.val = 0;
    node3.val = 0;
    node3.next = 0;
    printf("%s\n", isPalindrome(&node1) == 1 ? "true" : "false");
    print_linkedlist(&node1);
}