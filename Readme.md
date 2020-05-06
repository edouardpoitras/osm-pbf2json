# osm-pbf2json

[![Build Status](https://travis-ci.org/mkulke/osm-pbf2json.svg?branch=master)](https://travis-ci.org/mkulke/osm-pbf2json)

A parser/filter for OSM protobuf bundles

## Build

### Local

A rust build environment can be installed via [rustup](https://rustup.rs/).

```
cargo test
cargo build --release
```

### Docker

To build linux images on MacOS. The build artifact is located in `./docker-target/release` afterwards.

```
mkdir docker-target
docker run \
  -v $PWD/Cargo.toml:/build/Cargo.toml \
  -v $PWD/Cargo.lock:/build/Cargo.lock \
  -v $PWD/src:/build/src \
  -v $PWD/docker-target:/build/target \
  -w /build \
  rust:1.43 cargo build --release
```

## Run

```
./target/release/osm_pbf2json -t="addr:housenumber+addr:street+addr:postcode~10178" berlin.pbf | tail -3
{"id":544604702,"type":"way","tags":{"addr:city":"Berlin","addr:country":"DE","addr:housenumber":"17","addr:postcode":"10178","addr:street":"Sophienstraße","addr:suburb":"Mitte","building":"residential","heritage":"4","heritage:operator":"lda","lda:criteria":"Ensembleteil","ref:lda":"09080182"},"centroid":{"lat":52.52571770265661,"lon":13.401513737828404},"bounds":{"e":13.4015869,"n":52.525649699999995,"s":52.5254975,"w":13.4013709}}
{"id":569067822,"type":"way","tags":{"addr:city":"Berlin","addr:housenumber":"1","addr:postcode":"10178","addr:street":"Anna-Louisa-Karsch-Straße","amenity":"library","email":"theol@ub.hu-berlin.de","internet_access":"wlan","internet_access:fee":"no","name":"Humboldt-Universität zu Berlin Universitätsbibliothek Zweigbibliothek Theologie","name:en":"Humboldt University of Berlin University Library Theology Branch Library","opening_hours":"Mo-Fr 09:30-20:30; Sa 09:30-13:30","operator":"Universitätsbibliothek der Humboldt-Universität zu Berlin","phone":"+49 30 2093-91800","ref:isil":"DE-11-133","website":"https://www.ub.hu-berlin.de/de/standorte/zwbtheologie","wheelchair":"yes","wikidata":"Q73146656"},"centroid":{"lat":52.52113243392209,"lon":13.401373781610301},"bounds":{"e":13.4016009,"n":52.521282,"s":52.520961,"w":13.4011406}}
{"id":625034881,"type":"way","tags":{"addr:city":"Berlin","addr:housenumber":"1","addr:postcode":"10178","addr:street":"Karl-Marx-Allee","building":"commercial","building:levels":"1","height":"5","name":"Werkstatt Haus der Statistik"},"centroid":{"lat":52.52212174147001,"lon":13.418398630534096},"bounds":{"e":13.418516499999999,"n":52.5221701,"s":52.5220175,"w":13.4182626}}
```

## Test

```
cargo test
cargo bench
```
