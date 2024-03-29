use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config:Config) -> Result<(),Box<Error>>{
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query,&contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query:&str, contents:&'a str)-> Vec<&'a str>{

    contents.lines().filter(|line| line.contains(query)).collect()

}

fn search_case_insensitive<'a>(query:&str, contents:&'a str)-> Vec<&'a str>{

    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()

}


pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String])-> Result<Config,&'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE")
            .is_err();

        Ok(Config{
            query:query.to_string(),
            filename:filename.to_string(),
            case_sensitive: case_sensitive,
        })
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = dbg!(" Rust:\nsafe, fast, productive.\nPick three.");

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = dbg!("Rust:\nsafe, fast, productive.\nPick three.\nTrust me.");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}