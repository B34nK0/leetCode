#pragma once

#include <CommonStruct.h>
#include <utility>
#include <tuple>
/*
*leetCode 25 k��һ�鷴ת����
* ����һ��������������k����������ÿk���ڵ�Ϊһ����з�ת
* 1->2->3->4->5 k=2
* 2->1->4->3->5
*/

using namespace std;
class ReverseKGroupList {
public:
	//���ط�ת���ͷ��β�ڵ�λ��
	static pair<ListNode*, ListNode*> reverseSubList(ListNode* head, ListNode* tail) {
		//��������β�ڵ����һ���ڵ���Ϊ��ǰ��ǰ�ڵ�
		ListNode* prev = tail->next;
		ListNode* curr = head;
		//��ת����
		while (prev != tail) {
			ListNode* next = curr->next;
			curr->next = prev;
			prev = curr;
			curr = next;
		}

		return { tail, head };
	}

	static ListNode* Reverse(ListNode* head, int k) {
		ListNode* hair = new ListNode();
		hair->next = head;
		ListNode* pre = hair;
		while (head) {
			ListNode* tail = pre;

			//��ȡ��������k���ڵ�ͷ��β�ڵ�λ��
			// �鿴ʣ�ಿ�ֳ����Ƿ���ڵ��� k
			for (int i = 0; i < k; ++i) {
				tail = tail->next;
				if (!tail) {
					return hair->next;
				}
			}

			//������βֱ�ߵ�����һ���������������������ڿ�ʼ��һ��������
			ListNode* next = tail->next;
			//�������б��ķ�ת�б�
			tie(head, tail) = reverseSubList(head, tail);
			pre->next = head;
			tail->next = next;
			pre = tail;
			//��ת��һ��������
			head = tail->next;
		}

		return hair->next;
	}
};