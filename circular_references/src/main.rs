struct Student{
    name: String
}
impl Student {
    fn courses(&self, platform: Platform)-> Vec<String>{
        return platform.enrollments.iter().filter(|&e| e.student.name == self.name).map(|e| e.course.name.clone()).collect();
    }
}

struct Course{
    name: String
}

struct Enrollment<'a>{
    student: &'a Student,
    course: &'a Course, 
}
impl<'a> Enrollment <'a>{
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a>{
        return Enrollment{student:student, course:course};
    }
}

struct Platform<'a>{
    enrollments: Vec<Enrollment<'a>>
}
impl<'a> Platform<'a>{
    fn new() -> Platform<'a>{
        Platform{
            enrollments: Vec::new()
        }
    }
    fn enroll(&mut self, student: &'a Student, course: &'a Course){
        self.enrollments.push(Enrollment{student: student, course:course});
    }
}

fn main() {
    let john = Student{
        name: "John".into()
    };
    
    let course = Course{
        name: "Intro to Rust".into()
    };

    let mut platform = Platform::new();
    platform.enroll(&john, &course);

    for c in john.courses(platform){
        println!("John is taking {}", c);
    }
}
