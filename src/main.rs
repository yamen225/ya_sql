use db::{executor::ExecutionEngine, parser::Parser, query::QueryPlanner, schema::Row, storage_engine::{self, FileSystem}};

mod db;

fn main() {
    println!("Hello, world!");
    let mut file_system = FileSystem::new("database.db");
    file_system.create_table("users", vec!["id".to_string(), "name".to_string(), "age".to_string(), "email".to_string()]);
    file_system.insert_row("users", Row { 
        data:vec![
            ("id".to_string(), "1".to_string()),
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
            ("email".to_string(), "aliceàdummy.com".to_string()),
            ].into_iter().collect(),
    });
    let input = "SELECT * FROM users";
    let ast = Parser::parse(input).unwrap();
    let query_planner = QueryPlanner::new();
    let query_plan = query_planner.plan(&ast);



    let execution_engine = ExecutionEngine::new(file_system.storage_engine.clone());
    let result = execution_engine.execute(&query_plan).unwrap();
    println!("the result is {:?}", result);
}
