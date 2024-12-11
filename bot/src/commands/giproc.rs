use poise::samples::on_error;
use poise::serenity_prelude::OnlineStatus;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::*;

pub struct ReqResA {
    _fullname: String,
    ecofriendliness: f64,
    weather: f64,
    lifespan: f64,
    transport: f64,
    tourism: f64,
    envbeauty: f64,
}

pub struct ReqResB {
    _fullname: String,
    economy: f64,
    compassion: f64,
    weather: f64,
    obesity: f64,
    secularism: f64,
    foodquality: f64,
}

#[derive(Debug)]
pub struct Passed {
    pub fullname: String,
    pub index: f64,
    pub status: String,
}

struct RawResponse {
    body: String,
    header: String,
}

#[derive(PartialEq)]
pub enum Index {
    Green,
    Fest,
}

pub async fn gindexcalc(nation: String, index: Index) -> Result<Passed> {
    let rval = fetch(nation, &index).await?;
    if rval.header != "200 OK" {
        let off = Passed {
            fullname: String::from("No!"),
            index: 54.34354,
            status: rval.header,
        };
        return Ok(off);
    }
    let mut reader = Reader::from_str(&rval.body);
    reader.config_mut().trim_text(true);

    let mut _count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => _count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("{:?}", txt);
    let on;
    if index == Index::Green {
        let vals = ReqResA {
            _fullname: txt[0].clone(),
            ecofriendliness: txt[1].parse::<f64>().expect("REASON"),
            weather: txt[2].parse::<f64>().expect("REASON"),
            lifespan: txt[3].parse::<f64>().expect("REASON"),
            transport: txt[4].parse::<f64>().expect("REASON"),
            tourism: txt[5].parse::<f64>().expect("REASON"),
            envbeauty: txt[6].parse::<f64>().expect("REASON"),
        };
        let greenindex = (vals.envbeauty / 22500.0) * 0.75
            + (vals.weather / 4200.0) * 0.25
            + (vals.tourism / 10000.0) * 0.25
            + (vals.ecofriendliness / 60000.0) * 0.2
            + ((vals.lifespan / 105.0) * (vals.lifespan / 105.0)) * 0.1
            + (vals.transport / 28000.0) * 0.05;
        println!("{}", greenindex);
        on = Passed {
            fullname: vals._fullname,
            index: greenindex,
            status: rval.header,
        };
    } else if index == Index::Fest {
        let vals = ReqResB {
            _fullname: txt[0].clone(),
            economy: txt[1].parse::<f64>().expect("REASON"),
            compassion: txt[2].parse::<f64>().expect("REASON"),
            weather: txt[3].parse::<f64>().expect("REASON"),
            obesity: txt[4].parse::<f64>().expect("REASON"),
            secularism: txt[5].parse::<f64>().expect("REASON"),
            foodquality: txt[6].parse::<f64>().expect("REASON"),
        };
        let festindex = (vals.compassion / 250.0) + (vals.foodquality / 2500.0)
            - (vals.weather / 62500.0)
            - (vals.secularism / 150.0)
            + (vals.obesity / 1250.0)
            + (vals.economy / 250.0);
        on = Passed {
            fullname: vals._fullname,
            index: festindex,
            status: rval.header,
        };
    } else {
        panic!("You shouldn't be here!")
    }
    Ok(on)
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch(nation: String, index: &Index) -> Result<RawResponse> {
    let url = match index {
        Index::Green => String::from(
            "https://www.nationstates.net/cgi-bin/api.cgi?nation=".to_owned()
                + &nation
                + "&q=fullname+census;mode=score;scale=7+41+44+57+58+63",
        ),
        Index::Fest => String::from(
            "https://www.nationstates.net/cgi-bin/api.cgi?nation=".to_owned()
                + &nation
                + "&q=fullname+census;mode=score;scale=1+6+41+61+62+88",
        ),
    };
    let client = reqwest::Client::builder().user_agent("Garbelia").build()?;
    let res = client.get(url).send().await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());
    let nres = res.status();
    let bodyn = res.text().await?;
    let resp = RawResponse {
        body: bodyn,
        header: nres.to_string(),
    };
    Ok(resp)
}
