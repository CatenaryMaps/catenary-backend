use postgis::ewkb;
extern crate time;
extern crate travelling_salesman;
use geo::HaversineLength;
use geo::{line_string, polygon};
use geo_postgis::{FromPostgis, ToPostgis};

pub fn fix_broken_lines(
    linestring: ewkb::LineStringT<ewkb::Point>,
    feed_id: &str,
) -> ewkb::LineStringT<ewkb::Point> {
    let geo_typed = geo_types::LineString::from_postgis(&linestring);

    let length = geo_typed.haversine_length();

    if feed_id == "f-ezjm-informaciónoficial~consorcioregionaldetransportesdemadrid" {
        println!("{}", length);
    }

    linestring
}
