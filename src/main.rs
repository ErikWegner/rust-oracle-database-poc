use std::env;
use oracle::{Connection, Result, Version};

fn main() -> Result<()> {

    let svc = match env::var_os("SVC_NAME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$SVC_NAME is not set")
    };
    println!("\nService Name is {}", svc);

    let client_ver = Version::client()?;
    println!("\nOracle Client Version: {}", client_ver);

    let conn = Connection::connect("rustapp", "kuMxNAZ2xYSh", svc)?;
    let (server_ver, banner) = conn.server_version()?;

    println!("\nDatabase Server Version: {}", server_ver);
    println!("\nServer Banner: {}\n", banner);

    let sql = "select * from persons order by PersonID asc";
    let mut stmt = conn.statement(sql).build()?;
    let rows = stmt.query(&[])?;

    // Get the column names
    for info in rows.column_info() {
       print!("{} ", info.name())
    }
    println!();

    // Display the resultset
    for row_result in rows {
        // print column values
        for (idx, val) in row_result?.sql_values().iter().enumerate() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", val);
        }
        println!();
    }
    conn.close()?;

    println!("\nBye");
    Ok(())
}
