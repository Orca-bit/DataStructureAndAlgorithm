//
// Created by liuhao on 2021/7/13.
//

class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val) : val(_val), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
            : val(_val), left(_left), right(_right), next(_next) {}
};

class Solution {
    public:
    Node* connect(Node* root) {
        if (root == nullptr) return nullptr;
        auto most_left = root;
        while (most_left -> left != nullptr) {
            auto head = most_left;
            while (head != nullptr) {
                head -> left -> next = head -> right;
                if (head -> next != nullptr) {
                    head -> right -> next = head -> next -> left;
                }
                head = head -> next;
            }
            most_left = most_left -> left;
        }
        return root;
    }
};