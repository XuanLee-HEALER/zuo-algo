/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
#include <stdio.h>

struct ListNode
{
    int val;
    struct ListNode *next;
};

void print_linkedlist(struct ListNode *head)
{
    for (; head != 0; head = head->next)
    {
        printf("%d ", head->val);
    }
    printf("\n");
}

struct ListNode *sortList(struct ListNode *head)
{
    if (head == 0 || head->next == 0)
        return head;

    for (int step = 1;; step <<= 1)
    {
        struct ListNode *left = head;
        struct ListNode *right = head;
        struct ListNode *ori_last_group_tail = 0;
        for (int count = 0;; count++)
        {
            bool end = 0;
            for (int j = 0; j < step && right != 0; ++j)
            {
                right = right->next;
            }

            if (right == 0)
            {
                if (count == 0)
                    return head;
                else
                    break;
            }

            struct ListNode *nxt_head = 0;
            // set two single list
            struct ListNode *l_iter_node = left;
            struct ListNode *r_iter_node = right;
            for (int j = 0; j < step - 1; ++j)
            {
                l_iter_node = l_iter_node->next;
                if (r_iter_node != 0)
                    r_iter_node = r_iter_node->next;
            }
            l_iter_node->next = 0;
            if (r_iter_node != 0)
            {
                nxt_head = r_iter_node->next;
                r_iter_node->next = 0;
            }

            struct ListNode *cur_head = left;
            l_iter_node = left;
            r_iter_node = right;

            if (left->val > right->val)
            {
                cur_head = right;
                l_iter_node = right;
                r_iter_node = left;
            }

            while (l_iter_node->next != 0 && r_iter_node != 0)
            {
                if (l_iter_node->next->val > r_iter_node->val)
                {
                    struct ListNode *t_node = l_iter_node->next;
                    l_iter_node->next = r_iter_node;
                    l_iter_node = l_iter_node->next;
                    r_iter_node = t_node;
                }
                else
                {
                    l_iter_node = l_iter_node->next;
                }
            }

            if (r_iter_node != 0)
            {
                l_iter_node->next = r_iter_node;
            }

            while (l_iter_node->next != 0)
            {
                l_iter_node = l_iter_node->next;
            }

            if (count == 0)
            {
                ori_last_group_tail = l_iter_node;
                head = cur_head;
                l_iter_node->next = nxt_head;
            }
            else
            {
                ori_last_group_tail->next = cur_head;
                l_iter_node->next = nxt_head;
                ori_last_group_tail = l_iter_node;
            }

            left = nxt_head;
            right = nxt_head;
        }
    }
}

int main()
{
    struct ListNode node8 = {1, 0};
    struct ListNode node7 = {2, 0};
    struct ListNode node6 = {1, 0};
    struct ListNode node5 = {2, 0};
    struct ListNode node4 = {2, 0};
    struct ListNode node3 = {1, 0};
    struct ListNode node2 = {2, 0};
    struct ListNode node1 = {1, 0};
    node1.next = &node2;
    node2.next = &node3;
    node3.next = &node4;
    node4.next = &node5;
    node5.next = &node6;
    node6.next = &node7;
    node7.next = &node8;

    // print_linkedlist(&node1);
    struct ListNode *res = sortList(&node1);
    print_linkedlist(res);
}