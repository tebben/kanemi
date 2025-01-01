use criterion::{criterion_group, criterion_main, Criterion};
use kanemi::harmonie_cy43_p1::reader::GribFile;

//
// Benchmark results trying to check implementations and optimizations.
//
// ---------------------------------------------------------------------------
// Load grib file and read temperature at height 0 for 1 lon/lat location, no data is
// being passed around. The data is only read and not used.
//
// 1: Naive implementation iterating over all messages/data and picking the data we want.
//
//    985.50 µs 988.84 µs 992.29 µs
//
// 2: Read all messages and update PDS info with reader locations of messages.
//    pick the correct message we want and in BDS use the calculated location index to find
//    and read the values we want from BDS.
//
//    101.37 µs 101.95 µs 102.66 µs
//
// Conclusion: Implementation 2 is much faster than implementation 1 for the case of filtering
// data. Maybe implementation 1 is faster if we are going to read all data anyway.
//
// ---------------------------------------------------------------------------
// Continue with implementation 2, no need to fully index/read all messages if we are not
// going to use the data. We can stop reading the file when we have found the messages we want.
// When indexing we now break out of the loop when we found byte indexes for the data we want.
// After testing it seems that performance differs alot when reading the values from bds, for the next
// benchmarks reading values was disabled. Scenario 2 without reading bds values the
// time is on average 81.130 µs vs 101.95 µs.
//
// 1: Test parameter which is first in the messages (33-100-105: (tmp, 0))
//
//    22.441 µs 22.652 µs 22.898 µs
//
// 2: Test parameter which is last in the messages. (11-801-105: (isba, 801))
//
//    24.718 µs 24.749 µs 24.789 µs
//
// 3: Test parameter which is in the middle (24 + 1 since 49 msgs) of the messages. (52 - 2 - 105: (rh, 2))
//
//    50.549 µs 50.632 µs 50.718 µs
//
// 4: All parameters
//
//    82.422 µs 82.947 µs 83.558 µs
//
// 5: tmp + isba + rh
//
//    40.409 µs 40.591 µs 40.912 µs
//
// Conclusion: Breaking out of the indexing loop can improve execution time, reading all values with the added
// code is not slower than reading all values without check. Seems it safe to say this is a good optimization
// for getting certain values without decreasing performance when reading all values.
//
// ---------------------------------------------------------------------------

const FILE_PATH: &str = "../example_data/HA43_N20_202412221800_00000_GB";
const LOCATIONS_SINGLE: &[(f32, f32)] = &[(5.351926_f32, 51.716_8_f32)];
const PARAMETERS_SINGLE: &[(&str, u16)] = &[("tmp", 0)];

fn read_grib_cy43_p1() {
    let grib_file = GribFile::open(FILE_PATH);
    grib_file.unwrap().get(
        Some(&PARAMETERS_SINGLE.to_vec()),
        Some(&LOCATIONS_SINGLE.to_vec()),
    );
}

fn benchmarks(c: &mut Criterion) {
    c.bench_function("Read Grib", |b| b.iter(|| read_grib_cy43_p1()));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
