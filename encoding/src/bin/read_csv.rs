use ansi_term::Colour::Red;
use csv::Error;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct RecordTab {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

#[derive(Serialize)]
struct Record2<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

fn main() -> Result<(), Error> {
    // let breakline = Red
    //     .paint("------------------------------------------------------------")
    //     .to_string();

    // 读取标准的CSV记录进入`csv::StringRecord`，这是一个弱类型数据读取
    println!("{}", Red.paint("读取CSV记录").to_string());
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} build the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }

    // 使用Serde反序列化数据进入强数据结构
    println!(
        "{}",
        Red.paint("使用Serde读取CSV记录到强数据结构").to_string()
    );
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";
    let mut reader2 = csv::Reader::from_reader(csv.as_bytes());
    for record in reader2.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} build the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }

    // 读取使用tab字符分割的记录
    println!("{}", Red.paint("读取tab分割的CSV记录").to_string());
    let data = "name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92";
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<RecordTab>() {
        println!("{:?}", result?);
    }

    // 条件查询
    println!("{}", Red.paint("查询符合条件的记录").to_string());
    let query = "AL";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;

    // 处理无效数据，会自动转换为None
    println!("{}", Red.paint("处理无效数据：").to_string());
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: RecordTab = result?;
        println!("{:?}", record);
    }

    // 序列化记录到CSV
    println!("{}", Red.paint("序列化记录到CSV").to_string());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;

    // 使用Serde序列化记录到CSV
    println!("{}", Red.paint("使用Serde序列化记录到CSV").to_string());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let rec1 = Record2 {
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record2 {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record2 {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())
}
