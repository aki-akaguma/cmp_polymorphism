# cmp_polymorphism

![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

research: compare polymorphism

## Benchmark Results

rustc 1.56.1 (59eed8a2a 2021-11-01)
|         `name`         |   `bench`   |  `per 1k`   |
|:-----------------------|------------:|------------:|
| enum_obj^              |    0.025 kc |    0.000 kc |
| enum_obj^vec^id^01k    |    4.576 kc |    4.576 kc |
| enum_obj^vec^id^08k    |   36.355 kc |    4.544 kc |
| enum_obj^vec^id^90k    | 1270.354 kc |   14.115 kc |
| enum_obj^vec^rem^01k   |   12.587 kc |   12.587 kc |
| enum_obj^vec^rem^08k   |   99.855 kc |   12.482 kc |
| enum_obj^vec^rem^90k   | 1914.675 kc |   21.274 kc |
| enum_obj^vec^sum^01k   |    7.910 kc |    7.910 kc |
| enum_obj^vec^sum^08k   |   87.262 kc |   10.908 kc |
| enum_obj^vec^sum^90k   | 1597.248 kc |   17.747 kc |
| trait_obj^             |    0.025 kc |    0.000 kc |
| trait_obj^vec^id^01k   |    6.858 kc |    6.858 kc |
| trait_obj^vec^id^08k   |   49.307 kc |    6.163 kc |
| trait_obj^vec^id^90k   |  627.355 kc |    6.971 kc |
| trait_obj^vec^rem^01k  |   10.774 kc |   10.774 kc |
| trait_obj^vec^rem^08k  |   84.752 kc |   10.594 kc |
| trait_obj^vec^rem^90k  | 2287.182 kc |   25.413 kc |
| trait_obj^vec^sum^01k  |    8.720 kc |    8.720 kc |
| trait_obj^vec^sum^08k  |   71.800 kc |    8.975 kc |
| trait_obj^vec^sum^90k  | 2374.904 kc |   26.388 kc |

rustc 1.53.0 (53cb7b09b 2021-06-17)
|         `name`         |   `bench`   |  `per 1k`   |
|:-----------------------|------------:|------------:|
| enum_obj^              |    0.024 kc |    0.000 kc |
| enum_obj^vec^id^01k    |    4.358 kc |    4.358 kc |
| enum_obj^vec^id^08k    |   35.195 kc |    4.399 kc |
| enum_obj^vec^id^90k    |  604.227 kc |    6.714 kc |
| enum_obj^vec^rem^01k   |   11.488 kc |   11.488 kc |
| enum_obj^vec^rem^08k   |   91.363 kc |   11.420 kc |
| enum_obj^vec^rem^90k   | 1198.158 kc |   13.313 kc |
| enum_obj^vec^sum^01k   |   10.501 kc |   10.501 kc |
| enum_obj^vec^sum^08k   |   60.464 kc |    7.558 kc |
| enum_obj^vec^sum^90k   |  930.674 kc |   10.341 kc |
| trait_obj^             |    0.024 kc |    0.000 kc |
| trait_obj^vec^id^01k   |    7.092 kc |    7.092 kc |
| trait_obj^vec^id^08k   |   56.413 kc |    7.052 kc |
| trait_obj^vec^id^90k   |  637.158 kc |    7.080 kc |
| trait_obj^vec^rem^01k  |   10.077 kc |   10.077 kc |
| trait_obj^vec^rem^08k  |   80.625 kc |   10.078 kc |
| trait_obj^vec^rem^90k  | 1364.740 kc |   15.164 kc |
| trait_obj^vec^sum^01k  |    8.369 kc |    8.369 kc |
| trait_obj^vec^sum^08k  |   66.302 kc |    8.288 kc |
| trait_obj^vec^sum^90k  | 1203.361 kc |   13.371 kc |

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


[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
