//---------------------------------------------
// Iterators
//---------------------------------------------

// The iterator design pattern allows different types to have a common interface for accessing
// the elements of a type sequentially, while abstracting away how that iteration is implemented
// internally or how the type is laid down or defined interally.

// The Rust standard library has a trait called `Iterator` which any type can implement. The
// definition looks something like this.

// trait Iterator {
//    type Item;
//    fn next(&mut self) -> Option<Self::Item>;
//}

// It may be noted that the Iterator trait has many methods with default implementations. However,
// the next method has no default implementation and needs to be implemented explicitly.

struct Employee {
    name: String,
    salary: i32,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

// -----
// NOTE
// -----
// The basic philosphy of the `Iterator` trait is that the trait should have one mandatory function called
// `next`, which returns the next item. Moreover, the items returned must have the same type.

impl Iterator for Employee_Records {
    // We can return our own custom types by modifying the `Item` type and `next` method accordingly.
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40000,
    };

    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30000,
    };

    let mut emp_3 = Employee {
        name: String::from("Jack"),
        salary: 25000,
    };

    let mut emp_recs = Employee_Records {
        employee_db: vec![emp_1, emp_2, emp_3],
    };

    // Fetching records
    // println!("{:?}", emp_recs.next());
    // println!("{:?}", emp_recs.next());
    // println!("{:?}", emp_recs.next());
    // println!("{:?}", emp_recs.next()); // Should return `None` as we have only 3 records in the emp_recs

    println!();

    // Fetching records using loop. For this I have commented out the code using `next` method above as it is
    // iterates through all the indexes and the for loop cannot iterate over anything due to this.
    // The for loops knows how to handle the types that implement the `Iterator` trait. In the below
    // case although the employee database is not an iterator, but the loop is smart enough to turn it into
    // an iterator, producing items of type String, which is the type of variable employee.
    // The loop will automatically end, when the `None` variant is encountered. Furthermore, all the values
    // returned from the `next` method will be unwrapped.
    for employee in emp_recs {
        println!("{}", employee);
    }
}
