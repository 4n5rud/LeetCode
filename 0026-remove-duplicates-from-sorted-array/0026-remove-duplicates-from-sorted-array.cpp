class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        // 투 포인터를 이용한 풀이를 배웠습니다.

        int write = 1;
        for (int i = 1 ; i < nums.size(); i++) {
            if (nums[i] != nums[i-1]) {
                nums[write] = nums[i];
                write++;
            }
        }

        //write는 새로운 값이 생길때 그 idx에 새로운 값을 채워넣고 write idx 를 ++ 합니다.
        //int i => read 전용 idx입니다, 전체를 순환합니다.
        return write;
    }
};