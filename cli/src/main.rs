use anyhow::Result;

#[derive(Debug)]
struct Args {
    path: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut path = None;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) if path.is_none() => {
                path = Some(val.into_string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        path: path.ok_or("missing argument PATH")?,
    })
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let args = parse_args()?;
    println!("{:?}", args);
    println!("{}", args.path);
    Ok(())
}
