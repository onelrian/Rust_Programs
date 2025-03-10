#[derive(Debug)]
struct Employee {
    name: String,
    role: Role,
    is_active: bool,
}

#[derive(Debug)]
enum Role {
    Manager,
    Developer,
    Designer,
    Intern,
}

#[derive(Debug)]
enum Action {
    Hire,
    Terminate,
    Promote(Role), // Promote includes the new role
    Query,
}

fn perform_action(employee: &mut Employee, action: Action) -> Result<String, String> {
    match action {
        Action::Hire => {
            if !employee.is_active {
                employee.is_active = true;
                Ok(format!("{} has been hired as a {:?}", employee.name, employee.role))
            } else {
                Err(format!("{} is already active", employee.name))
            }
        }
        Action::Terminate => {
            if employee.is_active {
                employee.is_active = false;
                Ok(format!("{} has been terminated", employee.name))
            } else {
                Err(format!("{} is already inactive", employee.name))
            }
        }
        Action::Promote(new_role) => {
            employee.role = new_role;
            Ok(format!("{} has been promoted to {:?}", employee.name, employee.role))
        }
        Action::Query => {
            let status = if employee.is_active { "Active" } else { "Inactive" };
            Ok(format!("{} is {} as a {:?}", employee.name, status, employee.role))
        }
    }
}

fn find_employee_by_name<'a>(employees: &'a mut Vec<Employee>, name: &'a str) -> Option<&'a mut Employee> {
    employees.iter_mut().find(|employee| employee.name == name)
}

fn main() {
    let mut employees = vec![
        Employee {
            name: "Alice".to_string(),
            role: Role::Developer,
            is_active: false,
        },
        Employee {
            name: "Bob".to_string(),
            role: Role::Manager,
            is_active: true,
        },
    ];

    // Perform actions
    if let Some(employee) = find_employee_by_name(&mut employees, "Alice") {
        println!("{:?}", perform_action(employee, Action::Hire));
    }

    if let Some(employee) = find_employee_by_name(&mut employees, "Bob") {
        println!("{:?}", perform_action(employee, Action::Promote(Role::Manager)));
    }

    if let Some(employee) = find_employee_by_name(&mut employees, "Alice") {
        println!("{:?}", perform_action(employee, Action::Query));
    }

    // Handle non-existent employee
    if let Some(employee) = find_employee_by_name(&mut employees, "Charlie") {
        println!("{:?}", perform_action(employee, Action::Query));
    } else {
        println!("Employee not found in the system.");
    }
}