use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct EmployeeInfo
{
    name: String,
    department: String,
}


fn main() 
{
    let mut employee_directory: HashMap<String, String> = HashMap::from(
        [
            ("Kate".to_string(),"Sales".to_string()),
            ("Andrew".to_string(),"Sales".to_string()),
            ("Morgan".to_string(),"Sales".to_string()),
            ("Alice".to_string(),"Engineering".to_string()),
            ("Bob".to_string(),"Engineering".to_string()),
            ("Liam".to_string(),"Engineering".to_string()),
        ]
    );

    let new_employee = get_new_employee();

    // add_to_directory(&mut employee_directory, new_employee);
    employee_directory.insert(new_employee.name, new_employee.department);
  
    let sales_team = get_department_employees(&employee_directory, &String::from("Sales"));

    println!("{:?}",sales_team)
}

fn get_new_employee() -> EmployeeInfo
{
    println!("Enter Employee Name");
    let mut name_input = String::new();
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read line");

    println!("Enter Department");
    let mut department_input = String::new();
    io::stdin()
        .read_line(&mut department_input)
        .expect("Failed to read line");

    let new_employee = EmployeeInfo
    {
        name: String::from(name_input.trim()),
        department: String::from(department_input.trim()),
    };
    return new_employee;
}


fn get_department_employees(directory: &HashMap<String, String>,department: &String) -> Vec<String>
{
    let mut list_names: Vec<String> = Vec::new();
    for (key, value) in directory
    {
        if value.eq(department)
        {
            list_names.push(key.clone());
        }
    }
    list_names.sort();
    return list_names;
}