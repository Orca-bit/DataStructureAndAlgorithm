//
// Created by liuhao on 2021/5/31.
//
#include <vector>
#include <stack>
using namespace std;
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(): val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x): val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right): val(x), left(left), right(right) {}
};
class Solution {
public:
    vector<int> preorder_traversal(TreeNode* root) {
        vector<int> ans;
        preorder_traversal_recur(ans, root);
        return ans;
    }

    void preorder_traversal_recur(vector<int> &v, TreeNode* node) {
        if (node == nullptr) return;
        v.push_back(node->val);
        preorder_traversal_recur(v, node->left);
        preorder_traversal_recur(v, node->right);
    }

    vector<int> preorder_traversal_unrecur(TreeNode* root) {
        stack<TreeNode*> stk;
        vector<int> ans;
        if (root == nullptr) return ans;
        stk.push(root);
        while (!stk.empty()) {
            auto node = stk.top();
            stk.pop();
            ans.push_back(node->val);
            if (node->right != nullptr) stk.push(node->right);
            if (node->left!= nullptr) stk.push(node->left);
        }
        return ans;
    }

    vector<int> inorder_traversal(TreeNode* root) {
        vector<int> ans;
        inorder_traversal_recur(ans, root);
        return ans;
    }

    void inorder_traversal_recur(vector<int> &v, TreeNode* node) {
        if (node == nullptr) return;
        inorder_traversal_recur(v, node->left);
        v.push_back(node->val);
        inorder_traversal_recur(v, node->right);
    }

    vector<int> inorder_traversal_unrecur(TreeNode* root) {
        stack<TreeNode*> stk;
        vector<int> ans;
        if (root == nullptr) return ans;
        while (!stk.empty() || root != nullptr) {
            if (root != nullptr) {
                stk.push(root);
                root = root->left;
            } else {
                auto node = stk.top();
                stk.pop();
                ans.push_back(node->val);
                root = node->right;
            }
        }
        return ans;
    }

};