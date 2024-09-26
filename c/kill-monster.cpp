#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <algorithm>
#include <cctype>
#include <limits>

// 函数用于修剪字符串两端的空白字符
std::string trim(const std::string &str)
{
    auto start = str.begin();
    while (start != str.end() && std::isspace(*start))
    {
        start++;
    }

    auto end = str.end();
    do
    {
        end--;
    } while (std::distance(start, end) > 0 && std::isspace(*end));

    return std::string(start, end + 1);
}

// 函数用于将字符串按空格分割成数组
std::vector<std::string> split(const std::string &str)
{
    std::istringstream stream(str);
    std::string word;
    std::vector<std::string> words;

    while (stream >> word)
    {
        words.push_back(word);
    }

    return words;
}

void swapElements(int (&arr)[10], int index1, int index2)
{
    int temp = arr[index1];
    arr[index1] = arr[index2];
    arr[index2] = temp;
}

// 返回值为当前怪物血量，最少需要用几个技能怪物才能死
int min_mana(int (&kill)[10], int (&double_limit)[10], int monster, size_t mana, size_t cur_mana)
{
    // std::cout << "debug " << monster << " " << mana << " " << cur_mana << std::endl;

    int res = -1;
    if (monster <= 0)
    {
        return 0;
    }
    else if (cur_mana == mana)
    {
        return res;
    }
    for (auto i = cur_mana; i < mana; i++)
    {
        swapElements(kill, i, cur_mana);
        swapElements(double_limit, i, cur_mana);
        int next_monster = monster <= double_limit[cur_mana] ? monster - 2 * kill[cur_mana] : monster - kill[cur_mana];
        int tmp_res = min_mana(kill, double_limit, next_monster, mana, cur_mana + 1);
        res = res == -1 ? tmp_res == -1 ? -1 : 1 + tmp_res : tmp_res == -1 ? res
                                                                           : std::min(res, 1 + tmp_res);
        // std::cout << "current result: " << res << std::endl;
        swapElements(kill, i, cur_mana);
        swapElements(double_limit, i, cur_mana);
    }
    return res;
}

int main()
{
    std::string line;
    uint32_t iter_times;
    int monster;
    int kill[10];
    int double_limit[10];

    if (std::getline(std::cin, line))
    {
        try
        {
            iter_times = std::stoul(line);
        }
        catch (const std::exception &e)
        {
            throw std::runtime_error(e.what());
        }
    }
    else
    {
        throw std::runtime_error("io error");
    }

    for (uint32_t i = 0; i < iter_times; ++i)
    {
        size_t cur_mana;

        if (std::getline(std::cin, line))
        {
            line = trim(line);
            auto segs = split(line);
            cur_mana = std::stoul(segs[0]);
            monster = std::stoul(segs[1]);
        }
        else
        {
            throw std::runtime_error("io error");
        }

        for (size_t i = 0; i < cur_mana; i++)
        {
            if (std::getline(std::cin, line))
            {
                line = trim(line);
                auto segs = split(line);
                kill[i] = std::stoi(segs[0]);
                double_limit[i] = std::stoi(segs[1]);
            }
            else
            {
                throw std::runtime_error("io error");
            }
        }

        // std::cout << "debug1 " << kill[0] << " " << kill[1] << " " << kill[2] << std::endl;
        // std::cout << "debug2 " << double_limit[0] << " " << double_limit[1] << " " << double_limit[2] << std::endl;
        int cur_res = min_mana(kill, double_limit, monster, cur_mana, 0);
        std::cout << cur_res << std::endl;
        std::cout.flush();
    }

    return 0;
}
