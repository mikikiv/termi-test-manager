use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

struct TestCase {

    title: String,
    steps: String,
    expected_result: String,
    status: TestCaseStatus,
    added_date: String,
}

#[derive(Debug)]
enum TestCaseStatus {
    Open,
    Passed,
    Failed,
    Blocked,
}

impl ToString for TestCase {
    fn to_string(&self) -> String {
        // define how the struct should be converted to a String
        // e.g. concatenate the field values
        format!("{} - {} - {}", self.title, self.steps, self.expected_result)
    }
}

fn create_tests (
    title: String,
    steps: String,
    expected_result: String,
) -> TestCase {
    TestCase {
        title,
        steps,
        expected_result,
        status: TestCaseStatus::Open,
        added_date: String::from("2021-09-01"),
    }
}

// Function to read tasks from a file
fn read_tests (
    file_name: String
) -> String {
    let mut file = File::open(file_name).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents
}

// Function to write tasks to a file (create or overwrite) (tests.json)
fn write_tests(
    file_name: String,
    contents: String
) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(contents.as_bytes()).expect("Unable to write file");
}






fn main() {

    
    let test1 = create_tests(
        "can login".to_string(), 
        "complete teh login fields, then submit".to_string(), 
        "user is logged in".to_string(),  
    );

    let test1_string = test1.to_string();

    // write the test to the file
    write_tests("tests.json".to_string(), test1_string);


    // read tests from a file
    let tests = read_tests("tests.json".to_string());
    println!("tests: {}", tests);

}
