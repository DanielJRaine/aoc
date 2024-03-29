#![feature(slice_pattern)]
extern crate core;

use core::slice::SlicePattern;
use std::env;

use aoc;
use eyre::{bail, eyre};
use jane_eyre::Result;
use regex::Regex;
use rust_lapper::{Interval, Lapper};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<()> {
    jane_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        color_eyre::eyre::bail!("Must provide part #. Allowed values: {:?}", vec![1, 2]);
    }

    let part_number = &args[1];
    match part_number.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => Err(color_eyre::eyre::eyre!("Select part 1 or 2")),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Resource {
    range_start: u128,
    range_end: u128,
}

#[derive(Debug, PartialEq, Eq)]
struct ResourceMap {
    map_ranges: Vec<ResourceMapRange>,
}

impl ResourceMap {
    // checks source range, returns correct destination resource
    fn to_destination(&self, &resource: &u128) -> u128 {
        for range in &self.map_ranges {
            if let Some(destination) = range.to_destination(resource) {
                return destination;
            };
        }

        // if resource is not mapped, return the self-mapped value
        resource
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ResourceMapRange {
    destination_range_start: u128,
    source_range_start: u128,
    range_length: u128,
}

impl ResourceMapRange {
    fn to_destination(&self, resource: u128) -> Option<u128> {
        // checks source range, returns correct destination resource
        if (self.source_range_start..self.source_range_start + self.range_length)
            .contains(&resource)
        {
            // calculate the offset from the range start
            let offset = resource - self.source_range_start;

            // take the same offset and apply it to the destination range to get the returned resource
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }
}

fn parse_ranges(input: &str) -> Vec<ResourceMapRange> {
    let mut ranges = vec![];
    for line in input.trim().lines() {
        let mut iter = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u128>().unwrap());
        ranges.push(ResourceMapRange {
            destination_range_start: iter.next().to_owned().unwrap(),
            source_range_start: iter.next().to_owned().unwrap(),
            range_length: iter.next().to_owned().unwrap(),
        });
    }

    ranges
}

fn parse_intervals(input: &str) -> Vec<Interval<u128, u128>> {
    let mut ranges = vec![];
    for line in input.trim().lines() {
        let mut iter = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u128>().unwrap());
        ranges.push(ResourceMapRange {
            destination_range_start: iter.next().to_owned().unwrap(),
            source_range_start: iter.next().to_owned().unwrap(),
            range_length: iter.next().to_owned().unwrap(),
        });
    }
    todo!();
}

fn part1() -> Result<()> {
    // let input: String = aoc::read_input();
    // for line in input.lines() {}

    // split on ':'

    let seeds_input = "79 14 55 13";
    let seeds: Vec<u128> = seeds_input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seed_to_soil_input = "50 98 2
52 50 48";
    let seed_to_soil_map = ResourceMap {
        map_ranges: parse_ranges(seed_to_soil_input),
    };

    let soil_to_fertilizer_input = "0 15 37
37 52 2
39 0 15";
    let soil_to_fertilizer_map = ResourceMap {
        map_ranges: parse_ranges(soil_to_fertilizer_input),
    };

    let fertilizer_to_water_input = "49 53 8
0 11 42
42 0 7
57 7 4";

    let fertilizer_to_water_map = ResourceMap {
        map_ranges: parse_ranges(fertilizer_to_water_input),
    };

    let water_to_light_input = "88 18 7
18 25 70";
    let water_to_light_map = ResourceMap {
        map_ranges: parse_ranges(water_to_light_input),
    };

    let light_to_temperature_input = "45 77 23
81 45 19
68 64 13";
    let light_to_temperature_map = ResourceMap {
        map_ranges: parse_ranges(light_to_temperature_input),
    };

    let temperature_to_humidity_input = "0 69 1
1 0 69
";
    let temperature_to_humidity_map = ResourceMap {
        map_ranges: parse_ranges(temperature_to_humidity_input),
    };

    let humidity_to_location_input = "60 56 37
56 93 4";
    let humidity_to_location_map = ResourceMap {
        map_ranges: parse_ranges(humidity_to_location_input),
    };

    // lowest location id that corresponds to any of the initial seeds
    // we could solve this forwards...
    let mut locations = vec![];
    for seed in seeds {
        // find location
        let soil = seed_to_soil_map.to_destination(&seed);
        let fertilizer = soil_to_fertilizer_map.to_destination(&soil);
        let water = fertilizer_to_water_map.to_destination(&fertilizer);
        let light = water_to_light_map.to_destination(&water);
        let temp = light_to_temperature_map.to_destination(&light);
        let hum = temperature_to_humidity_map.to_destination(&temp);
        let loc = humidity_to_location_map.to_destination(&hum);

        locations.push(loc);
    }

    let min = locations.iter().min().unwrap();

    // too low 68979978
    println!("{min}");
    // ...or backwards
    // let locations =

    Ok(())
}

fn part2() -> Result<()> {
    let seeds_input = "3640772818 104094365 1236480411 161072229 376099792 370219099 1590268366 273715765 3224333694 68979978 2070154278 189826014 3855332650 230434913 3033760782 82305885 837883389 177854788 2442602612 571881366";
    let seed_strs: Vec<&str> = seeds_input.split_ascii_whitespace().collect();
    let seed_chunks = seed_strs.chunks(2);

    let intervals: Vec<Interval<u128, u128>> = seed_chunks
        .map(|ranges| {
            if let [range_start, range_length] = ranges.as_slice() {
                let start = range_start.parse::<u128>().unwrap();
                let length = range_length.parse::<u128>().unwrap();
                let stop = start + length - 1;
                Interval {
                    start,
                    stop, // FIXME: may want stop + 1 for inclusive
                    val: 0,
                }
            } else {
                unreachable!();
            }
        })
        .collect();

    let lapper1 = Lapper::new(intervals);

    let seed_to_soil_input = "496269031 1203272644 52136246
548405277 496269031 457095898
1005501175 953364929 249907715";
    let seed_to_soil_map = ResourceMap {
        map_ranges: parse_ranges(seed_to_soil_input),
    };

    let soil_to_fertilizer_input = "217408321 2086205436 25053699
2604208456 1670861921 31003781
1631572552 0 258383552
129225554 3768288787 36192668
2421205388 2905533654 126666762
242462020 3399542287 357404885
866152503 3032200416 253960559
2039921781 2262442546 381283607
2635212237 2714844607 190689047
3613008578 1753855801 23976114
3636984692 1503365158 167496763
1340671861 2111259135 274956
1889956104 806620565 149965677
0 3286160975 113381312
2987089260 956586242 546778916
2547872150 3756947172 11341615
3846919647 3807789063 43277850
3533868176 258383552 79140402
165418222 1701865702 51990099
1120113062 586061766 220558799
2559213765 2217447855 44994691
3807789063 4255836712 39130584
3890197497 3851066913 404769799
1340946817 1777831915 42087923
2881175496 2111534091 105913764
113381312 2643726153 15844242
1383034740 337523954 248537812
2825901284 2659570395 55274212
599866905 1819919838 266285598";
    let soil_to_fertilizer_map = ResourceMap {
        map_ranges: parse_ranges(soil_to_fertilizer_input),
    };

    let fertilizer_to_water_input = "3950520280 1751042330 139651634
936578795 3912173308 42397072
3553681000 1722281506 28760824
697953317 651809140 90189394
3582441824 876081661 368078456
978975867 2358439651 252255693
1495879532 2678320518 199775133
1910380638 3308279888 122339216
3355092099 1561901004 91630618
3187667509 2033753243 70292073
2752202873 3816184128 41568037
648600286 479585511 49353031
3149600631 2356473769 1965882
1695654665 2629130810 49189708
201901143 385568770 94016741
536376004 741998534 3182157
2472303091 3954570380 279899782
539558161 257732262 15304877
3131165165 2610695344 18435466
3446722717 1890693964 106958283
2793770910 1653531622 40823934
9849113 65680232 192052030
2878736712 2104045316 252428453
1792836692 2878095651 117543946
2172585320 1244160117 273599019
2060645804 3884012463 28160845
3257959582 3719051611 97132517
408449515 745180691 127926489
876081661 4234470162 60497134
1231231560 3043631916 264647972
3151566513 1997652247 36100996
2446184339 3430619104 26118752
803816626 582518586 69290554
554863038 9849113 40157204
2146325022 3857752165 26260298
788142711 50006317 15673915
295917884 273037139 112531631
2834594844 1517759136 44141868
4090171914 3456737856 204795382
2088806649 3661533238 57518373
1744844373 2995639597 47992319
2032719854 1694355556 27925950
595020242 528938542 53580044";

    let fertilizer_to_water_map = ResourceMap {
        map_ranges: parse_ranges(fertilizer_to_water_input),
    };

    let water_to_light_input = "556810106 840812947 14926117
2598413684 2184905392 114045192
2130064037 1600958027 248227533
1271028210 1253957270 39538107
3521286912 4262821917 32145379
1930562940 1944404618 64932992
4252373354 3302720391 42593942
1109026743 279496091 162001467
725777554 2565853410 3969864
2712458876 1330352326 185715465
4039742261 3173278185 129442206
0 2064417497 120487895
299311037 855739064 257499069
729747418 1849185560 95219058
1373001379 767226476 66231296
571736223 688212171 79014305
1878676528 2890412515 51886412
2898174341 682278717 5933454
1781236499 441497558 97440029
3090089298 3673052565 50139248
1365646204 833457772 7355175
1738475707 2412172480 42760792
1995495932 2569823274 134568105
3193548680 3476912261 196140304
824966476 538937587 143341130
3553432291 3723191813 486309970
3140228546 4209501783 53320134
968307606 1113238133 140719137
4169184467 3090089298 83188887
650750528 204469065 75027026
1439232675 2704391379 186021136
2511835025 1516067791 84890236
1625253811 2298950584 113221896
3389688984 3345314333 131597928
2378291570 0 22623317
2596725261 2942298927 1688423
1310566317 2009337610 55079887
2940964744 201446459 3022606
120487895 22623317 178823142
2904107795 1293495377 36856949
2400914887 2454933272 110920138";
    let water_to_light_map = ResourceMap {
        map_ranges: parse_ranges(water_to_light_input),
    };

    let light_to_temperature_input = "1244459013 624435822 80444775
2608592263 3309263777 172991510
3165402867 2278806547 335097905
292819381 1643978777 105413752
704475267 462426854 15399493
3696584161 2678497330 345840247
2891254573 3613542439 34162874
1894523870 281665589 180761265
1706897891 1456352798 187625979
398233133 1877125477 198159658
4176101046 3482255287 104541624
2278806547 3662029939 329785716
3500500772 4098883907 196083389
4280642670 3647705313 14324626
4149492660 3024337577 11203522
85784517 0 207034864
2925417447 3231118601 78145176
0 1749392529 85784517
4160696182 3215713737 15404864
4042424408 3991815655 107068252
1129076520 207034864 74630725
596392791 1348270322 108082476
3100809989 2613904452 64592878
1326100451 1114082357 234187965
2864509045 3586796911 26745528
2781583773 3132788465 82925272
1560288416 477826347 146609475
3003562623 3035541099 97247366
719874760 704880597 409201760
1324903788 1875928814 1196663
1203707245 1835177046 40751768";
    let light_to_temperature_map = ResourceMap {
        map_ranges: parse_ranges(light_to_temperature_input),
    };

    let temperature_to_humidity_input = "2622049454 736812858 79169969
3979548277 2854489162 116161222
0 2175018874 84480806
567798788 2400631546 48501534
3627076350 2837901836 16587326
1459224370 1289368272 21397154
3018037189 3252694507 93925363
4095709499 4063232797 21404553
1675574530 2560175285 695067
650203851 328659590 327259736
3143763337 3475937023 49576
1591893253 2091337597 83681277
3655324442 3516768137 39649206
1125784995 2259499680 94582805
1082589333 280432563 43195662
1676269597 1011614859 3965286
3955996260 4039680780 23552017
2701219423 684725863 52086995
977463587 2387091631 13539915
3643663676 3346619870 11660766
3143812913 3556417343 483263437
2907111949 3358280636 110925240
2293944099 2615629615 132579811
991003502 920029028 91585831
1513630670 1015580145 78262583
196718861 1310765426 316320664
1680234883 1638774240 331394507
2011629390 815982827 1882146
2874635247 4084637350 32476702
3701704795 2998403042 254291465
2426523910 1093842728 195525544
2837901836 3475986599 8980753
3694973648 3469205876 6731147
3111962552 3484967352 31800785
616300322 662956301 21769562
1220367800 1627086090 10492300
1480621524 2354082485 33009146
2846882589 2970650384 27752658
1352028950 323628225 5031365
2753306418 655919326 7036975
513039525 2560870352 54759263
84480806 1637578390 1195850
2013511536 0 280432563
85676656 2449133080 111042205
1357060315 817864973 102164055
638069884 2748209426 12133967
1230860100 1970168747 121168850";
    let temperature_to_humidity_map = ResourceMap {
        map_ranges: parse_ranges(temperature_to_humidity_input),
    };

    let humidity_to_location_input = "3071447765 3790677895 35519893
501148922 1470714761 60946444
949413779 3960084356 1114317
2276139972 547813284 42132370
261623667 0 220957931
0 220957931 207965683
2629055810 2988733812 367963097
936813255 2507216386 12600524
3106967658 2402339659 33240399
909841910 3356696909 11084951
1030029700 920191219 341387512
2503236334 3572469232 125819476
1371417212 1666455982 273052538
2997018907 3367781860 25674024
3311094548 2519816910 468916902
2021404744 1531661205 83389724
3140208057 1939508520 59879232
259387264 428923614 2236403
950528096 1334878052 79501604
2446901229 1414379656 56335105
1644469750 3393455884 140294896
3022692931 3698288708 48754834
4042169428 1999387752 252797868
1805974329 3981490698 215430415
1784764646 501148922 21209683
889549885 3961198673 20292025
3780011450 522358605 10399946
2104794468 1261578731 73299321
3790411396 3842084182 118000174
2318272342 2252185620 113574154
4003450976 3533750780 38718452
2178093789 4196921113 98046183
3908411570 3747043542 43634353
562095366 2435580058 71636328
2431846496 532758551 15054733
920926861 3826197788 15886394
3952045923 1615050929 51405053
207965683 431160017 51421581
633731694 2365759774 36579885
670311579 700952913 219238306
3200087289 589945654 111007259
";
    let humidity_to_location_map = ResourceMap {
        map_ranges: parse_ranges(humidity_to_location_input),
    };

    // lowest location id that corresponds to any of the initial seeds
    // we could solve this forwards...
    let mut locations: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(vec![]));

