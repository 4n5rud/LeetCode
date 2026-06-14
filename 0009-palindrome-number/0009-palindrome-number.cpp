#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
    bool isPalindrome(int x) {
        std::string str = to_string(x);
        std::vector<int> arr;

        int st = 0;
        int en = str.size() - 1;
        // std::cout << "st:" << st << '\n';
        // std::cout << "en:" << en << '\n';

        while(st <= en) {
            // std::cout << st << "/" << en;
            if(str[st] == str[en]) {
                st++;
                en--;
            } else {
                return false;
            }

        }

        return true;


    }
};