extern crate osm_pbf2json;

use geojson::GeoJson;
use osm_pbf2json::output::Output;
use osm_pbf2json::{boundaries, filter, objects, streets};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

fn get_string(cursor: &mut Cursor<Vec<u8>>) -> String {
    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    cursor.read_to_end(&mut out).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn find_fountains_or_townhalls() {
    let mut cursor = Cursor::new(Vec::new());
    let groups = filter::parse("amenity~fountain+tourism,amenity~townhall");
    let file = File::open("./tests/data/alexanderplatz.pbf").unwrap();
    let objects = objects(file, Some(&groups), false).unwrap();
    objects.write_json_lines(&mut cursor).unwrap();
    let string = get_string(&mut cursor);
    let lines: Vec<&str> = string.trim().split('\n').collect();
    assert_eq!(lines.len(), 4);
    for line in lines {
        assert!(
            (line.contains(r#"amenity":"fountain"#) && line.contains(r#"tourism"#))
                || line.contains(r#"amenity":"townhall"#)
        );
    }
}

#[test]
fn find_bike_parking_for_six() {
    let mut cursor = Cursor::new(Vec::new());
    let groups = filter::parse("amenity~bicycle_parking+capacity~6");
    let file = File::open("./tests/data/alexanderplatz.pbf").unwrap();
    let objects = objects(file, Some(&groups), false).unwrap();
    objects.write_json_lines(&mut cursor).unwrap();
    let string = get_string(&mut cursor);
    let lines: Vec<&str> = string.trim().split('\n').collect();
    assert_eq!(lines.len(), 14);
}

#[test]
fn streets_as_geojson() {
    let mut cursor = Cursor::new(Vec::new());
    let file = File::open("./tests/data/alexanderplatz.pbf").unwrap();
    let streets = streets(file, Some("Alexanderstraße"), None).unwrap();
    streets.write_geojson(&mut cursor).unwrap();
    let geojson_str = get_string(&mut cursor);
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    if let GeoJson::FeatureCollection(col) = geojson {
        assert_eq!(col.features.len(), 1);
    } else {
        panic!("not a feature collection");
    }
}

#[test]
fn rosa_luxemburg_street() {
    let mut cursor = Cursor::new(Vec::new());
    let name = "Rosa-Luxemburg-Straße";
    let file = File::open("./tests/data/alexanderplatz.pbf").unwrap();
    let streets = streets(file, Some(name), None).unwrap();
    streets.write_json_lines(&mut cursor).unwrap();
    let string = get_string(&mut cursor);
    let lines: Vec<&str> = string.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("Rosa-Luxemburg-Straße"));
}

#[test]
fn split_street_by_boundary() {
    let mut cursor = Cursor::new(Vec::new());
    let name = "Wilhelmstraße";
    let file = File::open("./tests/data/wilhelmstrasse.pbf").unwrap();
    let streets = streets(file, Some(name), Some(10)).unwrap();
    streets.write_json_lines(&mut cursor).unwrap();
    let string = get_string(&mut cursor);
    let mut lines: Vec<&str> = string.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    lines.sort();
    assert!(lines[0].contains("Wilhelmstraße"));
    assert!(lines[0].contains("Kreuzberg"));
    assert!(lines[1].contains("Wilhelmstraße"));
    assert!(lines[1].contains("Mitte"));
}

#[test]
fn boundary_as_geojson() {
    let mut cursor = Cursor::new(Vec::new());
    let file = File::open("./tests/data/wilhelmstrasse.pbf").unwrap();
    let boundaries = boundaries(file, Some(vec![10])).unwrap();
    boundaries.write_geojson(&mut cursor).unwrap();
    let geojson_str = get_string(&mut cursor);
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    if let GeoJson::FeatureCollection(col) = geojson {
        assert_eq!(col.features.len(), 2);
    } else {
        panic!("not a feature collection");
    }
}

#[test]
fn extract_boundaries() {
    let mut cursor = Cursor::new(Vec::new());
    let file = File::open("./tests/data/wilhelmstrasse.pbf").unwrap();
    let boundaries = boundaries(file, Some(vec![10])).unwrap();
    boundaries.write_json_lines(&mut cursor).unwrap();
    let string = get_string(&mut cursor);
    let mut lines: Vec<&str> = string.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    lines.sort();
    assert!(lines[0].contains("Kreuzberg"));
    assert!(lines[1].contains("Mitte"));
}
