use clap::Parser;
use tide::Request;
use std::{thread, net::SocketAddr};

/// NAC PXE (TFTP & HTTP) server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the directory to serve
    #[arg(short, long, default_value = "./public")]
    filepath: String,

    /// Port to listen on for HTTP
    #[arg(short, long, default_value_t = 4433)]
    port: u16,

    /// Path for boot menu script
    #[arg(short, long, default_value = "/Altiris/iPXE/GetPxeScript.aspx")]
    menu: String,

    /// Bind IP address for TFTP & HTTP
    #[arg(short, long, default_value = "0.0.0.0")]
    bind: String,
}

#[async_std::main]
async fn  main() -> tide::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // TFTP server (using tftpd)
    let mut config = tftpd::Config::default();
    config.directory = args.filepath.parse()?;
    config.ip_address = args.bind.parse()?;
    thread::spawn(move || {
        let mut server = tftpd::Server::new(&config).unwrap_or_else(|err| {
            eprintln!(
                "Problem creating TFTP Server on {}:{}: {err}",
                config.ip_address, config.port
            );
            std::process::exit(1);
        });
        println!(
            "Running TFTP Server on {} in {}",
            SocketAddr::new(config.ip_address, config.port),
            config.directory.display()
        );
        server.listen();
    });

    // HTTP server (using tide)
    let mut app = tide::new();
    app.at(&*args.menu).get(get_pxe);
    app.at(&*args.menu).post(get_pxe);
    app.at("/AuthPxe").get(auth_pxe);
    // Return other files from the directory as static files
    app.at("/").serve_dir(args.filepath)?;
    println!("Running HTTP Server on {}:{}", args.bind, args.port);
    app.listen(format!("{}:{}",args.bind, args.port)).await?;
    Ok(())
}

// Example using pin for authentication
async fn get_pxe(_req: Request<()>) -> tide::Result {
    Ok("#!ipxe\n\necho -n Please provide a pin:\nread pin\nchain http://${next-server}:4433/AuthPxe?pin=${pin}&asset=${asset}&mac=${net0/mac:hexhyp}&serial=${serial}&manufacturer=${manufacturer}&product=${product} || shell".into())
}

async fn auth_pxe(req: Request<()>) -> tide::Result {
    let url = req.url();
    let pin = url.query_pairs().find(|(key, _)| key == "pin").map(|(_, value)| value);
    let asset = url.query_pairs().find(|(key, _)| key == "asset").map(|(_, value)| value);
    let mac = url.query_pairs().find(|(key, _)| key == "mac").map(|(_, value)| value);
    let serial = url.query_pairs().find(|(key, _)| key == "serial").map(|(_, value)| value);
    let manufacturer = url.query_pairs().find(|(key, _)| key == "manufacturer").map(|(_, value)| value);
    let product = url.query_pairs().find(|(key, _)| key == "product").map(|(_, value)| value);

    if pin.is_none() || asset.is_none() || mac.is_none() || serial.is_none() || manufacturer.is_none() || product.is_none() {
        return Ok("Missing parameters".into());
    }

    //Run a Powershell script
    let output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg( format!(".\\AuthPxe.ps1 -pin {} -Asset {} -Mac {} -Serial {} -Manufacturer {} -Product {}", pin.unwrap(), asset.unwrap(), mac.unwrap(), serial.unwrap(), manufacturer.unwrap(), product.unwrap()))
        .output()
        .expect("Failed to execute command");

    Ok(String::from_utf8(output.stdout)?.into())
}