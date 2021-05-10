rustc 1.52.0 (88f19c6da 2021-05-03)
|         `name`         |   `bench`   |  `per 1k`   |
|:-----------------------|------------:|------------:|
| enum_obj^              |    0.024 kc |    0.000 kc |
| enum_obj^vec^id^01k    |    4.425 kc |    4.425 kc |
| enum_obj^vec^id^08k    |   35.634 kc |    4.454 kc |
| enum_obj^vec^id^90k    |  583.796 kc |    6.487 kc |
| enum_obj^vec^rem^01k   |   11.453 kc |   11.453 kc |
| enum_obj^vec^rem^08k   |   90.914 kc |   11.364 kc |
| enum_obj^vec^rem^90k   | 1182.439 kc |   13.138 kc |
| enum_obj^vec^sum^01k   |   10.670 kc |   10.670 kc |
| enum_obj^vec^sum^08k   |   79.980 kc |    9.998 kc |
| enum_obj^vec^sum^90k   |  900.310 kc |   10.003 kc |
| trait_obj^             |    0.024 kc |    0.000 kc |
| trait_obj^vec^id^01k   |    6.062 kc |    6.062 kc |
| trait_obj^vec^id^08k   |   48.345 kc |    6.043 kc |
| trait_obj^vec^id^90k   |  546.009 kc |    6.067 kc |
| trait_obj^vec^rem^01k  |   10.162 kc |   10.162 kc |
| trait_obj^vec^rem^08k  |   80.179 kc |   10.022 kc |
| trait_obj^vec^rem^90k  | 1280.631 kc |   14.229 kc |
| trait_obj^vec^sum^01k  |    8.413 kc |    8.413 kc |
| trait_obj^vec^sum^08k  |   66.099 kc |    8.262 kc |
| trait_obj^vec^sum^90k  | 1182.313 kc |   13.137 kc |
