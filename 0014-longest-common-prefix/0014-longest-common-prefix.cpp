class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {

        if (strs.empty()) {
            return "";
        }
        int min = strs[0].size();
        for (int i = 1 ; i < strs.size(); i++) {
            if (min > strs[i].size()) {
                min = strs[i].size();
            }
        }
        

        for (int k = 0 ; k < min; k++) {
            for (int i = 1 ; i < strs.size(); i++) {
                if(strs[0][k] != strs[i][k]) {
                    return strs[0].substr(0,k);
                }
      
            }
        }

        return strs[0].substr(0,min);
    }
};