    let min_interval = intervals
        .into_par_iter()
        .min_by(|seed_interval1, seed_interval2| {
            let min_seed1: u128 = (seed_interval1.start..=seed_interval1.stop)
                .into_par_iter()
                .min_by(|seed1, seed2| {
                    // this is where we should use interval_overlap... I think
                    let soil = &seed_to_soil_map.to_destination(&seed1);
                    let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
                    let water = &fertilizer_to_water_map.to_destination(&fertilizer);
                    let light = &water_to_light_map.to_destination(&water);
                    let temp = &light_to_temperature_map.to_destination(&light);
                    let hum = &temperature_to_humidity_map.to_destination(&temp);
                    let loc1 = &humidity_to_location_map.to_destination(&hum);

                    let soil = &seed_to_soil_map.to_destination(&seed2);
                    let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
                    let water = &fertilizer_to_water_map.to_destination(&fertilizer);
                    let light = &water_to_light_map.to_destination(&water);
                    let temp = &light_to_temperature_map.to_destination(&light);
                    let hum = &temperature_to_humidity_map.to_destination(&temp);
                    let loc2 = &humidity_to_location_map.to_destination(&hum);

                    let mut data = locations.lock().unwrap();
                    data.push(loc1.clone());

                    loc1.cmp(loc2)
                })
                .unwrap();

            let min_seed2 = (seed_interval2.start..=seed_interval2.stop)
                .into_par_iter()
                .min_by(|seed1, seed2| {
                    let soil = &seed_to_soil_map.to_destination(&seed1);
                    let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
                    let water = &fertilizer_to_water_map.to_destination(&fertilizer);
                    let light = &water_to_light_map.to_destination(&water);
                    let temp = &light_to_temperature_map.to_destination(&light);
                    let hum = &temperature_to_humidity_map.to_destination(&temp);
                    let loc1 = &humidity_to_location_map.to_destination(&hum);

                    let soil = &seed_to_soil_map.to_destination(&seed2);
                    let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
                    let water = &fertilizer_to_water_map.to_destination(&fertilizer);
                    let light = &water_to_light_map.to_destination(&water);
                    let temp = &light_to_temperature_map.to_destination(&light);
                    let hum = &temperature_to_humidity_map.to_destination(&temp);
                    let loc2 = &humidity_to_location_map.to_destination(&hum);

                    let mut data = locations.lock().unwrap();
                    data.push(loc1.clone());

                    loc1.cmp(loc2)
                })
                .unwrap();

            min_seed1.cmp(&min_seed2)
        })
        .unwrap();

