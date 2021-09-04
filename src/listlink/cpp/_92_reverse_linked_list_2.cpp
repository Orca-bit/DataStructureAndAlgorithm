//
// Created by liuhao on 2021/9/2.
//

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
 struct ListNode {
         int val;
         ListNode *next;
         ListNode() : val(0), next(nullptr) {}
         explicit ListNode(int x) : val(x), next(nullptr) {}
         ListNode(int x, ListNode *next) : val(x), next(next) {}
 };

class Solution {
public:
    static ListNode* reverseBetween(ListNode* head, int left, int right) {
        auto* dummy = new ListNode();
        dummy -> next = head;
        auto* g = dummy;
        auto* p = dummy -> next;
        for(int i = 0; i < left - 1; ++i) {
            g = g -> next;
            p = p -> next;
        }
        for(int i = 0; i <= right - left; ++i) {
            auto* next = p -> next;
            p -> next = next -> next;
            next -> next = g -> next;
            g -> next = next;

        }
        return dummy -> next;
    }
};