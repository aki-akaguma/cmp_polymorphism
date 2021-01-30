rustc 1.49.0 (e1884a8e3 2020-12-29)
|         `name`         |   `bench`   |  `per 1k`   |
|:-----------------------|------------:|------------:|
| enum_obj^              |    0.024 kc |    0.000 kc |
| enum_obj^vec^id^01k    |    4.451 kc |    4.451 kc |
| enum_obj^vec^id^08k    |   35.691 kc |    4.461 kc |
| enum_obj^vec^id^90k    |  959.015 kc |   10.656 kc |
| enum_obj^vec^rem^01k   |   11.608 kc |   11.608 kc |
| enum_obj^vec^rem^08k   |   94.461 kc |   11.808 kc |
| enum_obj^vec^rem^90k   | 1393.037 kc |   15.478 kc |
| enum_obj^vec^sum^01k   |    7.873 kc |    7.873 kc |
| enum_obj^vec^sum^08k   |   62.637 kc |    7.830 kc |
| enum_obj^vec^sum^90k   | 1120.224 kc |   12.447 kc |
| trait_obj^             |    0.024 kc |    0.000 kc |
| trait_obj^vec^id^01k   |    7.181 kc |    7.181 kc |
| trait_obj^vec^id^08k   |   56.925 kc |    7.116 kc |
| trait_obj^vec^id^90k   |  655.004 kc |    7.278 kc |
| trait_obj^vec^rem^01k  |   10.322 kc |   10.322 kc |
| trait_obj^vec^rem^08k  |   81.234 kc |   10.154 kc |
| trait_obj^vec^rem^90k  | 1605.206 kc |   17.836 kc |
| trait_obj^vec^sum^01k  |    8.627 kc |    8.627 kc |
| trait_obj^vec^sum^08k  |   65.433 kc |    8.179 kc |
| trait_obj^vec^sum^90k  | 1442.311 kc |   16.026 kc |
