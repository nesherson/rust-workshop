struct Student {
    name: String,
    age: u8,
    grades: [u8; 5]
}

impl Student {
    fn average_grade(&self) -> f32 {
        let mut sum: f32 = 0.0;

        for grade in self.grades {
            sum += grade as f32;
        }

        sum / self.grades.iter().count() as f32
    }

    fn display(&self) {
        println!("Student name: {}", self.name);
        println!("Student age: {}", self.age);
        println!("Student average grade: {}", self.average_grade());
    }
}

fn main() {
    let student1 = Student {
        name: String::from("Adam Testerson"),
        age: 22,
        grades: [4, 5, 5, 4, 3]
    };

    student1.display();
}