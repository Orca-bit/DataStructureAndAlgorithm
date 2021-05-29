#[derive(Clone, Debug)]
struct Student {
    name: String,
    id: i32,
    score: i32
}

impl Student {
    pub fn new(name: String, id: i32, score: i32) -> Self {
        Self {
            name,
            id,
            score
        }
    }
}

#[test]
fn test() {
    let student1 = Student::new("A".to_string(), 2, 20);
    let student2 = Student::new("B".to_string(), 3, 21);
    let student3 = Student::new("C".to_string(), 1, 22);
    let mut student_v = vec![student1.clone(), student2.clone(), student3.clone()];
    student_v.sort_unstable_by(|stu1, stu2| stu2.id.cmp(&stu1.id));
    println!("{:?}", student_v);
}