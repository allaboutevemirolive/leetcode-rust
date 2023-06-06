// https://leetcode.com/problems/first-missing-positive/solutions/3408002/beat-99-in-space-complexity/
class Solution {
    public int firstMissingPositive(int[] nums) {
       boolean b[] = new boolean[nums.length+2];
       for(int i=0; i<nums.length; i++){
           if(nums[i] > 0 && nums[i] <= nums.length){
               b[nums[i]] = true;
           }
       }
       System.gc();
       for(int i=1; i<b.length; i++){
           if(!b[i]){
               return i;
           }
       }
       return 0;
    }
}