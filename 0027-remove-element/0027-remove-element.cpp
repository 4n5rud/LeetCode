class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int write = 0;
        for (int i = 0 ; i < nums.size(); i++) {
            // cout << "i : " << i << "\n"; 
            // cout << "write : " << write << "\n"; 
            // cout << "[ ";
            // for(int j = 0 ; j < nums.size(); j++) {
            //     cout << nums[j];
            // }
            // cout << " ]" << '\n';

            if(nums[i] != val && i == write) {
                write++;
            } else if (nums[i] != val && i != write) {
                int tmp = nums[i];
                nums[i] = nums[write];
                nums[write] = tmp;

                write++;
            }

        }
        return write;
    }
};