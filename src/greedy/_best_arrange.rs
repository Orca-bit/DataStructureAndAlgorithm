#[derive(Clone, Copy, Debug)]
struct Program {
    start: u32,
    end: u32,
}

impl Program {
    fn new(start: u32, end: u32) -> Self {
        if start < 0 || end > 23 || end < start {
            panic!("Please input the correct time");
        }
        Self { start, end }
    }
}

struct BestArrange;

impl BestArrange {
    fn best_arrange(programs: &mut [Program]) -> Vec<Program> {
        let mut arranged = vec![];
        programs.sort_unstable_by(|program1, program2| program1.end.cmp(&program2.end));
        let mut time_point = 0;
        for &mut program in programs {
            if time_point <= program.start {
                arranged.push(program);
                time_point = program.end;
            }
        }
        arranged
    }
}

#[test]
fn test() {
    let program1 = Program::new(1, 3);
    let program2 = Program::new(11, 14);
    let program3 = Program::new(12, 13);
    let program4 = Program::new(13, 18);
    let program5 = Program::new(14, 17);
    let program6 = Program::new(17, 20);
    let mut v = vec![program1, program2, program3, program4, program5, program6];
    let arrange = BestArrange::best_arrange(&mut v);
    for p in arrange {
        println!("{:?}", p);
    }
}
