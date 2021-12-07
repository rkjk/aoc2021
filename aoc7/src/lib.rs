use std::cmp::{min, max};

/// Return a different cost function based on flag
fn compute_function(linear: bool) -> Box<dyn Fn(&[i64], i64) -> i64> {
    if !linear {
        return Box::new(
            |input: &[i64], pivot: i64| -> i64 {
                input.iter().map(|val| (val - pivot).abs()).sum()
            }
        )
    } else {
        return Box::new(
            |input: &[i64], pivot: i64| -> i64 {
                input.iter().map(|val| {
                    let steps = (val - pivot).abs();
                    steps * (steps + 1) / 2
                }).sum()
            }
        )
    }
}

/// Binary search to find "peak" element except we want the trough
fn find_trough(input: &[i64], linear: bool) -> i64 {
    let mut start: i64 = *input.iter().min().unwrap();
    let mut end: i64 = *input.iter().max().unwrap();
    let compute_fuel = compute_function(linear);
    while start < end {
        let mid = start + (end  - start) / 2;
        if compute_fuel(input, mid) < compute_fuel(input, mid + 1) {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    compute_fuel(input, start)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = [16,1,2,0,4,2,7,1,2,14];
        println!("Part1: {}", find_trough(&input, false));
        println!("Part2: {}", find_trough(&input, true));
    }

    #[test]
    fn actual() {
        let input = [1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,231,350,1278,139,664,182,40,377,157,886,184,138,727,661,904,24,77,349,608,1346,963,12,759,112,129,818,1046,600,43,523,709,1002,2,202,212,11,264,0,505,956,163,560,128,299,0,229,4,33,1402,268,418,1435,151,786,727,100,248,57,763,311,206,67,997,379,378,332,0,23,143,93,389,815,11,66,873,1414,7,7,681,140,288,390,434,314,215,360,3,317,463,294,0,33,801,1417,54,782,937,94,102,95,15,177,649,45,107,695,100,49,193,636,263,177,932,345,706,99,28,211,449,442,117,7,467,101,732,947,818,346,87,78,285,236,707,912,1652,294,333,706,758,1621,782,246,546,663,699,177,94,221,746,66,170,514,364,751,1486,157,54,38,1469,0,79,83,1060,422,252,27,1332,386,523,41,934,988,278,409,438,61,1047,260,300,240,0,496,1392,181,268,413,544,1169,662,566,4,988,267,1259,250,346,319,235,172,728,1621,505,1490,17,104,711,714,1139,497,603,759,393,1184,60,369,1326,333,45,51,118,1171,29,1560,252,139,481,1160,177,555,150,115,129,237,1672,613,1311,999,217,20,936,323,116,60,198,644,718,69,594,1142,607,854,878,926,515,29,2,740,1281,74,1406,47,88,249,1416,1263,943,1477,39,123,1919,37,167,227,478,405,421,316,335,1375,359,498,173,507,456,40,226,160,927,229,848,6,1174,1107,710,13,480,1249,817,85,80,128,12,48,243,576,199,208,338,1521,1167,282,690,16,362,791,25,435,495,1217,1215,387,36,1620,166,1586,345,698,541,590,277,328,85,862,751,1273,950,817,77,749,198,156,212,404,6,197,425,582,453,59,45,1059,1058,389,178,547,847,670,559,81,1180,220,1338,216,1528,629,601,802,903,207,352,228,29,761,477,161,268,228,647,80,110,402,470,714,439,511,13,70,277,746,492,657,1215,146,201,63,84,1158,1615,513,1182,83,73,60,22,221,888,344,27,205,1344,325,1362,102,1396,1117,426,80,497,458,11,218,165,221,649,524,264,251,617,825,172,1120,931,520,112,1286,818,1464,11,1,83,184,320,152,730,744,409,604,73,1205,411,732,1078,775,334,130,202,716,368,734,794,723,1140,367,222,435,596,566,719,1046,1428,797,470,124,380,1833,180,62,714,1112,772,26,89,445,9,147,76,764,267,1400,6,275,69,292,143,522,376,797,73,136,688,30,417,1835,47,54,19,32,565,85,320,426,771,66,1656,740,75,10,284,23,14,65,719,1719,874,426,599,314,445,796,994,467,49,0,1141,248,957,50,1024,427,696,533,1284,811,89,17,597,463,1501,13,199,701,53,318,7,628,608,147,291,22,518,191,1243,333,88,12,138,363,262,753,467,456,74,1047,15,339,234,612,452,424,340,481,13,4,303,30,908,1069,1018,1584,426,192,304,337,326,1087,406,132,449,1142,279,307,315,1445,113,49,705,120,187,4,798,960,431,214,1051,848,54,845,64,83,1059,813,1390,1008,237,469,156,61,635,1074,1621,523,24,140,141,715,1124,402,400,204,18,452,1107,453,377,467,241,340,35,320,799,680,5,123,43,1614,1774,549,651,163,700,776,65,336,145,426,150,1049,113,1346,434,45,521,729,55,1448,85,1133,1421,375,1398,319,206,606,68,1597,716,1507,963,141,95,72,33,1242,251,448,1337,1132,83,1779,284,58,625,253,1247,344,47,1194,1047,190,538,103,322,652,44,422,53,31,345,1346,27,768,1006,179,447,1318,199,92,364,141,121,276,284,847,462,700,780,360,843,1430,185,69,635,292,413,43,71,240,15,787,379,1353,173,305,227,118,844,632,471,523,1139,8,811,355,811,223,37,267,438,1011,58,39,64,422,167,844,165,80,618,1115,194,547,47,99,639,171,43,246,104,1429,510,127,125,1035,290,839,1060,26,160,31,570,623,80,1246,645,1396,99,543,159,525,211,446,209,885,512,1483,479,716,417,268,583,1467,573,553,95,729,1589,207,67,224,243,426,283,398,612,596,248,282,180,94,405,148,429,37,116,582,32,253,282,832,94,154,338,75,404,651,365,1436,60,266,1163,982,69,958,751,1693,850,1257,1294,429,120,133,741,564,328,315,1268,98,20,14,114,478,20,344,631,1296,24,1611,487,659,355,1336,20,1197,515,13,1165,1007,1403,1473,126,461,431,15,136,730,449,1109,1146,1210,944,158,742,1586,380,1051,41,1250,915,1417,681,642,70,1789,54,161,1568,676,113,287,338,127,1168,615,421,215];
        println!("Part1: {}", find_trough(&input, false));
        println!("Part2: {}", find_trough(&input, true));
    }
}
