use base64::encode_config_buf;
use std::fs::File;
use std::io::{prelude::*,self,Write};
use std::path::Path;
fn main() {
    // --snip--
    println!("insert your file path:");
    let mut filename = String::new();

    io::stdin().read_line(&mut filename).unwrap().to_string();
    filename = filename.to_string().trim().to_string();
    let mut file = File::open(&filename).expect("no file");
    println!("file {} Opened!", filename);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("can not read the file");
    let mut buf = String::new();
    encode_config_buf(buffer, base64::STANDARD, &mut buf);
     let mut fout = File::create(&Path::new(&format!("{}.b64decoded",filename))).unwrap();
     fout.write_all(&buf.as_bytes()).expect("cannot write the file");
     println!("file saved! in {}.b64decoded",filename );
}
