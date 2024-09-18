extern crate histo;
extern crate reqwest;
extern crate select;
extern crate structopt;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "gsmdata")]
struct Opt {
    #[structopt(short = "b", long = "battery", default_value = "4200")]
    min_battery_capacity: u32,

    #[structopt(short = "y", long = "year", default_value = "2023")]
    min_year: u32,

    #[structopt(short = "h", long = "header")]
    print_headers: bool,

    #[structopt(short = "q", long = "query")]
    query: Option<String>,
}

static GSMARENA_URI: &str = "https://www.gsmarena.com/";

macro_rules! gsmarena_query_fmtstring {
    () => {
        "\
        https://www.gsmarena.com/results.php3\
        ?nYearMin={}\
        &nRamMin=8000\
        &nDisplayResMin=2073600\
        &fDisplayInchesMin=5\
        &nDisplayFramesMin=90\
        &nBatCapacityMin={}\
        &nChargingWMin=1\
        &sMakers=46,107,133,45,4,7\
        &sFormFactors=1\
        &idQwerty=2\
        &nUSBType=1"
    };
}

fn scrape_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = Document::from(&reqwest::get(url)?.text()?[..])
        .find(Class("makers").descendant(Name("a")))
        .filter_map(|a| a.attr("href"))
        .map(|u| -> Result<(), Box<dyn std::error::Error>> {
            // println!("{}", u);

            let doc = Document::from(&reqwest::get(&format!("{}{}", GSMARENA_URI, u))?.text()?[..]);

            let name = doc
                .find(Class("specs-phone-name-title"))
                .next()
                .map(|s| s.text());

            let bat = doc
                .find(Name("strong").descendant(Attr("data-spec", "batsize-hl")))
                .next()
                .map(|s| s.text());

            let os_string = doc
                .find(Class("specs-brief-accent").descendant(Attr("data-spec", "os-hl")))
                .next()
                .map(|s| s.text());

            let release_string = doc
                .find(Class("specs-brief-accent").descendant(Attr("data-spec", "released-hl")))
                .next()
                .map(|s| s.text());

            let screen_size = doc
                .find(Attr("data-spec", "displaysize-hl"))
                .next()
                .map(|s| s.text());

            let ram = doc
                .find(Name("strong").descendant(Attr("data-spec", "ramsize-hl")))
                .next()
                .map(|s| s.text());

            let chipset = doc
                .find(Attr("data-spec", "chipset-hl"))
                .next()
                .map(|s| s.text());

            let resolution = doc
                .find(Attr("data-spec", "displayres-hl"))
                .next()
                .map(|s| s.text());

            println!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                bat.unwrap_or("(Unknown)".into()),
                screen_size.unwrap_or("(Unknown)".into()),
                ram.unwrap_or("(Unknown)".into()),
                chipset.unwrap_or("(Unknown)".into()),
                resolution.unwrap_or("(Unknown)".into()),
                name.unwrap_or("(Unknown)".into()),
                release_string.unwrap_or("(Unknown)".into()),
                os_string.unwrap_or("(Unknown)".into()),
            );

            Ok(())
        })
        .collect::<Vec<_>>();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    if opt.print_headers {
        println!("Battery Capacity\tScreensize\tRAM\tChipset\tResolution\tName\tReleased\tOS");
        return Ok(());
    }

    let address = opt.query.unwrap_or(format!(
        gsmarena_query_fmtstring!(),
        opt.min_year, opt.min_battery_capacity
    ));

    let _ = scrape_url(&address);

    Ok(())
}
