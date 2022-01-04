use structopt::StructOpt;
use utf8_chars::BufReadCharsExt;
use std::io::prelude::*;


#[derive(StructOpt, Debug)]
#[structopt(name = "boredsplit", about = "CSS splitter aligner")]
pub struct ProgramArguments
{
    #[structopt(short, long, default_value = "*.css")]
    file     : String,

    #[structopt(short, long, default_value = "4")]
    ident    : u8,

    #[structopt(short, long, default_value = "2")]
    lmarging : u8,

    #[structopt(short, long, default_value = "2")]
    rmarging : u8,
}


fn align_block(block: &str, arguments:&ProgramArguments) -> String 
{
    let block = block.strip_prefix('{').unwrap()
                          .strip_suffix('}').unwrap();
    
    let lines = block.split(';')
                                             .into_iter()
                                             .filter(|line| !line.trim().is_empty());
    
    let mut lefts  = Vec::new();
    let mut rights = Vec::new();
    
    let mut max_line_length = 0;
    
    for line in lines
    {
        let parts:Vec<&str> = line.split(':').collect();
        
        let left_part  = parts[0].trim();
        let right_part = parts[1].trim();
        
        lefts.push(left_part);
        rights.push(right_part);
        
        max_line_length = left_part.len().max(max_line_length); 
    }
                                
    let mut new_block = String::from("{\n");
    
    
    // Yea, I know it's very ugly. Left it as TODO
    for (left_part, right_part) in lefts.iter().zip(rights)
    {
        new_block.push_str(&" ".repeat(arguments.ident as usize));
        new_block.push_str(left_part);
        new_block.push_str(&" ".repeat(max_line_length - left_part.len()));
        new_block.push_str(&" ".repeat(arguments.lmarging as usize));
        new_block.push(':');
        new_block.push_str(&" ".repeat(arguments.rmarging as usize));
        new_block.push_str(&format!("{};\n", right_part));
    }
    
    new_block += "}";      
                                
    new_block
}

fn write_block(block: &str, file: &mut std::fs::File, arguments: &ProgramArguments) -> Result<(), std::io::Error>
{
    let new_block = align_block(block, arguments);
    file.write_all(&new_block.as_bytes().to_vec())?;
    Ok(())
}

fn align_file(file: &std::path::Path, arguments: &ProgramArguments) -> Result<(), std::io::Error> 
{
    let orig = std::fs::File::open(file)?;
    
    let tmp_filename = format!("{}.css", uuid::Uuid::new_v4().to_string());
    
    let mut aligned_file = std::fs::OpenOptions::new()
                                                 .create(true)
                                                 .write(true)
                                                 .truncate(true)
                                                 .open(&tmp_filename)?;
    
    let mut block           = String::new();
    let mut is_reading_block = false;
    
    for character in std::io::BufReader::new(orig).chars()
    {
        let character = character?;
        
        if is_reading_block
        {
            block.push(character);
            
            if character == '}'
            {  
                is_reading_block = false;
                write_block(&block, &mut aligned_file, arguments)?;
                block.clear();
            }
        }
        else if character == '{'
        {
            is_reading_block = true;
            block.clear();
            block.push(character);
        }
        else
        {
            aligned_file.write_all(&[{character as u8}])?;
        }
 
    }
    
    std::fs::remove_file(&file)?;
    std::fs::rename(&tmp_filename, 
                         file.to_path_buf())?;
    
    Ok(())
}

fn main()
{
    let arguments = ProgramArguments::from_args();
    
    let pattern   = match glob::glob(&arguments.file)
    {
        Ok(pattern) => pattern,
        Err(e) => 
        {
            println!("Failed to compile pattern! Reason: {}", e.to_string());
            return;
        }
    };
    
    for entry in pattern
    {
        let file = match entry
        {
            Ok(file) => file,
            Err(e) =>
            {
                println!("Failed to glob in files! Reason: {}", e.to_string());
                return;
            }
        };
        
        println!("Aligning file: {:?}", file);
        
        match align_file(&file, &arguments)
        {
            Ok(()) => {
                        println!("File was succesfully aligned!")
            },
            Err(err) => {
                    println!("Failed to align_file! Reason: {}", err.to_string());
            }
        }
    }
}
