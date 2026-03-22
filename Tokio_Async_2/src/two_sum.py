def sum_function(nums, t):
    for i in range(len(nums)):
        for j in range(i + 1, len(nums)):
            if nums[i] + nums[j] == t:
                return [nums[i], nums[j]]

nums = [1, 3, 5, 7, 4]
t = 8

result = sum_function(nums, t)
print(result)