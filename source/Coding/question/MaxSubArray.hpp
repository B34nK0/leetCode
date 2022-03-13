/*
* leetCode 53 ����������
* 
* ����һ������nums������������ĺ�
* 
* -2,1,-3,4,-1,2,1,-5,4
* 
* ������� 4,-1,2,1 ��Ϊ 6
*/

#include <vector>
using namespace std;

class MaxSubArray {
public:
	//���е�m���� [l,r]����Ϊ��������
	struct Status {
		//lsum ����������Ӷκ�
		//rsum ����������Ӷκ�
		//msum lr��������Ӷκ�
		//isum lr �����
		int lSum, rSum, mSum, iSum;

		Status(int lSum, int rSum, int mSum, int iSum) {
			this->lSum = lSum;
			this->rSum = rSum;
			this->mSum = mSum;
			this->iSum = iSum;
		}
	};


	Status pushUp(Status l, Status r) {
		int iSum = l.iSum + r.iSum;
		int lSum = max(l.lSum, l.iSum + r.lSum);
		int rSum = max(r.rSum, r.iSum + l.rSum);
		int mSum = max(max(l.mSum, r.mSum), l.rSum + r.lSum);
		return Status(lSum, rSum, mSum, iSum);
	};


	Status get(vector<int>& nums, int l, int r) {
		if (l == r) {
			return Status{ nums[l], nums[l], nums[l], nums[l] };
		}

		int m = (l + r) >> 1;
		Status left = get(nums, l, m);
		Status right = get(nums, m + 1, r);
		return pushUp(left, right);
	}

	int GetRes(vector<int>& nums) {
		if (nums.empty()) return 0;

		return get(nums, 0, nums.size()-1).mSum;
	}
};