#pragma once
/// <summary>
/// leetCode 92反转链表，给出链表头指针head，以及left、right值，反转left~right间节点
/// 
/// 输入：head = [1,2,3,4,5], left = 2, right = 4
/// 输出：[1, 4, 3, 2, 5]
/// 
/// 
/// 题目理解错误，，链表是无需的， 是将第left个节点到right个节点区间内的节点进行反转
/// </summary>
/// 
#include "CommonStruct.h"
class ReverseBetween {
public:
	//返回新的表头
	static void reverseSubList(ListNode* head, ListNode* tail) {
		//将子链表尾节点的下一个节点作为当前的前节点
		ListNode* prev = tail->next;
		ListNode* curr = head;
		while (prev != tail) {
			ListNode* next = curr->next;
			curr->next = prev;
			prev = curr;
			curr = next;
		}
	}

	static ListNode* GetReverseRes(ListNode* head, int left, int right) {
		if (left > right) return nullptr;
		if (!head) return nullptr;

		//先找出待选择的子链头节点跟尾节点
		ListNode* subHead = nullptr, * subTail = nullptr, *subListPre = nullptr;
		ListNode* tail = head;
		while (tail) {
			//先找出子链表头
			if (tail->val == left) {
				subHead = tail;
			}
			else if(!subHead){
				subListPre = tail;
			}
			//子链表尾必须是有了表头之后
			if (tail->val == right && subHead) {
				subTail = tail;
			}
			tail = tail->next;
		}
		//
		if (!subHead || !subTail) {
			return nullptr;
		}
		//反转子链表,返回新的表头
		reverseSubList(subHead, subTail);
		if (!subListPre) {
			head = subTail;
		}
		else {
			subListPre->next = subTail;
		}

		return head;
	}
};