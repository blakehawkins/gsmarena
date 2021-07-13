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
    #[structopt(short = "b", long = "battery", default_value = "3300")]
    min_battery_capacity: u32,

    #[structopt(short = "y", long = "year", default_value = "2020")]
    min_year: u32,

    #[structopt(short = "o", long = "os_query")]
    os_query: Option<String>,

    #[structopt(short = "h", long = "header")]
    print_headers: bool,
}

static GSMARENA_URI: &'static str = "https://www.gsmarena.com/";

// New phones with battery capacity like Pixel 3 XL or better.  Use macro-rules for inline fmtstring.
macro_rules! gsmarena_query_fmtstring {
    () => {
        "\
        https://www.gsmarena.com/results.php3\
        ?nYearMin={}&nIntMemMin=40000&nDisplayResMin=2527200\
        &fDisplayInchesMin=6.0&fDisplayInchesMax=7.0&chkGPS=selected\
        &chkNFC=selected&chkUSBC=selected&nBatCapacityMin={}\
        &sMakers=59,5,88,76,28,48,90,46,57,15,31,42,34,36,116,10,108,77,75,\
        24,105,61,104,93,106,2,40,50,65,47,92,107,33,41,45,35,52,69,119,29,60,\
        102,122,84,83,17,94,109,73,20,14,87,74,66,64,25,8,63,4,56,12,22,79,1,\
        97,30,71,27,6,32,81,11,72,101,86,103,38,117,118,13,9,18,26,23,3,7,\
        19,68,55,120,21,16,49,44,91,39,70,98,37,53,96,51,43,85,78,99,100\
        &sAvailabilities=1,2,3,5&sFormFactors=1&sOSes=2,3&sDisplayTechs=1,2\
        &idTouchscreen=1&nOrder=1"
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    if opt.print_headers {
        println!("Name\tBattery Capacity\tOS\tReleased\tPrice\tScreensize");
        return Ok(());
    }

    let _ = Document::from(
        &reqwest::get(&format!(
            gsmarena_query_fmtstring!(),
            opt.min_year, opt.min_battery_capacity
        ))?
        .text()?[..],
    )
    .find(Class("makers").descendant(Name("a")))
    .into_iter()
    .filter_map(|a| a.attr("href"))
    .map(|u| -> Result<(), Box<dyn std::error::Error>> {
        // println!("{}", u);

        let doc = Document::from(&reqwest::get(&format!("{}{}", GSMARENA_URI, u))?.text()?[..]);

        let name = doc
            .find(Class("specs-phone-name-title"))
            .into_iter()
            .next()
            .map(|s| s.text());

        let bat = doc
            .find(Name("strong").descendant(Attr("data-spec", "batsize-hl")))
            .into_iter()
            .next()
            .map(|s| s.text());

        let os_string = doc
            .find(Class("specs-brief-accent").descendant(Attr("data-spec", "os-hl")))
            .into_iter()
            .next()
            .map(|s| s.text());

        let release_string = doc
            .find(Class("specs-brief-accent").descendant(Attr("data-spec", "released-hl")))
            .into_iter()
            .next()
            .map(|s| s.text());

        let price_string = doc
            .find(Class("nfo").and(Attr("data-spec", "price")))
            .into_iter()
            .next()
            .map(|s| s.text());

        let screen_size = doc
            .find(Attr("data-spec", "displaysize-hl"))
            .into_iter()
            .next()
            .map(|s| s.text());

        if os_string
            .clone()
            .unwrap_or("".into())
            .contains(&opt.os_query.clone().unwrap_or("".into()))
        {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}",
                name.unwrap_or("(Unknown)".into()),
                bat.unwrap_or("(Unknown)".into()),
                os_string.unwrap_or("(Unknown)".into()),
                release_string.unwrap_or("(Unknown)".into()),
                price_string.unwrap_or("(Unknown)".into()),
                screen_size.unwrap_or("(Unknown)".into())
            );
        }

        Ok(())
    })
    .collect::<Vec<_>>();

    Ok(())
}
