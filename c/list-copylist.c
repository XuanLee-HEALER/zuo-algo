/**
 * Definition for a Node.
 * struct Node {
 *     int val;
 *     struct Node *next;
 *     struct Node *random;
 * };
 */

#include <stdio.h>
#include <stdlib.h>

struct Node
{
    int val;
    struct Node *next;
    struct Node *random;
};

typedef struct Node *Node;

Node copyRandomList(Node head)
{
    Node iter_node = head;
    while (iter_node != 0)
    {
        Node next = iter_node->next;
        Node new_node = (Node)malloc(sizeof(struct Node));
        new_node->next = 0;
        new_node->random = 0;
        new_node->val = iter_node->val;
        iter_node->next = new_node;
        iter_node->next->next = next;
        iter_node = next;
    }

    iter_node = head;
    while (iter_node != 0)
    {
        if (iter_node->random != 0)
        {
            iter_node->next->random = iter_node->random->next;
        }
        iter_node = iter_node->next->next;
    }

    struct Node t_head = {-1, 0, 0};
    Node sentinel = &t_head;
    Node iter1 = sentinel;
    iter_node = head;
    while (iter_node != 0)
    {
        Node next = iter_node->next->next;
        iter1->next = iter_node->next;
        iter1 = iter1->next;
        iter_node->next = next;
        iter_node = iter_node->next;
    }

    return sentinel->next;
}

int main()
{
    // [[7,null],[13,0],[11,4],[10,2],[1,0]]
    struct Node node5 = {1, 0, 0};
    struct Node node4 = {10, &node5, 0};
    struct Node node3 = {11, &node4, 0};
    struct Node node2 = {13, &node3, 0};
    struct Node node1 = {7, &node2, 0};
    node2.random = &node1;
    node3.random = &node4;
    node4.random = &node3;
    node5.random = &node1;

    Node res = copyRandomList(&node1);
    printf("%d\n", node1.next->val);

    return 0;
}