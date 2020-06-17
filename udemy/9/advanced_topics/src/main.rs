
// student* --- *course

use std::cell::RefCell;
use std::rc::Rc;
/*
struct Student<'a> {
    name: String,
    courses: Vec<&'a Course<'a>>
}

impl<'a> Student<'a> {
    fn new(name: &str) -> Student<'a> {
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course<'a> {
    name: String,
    students: Vec<&'a Student<'a>>
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(&'a mut self, student: &'a mut Student<'a>) {
        // wont work
        // lifetime `'a` defined here
        student.courses.push(self);
        self.students.push(student);
        // RefCell
    }
}

*/


// APPROACH 2---------------------

// -- With Rc and refcells


struct Student1 {
    name: String,
    courses: Vec<Rc<RefCell<Course1>>>
}

impl Student1{
    fn new(name: &str) -> Student1 {
        Student1{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course1{
    name: String,
    students: Vec<Rc<RefCell<Student1>>>
}

impl Course1 {
    fn new(name: &str) -> Course1 {
        Course1 {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course1>>,
        student: Rc<RefCell<Student1>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

// APPROACH 3---------------------
// using Enrollment

struct Student2{
    name: String
}

struct Course2 {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student2,
    course: &'a Course2
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student2, course: &'a Course2) -> Enrollment<'a>
    {
        Enrollment {student, course}
    }
}


struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl Student2 {
    fn courses(&self, platform: Platform) -> Vec<String>{
        platform.enrollments.iter().filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self,
              student: &'a Student2,
              course: &'a Course2) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}


fn circular_reference() {
    // let john = Student::new("John");
    //
    // let course = Course::new("Rust course");
    // // if john is destroyed, course will pointed to empty reference
    // course.add_student(john); //Rc & refcell can solve this

    let john1 = Rc::new(
        RefCell::new (
            Student1:: new("John1")
        )
    );


    let jane = Rc::new(
        RefCell::new (
            Student1:: new("Jane")
        )
    );
    let course1 = Course1::new("Rust course");

    let magic_course = Rc::new(RefCell::new(course1));

    // course1.add_student(john1);

    Course1::add_student(magic_course.clone(), john1);
    Course1::add_student(magic_course, jane);



    let john2 = Student2 {
        name: "John".into()
    };

    let course2 = Course2 {
        name: "Intro to Rust".into()
    };

    let mut p = Platform::new();
    p.enroll(&john2, &course2);

    for c in john2.courses(p) {
        println!("John is taking {}", c);
    }
}


fn main() {
    println!("Hello, world!");

    circular_reference();
}
