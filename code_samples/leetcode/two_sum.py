class Solution:
    def twoSum(self, nums, target):
        visto = {}
        for i, num in enumerate(nums):
            comp = target - num
            if comp in visto:
                return [visto[comp], i]
            visto[num] = i
        return None

if __name__=="__main__":
    sol = Solution()
    print(sol.twoSum([2,7,11,15], 9))  # [0,1]
    print(sol.twoSum([3,2,4], 6))       # [1,2]
