use std::io;
use std::error::Error;
use std::fs;

fn find_valorant_path(file_name: &String) -> Result<String, Box<dyn Error>>
{
    // read the path from file and return it
    let mut path = fs::read_to_string(&file_name)?;
    let default_path = String::from("C:\\Riot Games\\VALORANT\\live\\ShooterGame\\Content\\Paks");
    // if file was just created it means no path is found yet
    if path.len() == 0
    {   
        // if default path exists, write the path into path.rxr and return it
        if std::path::Path::new(&default_path).exists()
        {
            fs::write(&file_name, &default_path)?;
            return Ok(default_path);

        }
        // otherwise let the user fill in the path
        else 
        {
            std::io::stdin().read_line(&mut path).expect("Incorrect path");
            fs::write(&file_name, &path)?;
            return Ok(path);
        }

    }

    // otherwise return the path from file
    return Ok(path);
}

// checks to see if path exists, if it doesnt it creates it
// and return the path to valorant Paks folder
fn get_path() -> Result<String, Box<dyn Error>>
{
    // file name
    let file_name = String::from("path.txt");

    // if file doesnt exits create it and write the valorant path to it
    if !std::path::Path::new(&file_name).exists()
    {
        fs::write(&file_name, "")?;
        return find_valorant_path(&file_name);
    }
    
    return find_valorant_path(&file_name);
}

fn move_files(valorant_path: &Result<String, Box<dyn Error>>) -> Result<(), Box<dyn Error>>
{
    let path = valorant_path.as_ref().unwrap();

    match fs::copy("./Paks/jp/ja_JP_Text-WindowsClient.pak", path.to_owned() + "\\ja_JP_Text-WindowsClient.pak")
    {
        Err(e) => println!("{:?}", e),
        _ => println!("Moved pak")

    }
    match fs::copy("./Paks/jp/ja_JP_Text-WindowsClient.sig", path.to_owned() + "\\ja_JP_Text-WindowsClient.sig")
    {
        Err(e) => println!("{:?}", e),
        _ => println!("Moved sig")
    }


    return Ok(());

}

fn main()
{
    let valorant_path = get_path();
    move_files(&valorant_path);
    println!("Press any key to close this window.");
    io::stdin().read_line(&mut String::new()).unwrap();
    // println!("{:?}", valorant_path.unwrap())

}