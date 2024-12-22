use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn to_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(b) => *b,
            FileSize::Kilobytes(kb) => (*kb * 1000.0) as u64,
            FileSize::Megabytes(mb) => (*mb * 1_000_000.0) as u64,
            FileSize::Gigabytes(gb) => (*gb * 1_000_000_000.0) as u64,
        }
    }

    fn from_str(size: &str, unit: &str) -> Option<Self> {
        let size_num = size.parse::<f64>().ok()?;
        
        match unit.to_lowercase().as_str() {
            "b" | "bytes" => Some(FileSize::Bytes(size_num as u64)),
            "kb" | "kilobytes" => Some(FileSize::Kilobytes(size_num)),
            "mb" | "megabytes" => Some(FileSize::Megabytes(size_num)),
            "gb" | "gigabytes" => Some(FileSize::Gigabytes(size_num)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl Sizes {
    fn from_filesize(filesize: FileSize) -> Self {
        let bytes = filesize.to_bytes();
        Sizes {
            bytes,
            kilobytes: bytes as f64 / 1000.0,
            megabytes: bytes as f64 / 1_000_000.0,
            gigabytes: bytes as f64 / 1_000_000_000.0,
        }
    }

    fn from_str(size: &str, unit: &str) -> Option<Self> {
        FileSize::from_str(size, unit).map(Self::from_filesize)
    }

    fn all_sizes(&self) -> String {
        format!(
            "{0: <15} {1: <15} {2: <15} {3: <15}\n\
             {4: <15} {5: <15.2} {6: <15.2} {7: <15.2}",
            "BYTES", "KB", "MB", "GB",
            self.bytes,
            self.kilobytes,
            self.megabytes,
            self.gigabytes
        )
    }
    // sizes as a string
    // fn all_sizes(&self) -> String {
    //     format!(
    //         "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{:.2} kilobytes\", megabytes: \"{:.2} megabytes\", gigabytes: \"{:.2} gigabytes\" }}", 
    //         self.bytes,
    //         self.kilobytes,
    //         self.megabytes,
    //         self.gigabytes
    //     )
    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the script name, the second is a string containing the user's "filesize size_unit"
    assert_eq!(args.len(), 2);
    println!("You entered: {}. Here's your conversion table...\n", args[1]);
    // parse the user's input into vector elements containing a size and a unit.
    let size_unit_vec: Vec<&str> = args[1].split_whitespace().collect();
    assert_eq!(size_unit_vec.len(), 2);
    // println!("Size unit is: {}. Unit is: {}.",size_unit_vec[0], size_unit_vec[1]);

    // GOAL: Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
    match Sizes::from_str(size_unit_vec[0], size_unit_vec[1]) {
        Some(sizes) => println!("{}", sizes.all_sizes()),
        None => println!("Error: Invalid size or unit. Please use format like '24 mb' or '1000 bytes'"),
    }
}