    let global_min_seed: u128 = (min_interval.start..=min_interval.stop)
        .into_par_iter()
        .min_by(|seed1, seed2| {
            // thread::spawn(|| {
            let soil = &seed_to_soil_map.to_destination(&seed1);
            let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
            let water = &fertilizer_to_water_map.to_destination(&fertilizer);
            let light = &water_to_light_map.to_destination(&water);
            let temp = &light_to_temperature_map.to_destination(&light);
            let hum = &temperature_to_humidity_map.to_destination(&temp);
            let loc1 = &humidity_to_location_map.to_destination(&hum);

            let soil = &seed_to_soil_map.to_destination(&seed2);
            let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
            let water = &fertilizer_to_water_map.to_destination(&fertilizer);
            let light = &water_to_light_map.to_destination(&water);
            let temp = &light_to_temperature_map.to_destination(&light);
            let hum = &temperature_to_humidity_map.to_destination(&temp);
            let loc2 = &humidity_to_location_map.to_destination(&hum);

            loc1.cmp(loc2)
        })
        .unwrap();

    let soil = &seed_to_soil_map.to_destination(&global_min_seed);
    let fertilizer = &soil_to_fertilizer_map.to_destination(&soil);
    let water = &fertilizer_to_water_map.to_destination(&fertilizer);
    let light = &water_to_light_map.to_destination(&water);
    let temp = &light_to_temperature_map.to_destination(&light);
    let hum = &temperature_to_humidity_map.to_destination(&temp);
    let global_min_loc = &humidity_to_location_map.to_destination(&hum);

