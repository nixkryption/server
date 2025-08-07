use tokio::io;
mod app_config;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Load env from file via config
    let config = app_config::app_config::config();

    println!("Config: {:?}", config.debug);

    // This should have order management and data management functions, we can run this on 2
    // different processes which are async in nature
    let pid_order_management = start_order_management().await?;
    let pid_data_management = start_data_management().await?;

    println!("{}", pid_order_management);
    println!("{}", pid_data_management);

    Ok(())
}

async fn start_order_management() -> io::Result<i32> {
    println!("order management");
    Ok(32576)
}

async fn start_data_management() -> io::Result<i32> {
    println!("data management");
    Ok(32577)
}
