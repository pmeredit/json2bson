use std::io::{self, Read, Error, ErrorKind};
use serde_json::Value;
use bson::{Bson, encode_document};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
    } // release lock on stdin

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&buffer)?;
    let stdout = io::stdout();

    match v {
        o@Value::Object(_) => {
            let serialized = Bson::from(o);
            match serialized {
                Bson::Document(doc) => {
                    let mut stdout_handle = stdout.lock();
                    encode_document(&mut stdout_handle, &doc).unwrap();
                }
                _ => (),
            }
            Ok(())
        }
        Value::Array(ref a) => {
            for e in a.into_iter() {
                let serialized = Bson::from(e.to_owned());
                match serialized {
                    Bson::Document(doc) => {
                        let mut stdout_handle = stdout.lock();
                        encode_document(&mut stdout_handle, &doc).unwrap();
                    }
                    _ => (),
                }
            }
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::Other, "json must be object or array"))
    }
}
