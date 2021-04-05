use io::Read;
use std::{
    io,
    process::{Command, ExitStatus},
};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use winreg;

#[derive(Debug)]
struct ProductInfo {
    guid: String,
    name: String,
}

fn run() -> std::io::Result<()> {
    {
        let term = Term::stdout();
        term.write_line("SOLIDWORKS REPAIR UTILITY")?;
        term.write_line("")?;
        term.write_str("Loading products...")?;

        let sw_products: Vec<_> = get_installed_products()?
            .into_iter()
            .filter(|p| p.name.contains("SOLIDWORKS"))
            .collect();

        term.clear_line()?;
        term.write_line("SELECT PRODUCT TO REPAIR:")?;

        if let Some(product) = select_product(&sw_products) {
            term.write_line(&format!("Repairing: {}", &product.name))?;
            if let Ok(status) = repair(&product.guid) {
                if status.success() {
                    term.write_line("Success!")?;
                } else {
                    term.write_line(&format!("There was a problem... exit status = {}", status))?;
                }
            }
        } else {
            term.write_line("Aborting.")?;
        }
    }

    println!(" - Press enter to exit - ");

    let mut buf = [0u8; 1];
    std::io::stdin().lock().read_exact(&mut buf)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Unexpected error: {:?}.\n\nPress enter to exit.\n\n", e);

        let mut buf = [0u8; 1];
        std::io::stdin().lock().read_exact(&mut buf).unwrap();
    }
}

fn get_installed_products() -> std::io::Result<Vec<ProductInfo>> {
    //let base64 = r"Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall";
    let mut res = vec![];
    let base = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey(base)?;

    let mut key_path = String::new();
    for kname in cur_ver.enum_keys().map(|x| x.unwrap()) {
        key_path.clear();
        key_path.push_str(base);
        key_path.push_str("\\");
        key_path.push_str(&kname);

        if let Ok(subkey) = hklm.open_subkey(&key_path) {
            if let Ok(disp_name) = subkey.get_value::<String, _>("DisplayName") {
                res.push(ProductInfo {
                    guid: kname,
                    name: disp_name,
                })
            }
        }
    }
    Ok(res)
}

fn select_product(products: &[ProductInfo]) -> Option<&ProductInfo> {
    let items: Vec<_> = products.iter().map(|p| &p.name).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .ok()?;

    selection.map(|idx| products.get(idx)).flatten()
}

fn repair(guid: &str) -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new("msiexec.exe");
    cmd.arg("/f");
    cmd.arg(guid);

    cmd.status()
}
