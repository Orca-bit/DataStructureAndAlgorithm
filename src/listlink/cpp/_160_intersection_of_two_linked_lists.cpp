//
// Created by liuhao on 2021/7/3.
//

struct ListNode {
    int val;
    ListNode * next;
    ListNode(int val) : val(val), next(nullptr) {}
};

class Solution {
    public:
    ListNode* getIntersectionNode(ListNode *headA, ListNode *headB) {
        if (headA == nullptr || headB == nullptr) {
            return nullptr;
        }
        ListNode *p1 = headA;
        ListNode *p2 = headB;
        int n = 0;
        while (p1 != nullptr) {
            n += 1;
            p1 = p1 -> next;
        }
        while (p2 != nullptr) {
            n -= 1;
            p2 = p2 -> next;
        }
        if (p1 != p2) {
            return nullptr;
        }
        p1 = n > 0 ? headA : headB;
        p2 = p1 == headA ? headB : headA;
        if (n < 0) {
            n = -n;
        }
        while (n > 0) {
            n -= 1;
            p1 = p1 -> next;
        }
        while (p1 != p2) {
            p1 = p1 -> next;
            p2 = p2 -> next;
        }
        return p1;
    }
};