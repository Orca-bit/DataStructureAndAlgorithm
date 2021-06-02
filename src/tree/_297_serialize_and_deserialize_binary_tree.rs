use super::util::*;
use std::iter::Peekable;
use std::vec::IntoIter;

struct Codec;

enum Tok {
    Op(char),
    Num(i32),
}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: TreeLink) -> String {
        let mut res = "".to_string();
        Self::serialize_core(&root, &mut res);
        res
    }

    fn deserialize(&self, data: String) -> TreeLink {
        let token = Self::parse_token(data);
        let mut it = token.into_iter().peekable();
        Self::parse_tree(&mut it)
    }

    // 序列化核心代码，先序遍历
    fn serialize_core(root: &TreeLink, s: &mut String) {
        s.push('(');
        if let Some(node) = root {
            let node = node.borrow();
            *s += &format!("{}", node.val);
            Self::serialize_core(&node.left, s);
            Self::serialize_core(&node.right, s);
        }
        s.push(')');
    }

    // 序列化字符串转化为包含标识和数字的vec
    fn parse_token(data: String) -> Vec<Tok> {
        // peekable迭代器可以仅仅查看元素，不会消费元素
        let mut it = data.chars().peekable();
        let mut res = vec![];
        while let Some(c) = it.next() {
            // 循环读取字符并消费掉
            // 当字符为标识时，将标识压入vec
            // 当字符为数字或者正负号时，进一步处理
            if c == '(' || c == ')' {
                res.push(Tok::Op(c));
            } else {
                let mut sign = 1;
                let mut val = 0;
                if c == '-' {
                    sign = -1;
                } else {
                    val = (c as u8 - b'0') as i32;
                }
                // 先使用迭代器的peek()方法查看下一个元素
                // 如果为数字的话，使用next()消费掉该数字，循环进行，直到peek()看到元素不为数字为止
                while let Some('0'..='9') = it.peek() {
                    val *= 10;
                    val += (it.next().unwrap() as u8 - b'0') as i32;
                }
                // 将数字乘以符号压入vec
                res.push(Tok::Num(sign * val));
            }
        }
        res
    }

    // 利用vec of token还原树
    fn parse_tree(it: &mut Peekable<IntoIter<Tok>>) -> TreeLink {
        let mut res = None;
        // 先消费掉'('，开头和每个数字之前必为'('
        it.next();
        // 查看下一个元素，只有可能是数字或者')'
        match it.peek() {
            Some(&Tok::Num(val)) => {
                // 为数字的话，消费掉
                // 构建以当前数字为root的树
                // 左子树和右子树分别递归进行，迭代器指向下一位置
                it.next();
                res = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: Self::parse_tree(it),
                    right: Self::parse_tree(it),
                })));
            }
            // 如果是')'，说明原树当前位置为None，do nothing
            Some(&Tok::Op(')')) => {}
            _ => panic!(),
        }
        // 消费掉')',无论当前过程有没有处理数字，一定有与该开始消费的'('对应的')'
        it.next();
        res
    }
}

#[test]
fn test() {
    let codec = Codec::new();
    let root = tree!(1, tree!(2), tree!(3, tree!(4), tree!(5)));
    // println!("{}", codec.serialize(root));
    let res = tree!(1, tree!(2), tree!(3, tree!(4), tree!(5)));
    assert_eq!(codec.deserialize(codec.serialize(root)), res);
}

#[test]
fn test_peek() {
    let s = "abcdefg".to_string();
    let mut it = s.chars().peekable();
    while let Some('a'..='g') = it.peek() {
        println!("{}",it.next().unwrap());
    }
}
