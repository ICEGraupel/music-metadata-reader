mod reader;

fn main() {
    println!("Start extracting tags");
    let result = reader::read_assets();
    reader::write_to_json(result);
}
