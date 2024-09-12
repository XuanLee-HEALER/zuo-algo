struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *detectCycle(struct ListNode *head)
{
    if (head == 0 || head->next == 0 || head->next->next == 0)
        return 0;

    struct ListNode *slow = head;
    struct ListNode *fast = head;
    while (fast != 0 && fast->next != 0)
    {
        slow = slow->next;
        fast = fast->next->next;
        if (slow == fast)
        {
            break;
        }
    }

    if (fast == 0 || fast->next == 0)
        return 0;

    while (1)
    {
        if (head == slow)
        {
            return head;
        }
        else
        {
            head = head->next;
            slow = slow->next;
        }
    }
}