
//leetCode 54
//˳ʱ����������ξ��������

#include <vector>
using namespace std;
class SpiralOrder {
public :
	static vector<int> GetSpiralOrder(vector<vector<int>>& matrix) {
        int height = matrix.size();
        if (0 == height) {
            return vector<int>(0);
        }

        int width = matrix[0].size();
        //ȷ�ϵ�ǰ����ı߽磬�����ң����ϵ��£����ҵ��󣬴������ϣ����һ��˳ʱ�����
        int left = 0, right = width - 1, top = 0, bottom = height - 1;
        vector<int> result = vector<int>();
        while (true) {
            //����
            for (int i = left; i <= right; ++i) {
                result.push_back(matrix[top][i]);
            }
            top++;
            if (top > bottom) {
                break;
            }
            //�Ҳ�
            for (int i = top; i <= bottom; ++i) {
                result.push_back(matrix[i][right]);
            }
            right--;
            if (right < left) {
                break;
            }
            //�ײ�
            for (int i = right; i >= left; --i) {
                result.push_back(matrix[bottom][i]);
            }
            bottom--;
            if (bottom < top) {
                break;
            }
            //���
            for (int i = bottom; i >= top; --i) {
                result.push_back(matrix[i][left]);
            }
            left++;
            if (left > right) {
                break;
            }

        }
        return result;

	}
};