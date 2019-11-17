extern crate arrow;
extern crate tokio_reactor;

use arrow::csv::Writer;
use arrow::array::Int32Array;
use arrow::util::string_writer::StringWriter;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let eventloop = tokio_reactor::Reactor::new()?;
    println!("{:?}", eventloop);
    
    let data = Int32Array::from(vec![1, 2, 3, 4, 5]);
    println!("{:?}", data);
    
    let writer = Writer::new(StringWriter::new());
    println!("{:p}", &writer);
    
    Ok(())
}
