use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
   
    match args.first().map(|s| s.as_str()){
        Some("-concatenate") => 
        { 
            println!("{}", concatenate(&args[1..]));
        }
        Some("-mult") => 
        {
            match mul_cli(&args[1..])
            {
            Ok(total) => println!("Resultado: {}", total),
            Err(e) => println!("Error: {}", e),
        }
    }
    Some("-add") => {
        match add_cli(&args[1..]){
            Ok(total) => println!("Resultado: {}", total),
            Err(e) => println!("Error: {}", e),
        }
    }
    Some("-sub") => {
        match sub_cli(&args[1..])
        {
            Ok(total) => println!("Resultado: {}", total),
            Err(s) => println!("Error: {}", s),   
        }
    }
    Some("-div") => { 
        match div_cli(&args[1..]){
            Ok(total) => println!("Resultado: {}", total),
            Err(e) => println!("Error: {}", e),
        }
    }
    Some("-echo") => { println!("{}", echo_cli(&args[1..]));}
    
    Some("-wc") => {
        match wc_cli(&args[1..])
        {
            Ok(s) => println!("Resultado: {}", s),
            Err(e) => println!("Error: {}", e),

        }
    }
    Some("-ef") => {
        match echo_file(&args[1..])
        {
            Ok(s) => println!("Resultado: {}",s),
            Err(e) => println!("Error: {}",e),
        }
    }
    Some("-odd") => {
        match odd_cli(&args[1..])
        {
            Ok(s) => println!("Resultado: {}",s),
            Err(e) => println!("Error: {}", e),
        }

    }
    Some("-equal") => {
        match equal_cli(&args[1..])
        {
            Ok(s) => println!("Resultado: {}",s),
            Err(e) => println!("Error: {}",e),
        }
    }
    Some("-stat") => {
        match stat_cli(&args[1..]){
            Ok(s) => println!("{}",s),
            Err(e) => println!("Error: {}",e),

        }
    }
    _=> {
        println!("Error: operacion no valida!");
    }
    }
}
 
fn echo_file(args: &[String]) -> Result<String,String>
{
    let filename = args.get(0).ok_or("Falta el nombre del archivo")?;
    let buscar = args.get(1).ok_or("Falta el argumento 1")?;
    let nuevo = args.get(2).ok_or("Falta el argumento 2")?;
    let content = fs::read_to_string(filename).map_err(|_| "No se puede leer el archivo")?;
    if content.split_whitespace().any(|s| s == buscar){
        fs::write(filename, nuevo).map_err(|_| "No se pudo escribir en el archivo")?;

        Ok("Archivo escrito con exito!".to_string())
    }else{
        Err("Palabra no encontrada".to_string())
    }
}

fn wc_cli(args: &[String]) -> Result<String,String>{
    let filename = args.get(0).ok_or("Falta el nombre del archivo")?;
    let content = fs::read_to_string(filename).map_err(|_| "No se pudo leer el archivo")?;
    let lineas = content.lines().count();
    let palabras = content.split_whitespace().count();
    let bytes = content.as_bytes().len();
    Ok(format!("Archivo: {} Lineas: {}  Palabras: {} Bytes: {}",filename, lineas, palabras, bytes))
}
fn div_cli(args: &[String]) -> Result<i32, String> {
    let mut iter = args.iter();

    let mut total: i32 = iter
        .next()
        .ok_or("Faltan argumentos")?
        .parse()
        .map_err(|_| "El primer argumento no es un número".to_string())?;

    for arg in iter {
        let n: i32 = arg
            .parse()
            .map_err(|_| format!("'{}' no es un número", arg))?;

        if n == 0 {
            return Err("División por cero".to_string());
        }

        total /= n;
    }

    Ok(total)
}


fn concatenate(args: &[String]) -> String{
    args.join("")
}

fn echo_cli(args: &[String]) -> String{

    let no_newline = matches!(args.first().map(|s| s.as_str()), Some("-n"));

  if no_newline {
        args[1..].join(" ")
    } else {
        args.join(" ")
    }
}
  
fn add_cli(args: &[String]) -> Result<i32,String>{
    let mut total = 0;
    args.first().ok_or("Falta un argumento")?;
    for arg in args{
        let n: i32 = arg.parse().map_err(|_| format!("'{}' no es un numero",arg))?;
        total+=n;
    }
    Ok(total)
}

fn mul_cli(args: &[String]) -> Result<i32,String>{
    let mut total = 1;
    args.first().ok_or("Falta un argumento")?;
    for arg in args{
        let n: i32 = arg.parse().map_err(|_| format!("'{}' no es un numero",arg))?;
        total*=n
    }
    Ok(total)
}

fn sub_cli(args: &[String]) -> Result<i32, String>{
    let mut iter = args.iter();
    let mut total: i32 = iter
        .next()
        .ok_or("Faltan argumentos")?
        .parse()
        .map_err(|_| "El primer argumento no es un número".to_string())?;
    for arg in iter{
        let n: i32 = arg.parse().map_err(|_| format!("'{}' no es un numero",arg))?;
        total -= n;
    }
    Ok(total)
}

fn odd_cli(args: &[String]) -> Result<String,String>{
    let n: i32 = args.get(0).ok_or("Faltan argumentos")?
        .parse()
        .map_err(|_| "El primer argumento no es un número".to_string())?;
    if n % 2 == 0
    {
        Ok(format!("{} es un numero par",n))
    }
    else{
        Ok(format!("{} es un numero impar",n))
    }

}

fn equal_cli(args: &[String]) -> Result<String,String>{
    let val_a = args.get(0).ok_or("Argumento A requerido")?;
    let val_b = args.get(1).ok_or("Argumento B requerido")?;

    if val_a == val_b {
        Ok("Ambas expresiones son iguales".to_string())
    }
    else{
        Ok("Ambas expresiones son distintas".to_string())
    }
}

fn stat_cli(args: &[String]) -> Result<String,String>{
    let filename = args.get(0).ok_or("Falta el nombre del archivo")?;
    let m = fs::metadata(filename).map_err(|_| "No se puede leer el archivo")?;
    Ok(format!("Informacion del archivo: {}\nSize: {} bytes\nMode: {:o}\nUID: {}\nGID: {}\nInode: {}",filename, m.len(), m.mode(),m.uid(),m.gid(),m.ino()))
}