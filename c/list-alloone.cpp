#include <string>
#include <unordered_set>
#include <unordered_map>
#include <limits>

using std::numeric_limits;
using std::string;
using std::unordered_map;
using std::unordered_set;

class DoubleNode
{
public:
    int freq;
    DoubleNode *prev;
    DoubleNode *next;
    unordered_set<string> strs;

    DoubleNode(int x) : freq(x), prev(nullptr), next(nullptr) {}

    ~DoubleNode()
    {
        if (prev != nullptr)
        {
            prev->next = next;
        }

        if (next != nullptr)
        {
            next->prev = prev;
        }

        prev = nullptr;
        next = nullptr;
    }
};

class AllOne
{
    int len;
    DoubleNode *head;
    DoubleNode *tail;
    unordered_map<string, DoubleNode *> mp;

public:
    AllOne()
    {
        len = 0;
        head = new DoubleNode(0);
        tail = new DoubleNode(numeric_limits<int>::max());
        head->next = tail;
        tail->prev = head;
    }

    // 1. 判断字符串是否存在，如果不存在，那么词频为1
    // 2. 如果存在，那么词频为n+1
    // 3. 判断词频对应的节点是否存在，如果不存在，判断词频1的节点是否存在，不存在则创建并插入，如果存在则插入此字符串，老节点要删除此字符串
    // 4. 新词频一定在老词频的下一个节点，判断这个节点是否存在，如果存在则在新节点插入元素，老节点删除元素，如果不存在，新建节点，并且插入到老节点的后面
    // 5. 如果字符串不存在，长度加1，否则不动
    // 6. 修改词频表的内容
    void inc(string key)
    {
        auto find_val = mp.find(key);
        if (find_val == mp.end())
        {
            int cur_freq = 1;
            if (head->next->freq == 1)
            {
                head->next->strs.insert(key);
                mp[key] = head->next;
            }
            else
            {
                DoubleNode *new_node = new DoubleNode(1);
                new_node->strs.insert(key);
                new_node->prev = head;
                new_node->next = head->next;
                head->next = new_node;
                new_node->next->prev = new_node;
                mp[key] = new_node;
            }

            len++;
        }
        else
        {
            DoubleNode *old_node = find_val->second;
            DoubleNode *old_nxt_node = old_node->next;
            int new_freq = old_node->freq + 1;
            if (new_freq == old_nxt_node->freq)
            {
                old_nxt_node->strs.insert(key);
                mp[key] = old_nxt_node;
            }
            else
            {
                DoubleNode *new_node = new DoubleNode(new_freq);
                new_node->strs.insert(key);
                new_node->prev = old_node;
                new_node->next = old_nxt_node;
                old_node->next = new_node;
                old_nxt_node->prev = new_node;
                mp[key] = new_node;
            }

            old_node->strs.erase(key);
            if (old_node->strs.empty())
            {
                delete old_node;
            }
        }
    }

    // 1. 如果key不在词频表，不考虑
    // 2. 查看key在哪个词频，找到对应词频节点
    // 3. 如果key词频为1，那么删除词频节点中这个key，结束
    // 4. 如果key词频不为1，看这个词频的前一个节点是否存在，如果存在直接插入，不存在创建节点插入到老节点，要把老节点中的key删掉
    // 5. 更新map记录key的词频
    // 5. 删除词频的节点如果为空，那么这个节点也要被删除
    void dec(string key)
    {
        auto find_val = mp.find(key);
        if (find_val != mp.end())
        {
            DoubleNode *old_node = find_val->second;
            int new_freq = old_node->freq - 1;
            if (new_freq == 0)
            {
                old_node->strs.erase(key);
                mp.erase(key);
                len--;
            }
            else
            {
                DoubleNode *old_prev_node = old_node->prev;
                if (new_freq == old_prev_node->freq)
                {
                    old_prev_node->strs.insert(key);
                    mp[key] = old_prev_node;
                }
                else
                {
                    DoubleNode *new_node = new DoubleNode(new_freq);
                    new_node->strs.insert(key);
                    new_node->next = old_node;
                    new_node->prev = old_prev_node;
                    old_node->prev = new_node;
                    old_prev_node->next = new_node;

                    mp[key] = new_node;
                }
            }

            old_node->strs.erase(key);
            if (old_node->strs.empty())
            {
                delete old_node;
            }
        }
    }

    string getMaxKey()
    {
        return len == 0 ? "" : *(tail->prev->strs.begin());
    }

    string getMinKey()
    {
        return len == 0 ? "" : *(head->next->strs.begin());
    }
};

#include <iostream>
using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // ["AllOne","inc","inc","inc","inc","inc","dec","dec","getMaxKey","getMinKey"]
    // [[],["a"],["b"],["b"],["b"],["b"],["b"],["b"],[],[]]
    AllOne o = AllOne();
    o.inc("a");
    o.inc("b");
    o.inc("b");
    o.inc("b");
    o.inc("b");
    cout << "debug" << endl;
    o.dec("b");
    o.dec("b");
    cout << o.getMaxKey() << endl;
    cout << o.getMinKey() << endl;

    return 0;
}

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne* obj = new AllOne();
 * obj->inc(key);
 * obj->dec(key);
 * string param_3 = obj->getMaxKey();
 * string param_4 = obj->getMinKey();
 */