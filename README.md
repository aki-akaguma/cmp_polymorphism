rustc 1.51.0 (2fd73fabe 2021-03-23)
|         `name`         |   `bench`   |  `per 1k`   |
|:-----------------------|------------:|------------:|
| enum_obj^              |    0.024 kc |    0.000 kc |
| enum_obj^vec^id^01k    |    4.430 kc |    4.430 kc |
| enum_obj^vec^id^08k    |   35.199 kc |    4.400 kc |
| enum_obj^vec^id^90k    |  566.817 kc |    6.298 kc |
| enum_obj^vec^rem^01k   |   11.505 kc |   11.505 kc |
| enum_obj^vec^rem^08k   |   91.111 kc |   11.389 kc |
| enum_obj^vec^rem^90k   | 1162.610 kc |   12.918 kc |
| enum_obj^vec^sum^01k   |    7.761 kc |    7.761 kc |
| enum_obj^vec^sum^08k   |   61.818 kc |    7.727 kc |
| enum_obj^vec^sum^90k   |  877.909 kc |    9.755 kc |
| trait_obj^             |    0.024 kc |    0.000 kc |
| trait_obj^vec^id^01k   |    7.112 kc |    7.112 kc |
| trait_obj^vec^id^08k   |   56.416 kc |    7.052 kc |
| trait_obj^vec^id^90k   |  634.964 kc |    7.055 kc |
| trait_obj^vec^rem^01k  |   10.235 kc |   10.235 kc |
| trait_obj^vec^rem^08k  |   80.327 kc |   10.041 kc |
| trait_obj^vec^rem^90k  | 1309.619 kc |   14.551 kc |
| trait_obj^vec^sum^01k  |    8.569 kc |    8.569 kc |
| trait_obj^vec^sum^08k  |   67.534 kc |    8.442 kc |
| trait_obj^vec^sum^90k  | 1178.532 kc |   13.095 kc |