    let locations = locations.lock().unwrap();

    let min = locations.iter().min().unwrap();
    dbg!(min);
    println!("{min}");

    // println!("{global_min_loc}");
    // 511055685 is too high
    // 1100841983 is too high
    // 1134543333 is too high
    //
    // println!("{min}");
    // ...or backwards
    // let locations =

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_eq() {
        assert_eq!(1, 1);
    }

    #[test]
    fn parses_ranges() {
        let range = "
		0 15 37
		37 52 2
		";

        let solution = vec![
            ResourceMapRange {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            },
            ResourceMapRange {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            },
        ];

        let attempt = parse_ranges(range);
        assert_eq!(attempt, solution)
    }

    #[test]
    fn finds_self_mapped_destination() {
        let res_range = ResourceMapRange {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let res_map = ResourceMap {
            map_ranges: vec![res_range],
        };

        let source_resource = 1u128;
        let destination_resource = 1u128;
        // unmapped
        assert_eq!(
            res_map.to_destination(source_resource),
            destination_resource
        );

        // mapped
        assert_ne!(res_map.to_destination(99), 99);
    }

    #[test]
    fn finds_destination() {
        let res_range = ResourceMapRange {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let destination_resource = 51;
        let source_resource = 99;
        assert_eq!(
            res_range.to_destination(source_resource),
            Some(destination_resource)
        )
    }
}
