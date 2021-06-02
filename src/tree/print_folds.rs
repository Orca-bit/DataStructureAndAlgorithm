struct PrintFolds;

impl PrintFolds {
    fn print_folds(n: u32) {
        Self::process(1, n, true);
    }

    fn process(i: u32, n: u32, down: bool) {
        if i > n {
            return;
        }
        Self::process(i + 1, n, true);
        println!("{}", if down { "凹" } else { "凸" }); //中序遍历打印折痕
        Self::process(i + 1, n, false);
    }
}

#[test]
fn test() {
    PrintFolds::print_folds(3);
}
