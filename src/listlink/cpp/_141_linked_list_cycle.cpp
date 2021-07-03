//
// Created by liuhao on 2021/7/3.
//
struct ListNode {
    int val;
    ListNode* next;
    ListNode(int val) : val(val), next(nullptr) {}
}

class Solution {
    public:
    bool hasCycle(ListNode* head) {
        return getFirstLoopNode(head) != nullptr;
    }

    ListNode* getFirstLoopNode(ListNode* head) {
        if(head == nullptr || head -> next == nullptr || head -> next -> next == nullptr) {
            return nullptr;
        }
        ListNode* slow = head -> next;
        ListNode* fast = head -> next -> next;
        while(slow != fast) {
            if(fast -> next == nullptr || fast -> next -> next == nullptr) {
                return nullptr;
            }
            fast = fast -> next -> next;
            slow = slow -> next;
        }
        fast = head;
        while(slow != fast) {
            slow = slow -> next;
            fast = fast -> next;
        }
        return slow;
    }
};