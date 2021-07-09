//
// Created by liuhao on 2021/5/31.
//
#include <unordered_map>
using namespace std;
class Node {
public:
    int val;
    Node* next;
    Node* random;

    explicit Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};


class Solution {
public:
    Node* copy_random_list(Node* head) {
        unordered_map<Node*, Node*> map;
        Node* cur = head;
        while (cur != nullptr) {
            map[cur] = new Node(cur -> val);
            cur = cur -> next;
        }
        cur = head;
        while (cur != nullptr) {
            map[cur] -> next = map[cur -> next];
            map[cur] -> random = map[cur -> random];
            cur = cur -> next;
        }
        return map[head];
    }

    Node* copy_random_list_1(Node* head) {
        if (head == nullptr) return nullptr;
        Node* cur = head;
        Node* next = nullptr;
        while (cur != nullptr) {
            next = cur->next;
            cur->next = new Node(cur->val);
            cur->next->next = next;
            cur = next;
        }
        cur = head;
        Node* cur_copy = nullptr;
        while (cur != nullptr) {
            next = cur->next->next;
            cur_copy = cur->next;
            cur_copy->random = cur->random == nullptr? nullptr : cur->random->next;
            cur = next;
        }
        Node* res = head->next;
        cur = head;
        while (cur != nullptr) {
            next = cur->next->next;
            cur_copy = cur->next;
            cur->next = next;
            cur_copy->next = next == nullptr? nullptr : next->next;
            cur = next;
        }
        return res;
    }
};

