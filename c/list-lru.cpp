#include <unordered_map>
#include <iostream>

class DoubleNode
{
public:
    int key;
    int val;
    DoubleNode *prev;
    DoubleNode *next;

    DoubleNode(int k, int v) : key(k), val(v), prev(nullptr), next(nullptr)
    {
    }

    ~DoubleNode()
    {
    }
};

class DoubleList
{
private:
    DoubleNode *head;
    DoubleNode *tail;

public:
    DoubleList() : head(nullptr), tail(nullptr) {}

    ~DoubleList()
    {
        DoubleNode *cur = head;
        while (cur != nullptr)
        {
            DoubleNode *nxt = cur->next;
            delete cur;
            cur = nxt;
        }
    }

    void push(DoubleNode *node)
    {
        if (tail != nullptr)
        {
            tail->next = node;
            node->prev = tail;
            tail = node;
        }
        else
        {
            head = node;
            tail = node;
        }
    }

    DoubleNode *move_node_to_tail(DoubleNode *node)
    {
        if (node != tail)
        {
            if (node->prev != nullptr)
            {
                // node is mid
                node->prev->next = node->next;
                node->next->prev = node->prev;
            }
            else
            {
                // node is head
                head = node->next;
                head->prev = nullptr;
            }
            node->prev = tail;
            node->next = nullptr;
            tail->next = node;
            tail = node;
        }

        return node;
    }

    DoubleNode *pop()
    {
        if (head != nullptr)
        {
            DoubleNode *t_head = head;

            if (head->next == nullptr)
            {
                head = nullptr;
                tail = nullptr;
            }
            else
            {
                head = head->next;
                head->prev = nullptr;
            }

            t_head->next = nullptr;

            return t_head;
        }

        return nullptr;
    }

    void print_list()
    {
        for (DoubleNode *iter = head; iter != nullptr; iter = iter->next)
        {
            std::cout << iter->val << " ";
        }
        std::cout << std::endl;
    }
};

class LRUCache
{
private:
    DoubleList lst;
    std::unordered_map<int, DoubleNode *>
        mp;
    int cap;
    int len;

public:
    LRUCache(int capacity) : cap(capacity), len(0)
    {
    }

    int get(int key)
    {
        auto find_node = mp.find(key);
        if (find_node == mp.end())
        {
            return -1;
        }
        else
        {
            lst.move_node_to_tail(find_node->second);
            return find_node->second->val;
        }
    }

    void put(int key, int value)
    {
        auto find_node = mp.find(key);
        if (find_node == mp.end())
        {
            DoubleNode *node = new DoubleNode(key, value);
            if (len == cap)
            {
                DoubleNode *old_head = lst.pop();
                mp.erase(old_head->key);
            }
            else
            {
                len++;
            }
            mp[key] = node;
            lst.push(node);
        }
        else
        {
            DoubleNode *old_node = find_node->second;
            old_node->val = value;
            mp[key] = lst.move_node_to_tail(old_node);
        }
    }
};

int main(int argc, char *argv[])
{
    // std::cout << "good morning" << std::endl;
    // DoubleList lst;
    // for (int i = 0; i < 5; i++)
    // {
    //     std::cout << "debug" << std::endl;
    //     lst.push(new DoubleNode(i));
    // }

    // lst.pop();

    // lst.print_list();
    // [ "LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get" ]
    // [ [2], [ 1, 0 ], [ 2, 2 ], [1], [ 3, 3 ], [2], [ 4, 4 ], [1], [3], [4] ]

    // [ "LRUCache", "put", "get", "put", "get", "get" ]
    //     [[1], [ 2, 1 ], [2], [ 3, 2 ], [2], [3]]

    // [ "LRUCache", "get", "get", "put", "get", "put", "put", "put", "put", "get", "put" ]
    //     [[1], [6], [8], [ 12, 1 ], [2], [ 15, 11 ], [ 5, 2 ], [ 1, 15 ], [ 4, 2 ], [4], [ 15, 15 ]]

    LRUCache *cache = new LRUCache(1);
    // cache->put(2, 1);
    std::cout << "ans " << cache->get(1) << std::endl;
    // cache->put(3, 2);
    std::cout << "ans " << cache->get(6) << std::endl;
    std::cout << "ans " << cache->get(8) << std::endl;
    cache->put(12, 1);
    std::cout << "ans " << cache->get(2) << std::endl;
    cache->put(15, 11);
    std::cout << "xxx1" << std::endl;
    cache->put(5, 2);
    std::cout << "xxx2" << std::endl;
    cache->put(1, 15);
    std::cout << "xxx3" << std::endl;
    cache->put(4, 2);
    std::cout << "xxx4" << std::endl;
    std::cout << "ans " << cache->get(4) << std::endl;
    cache->put(15, 15);

    // std::cout << "ans " << cache->get(3) << std::endl;
    // std::cout << "ans " << cache->get(4) << std::endl;

    return 0;
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */