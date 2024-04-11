# Outstandingly Obvious Time Delta

This provides "more intuitive" time-reading strings.


## Installation

Install OOTD with `pip`

```bash
  pip install ootd
```
    
## Environment Variables

`OOTD_DEFAULT_LOCALE`: Locale string that would be set as the default locale if this is not set, default value would be `'C'`.


## Examples

|prev|now|timedelta|YT|OOTD|
|---|---|---|---|---|
|2021-04-30T11:57:16Z|2024-01-25T13:31:43Z|(days=1000, ...)|2년 전|3년 전|
|2022-09-12T09:40:33Z|2024-01-25T13:31:43Z|(days=500, ...)|1년 전|1년반 전|
|2023-12-09T18:21:29Z|2024-01-25T13:31:43Z|(days=48, ...)|1개월 전|한 달 반 전|
|2024-01-12T14:42:11Z|2024-01-25T13:31:43Z|(days=13, ...)|1주 전|2주 전|
|2024-01-24T20:29:54Z|2024-01-25T13:31:43Z|(hours=6, ...)|17시간 전|어제 밤|
|2024-01-25T03:29:54Z|2024-01-25T13:31:43Z|(hours=9, ...)|10시간 전|오늘 새벽|
|2024-01-25T12:08:43Z|2024-01-25T13:31:43Z|(hours=1, minutes=23, ...)|1시간 전|1시간반 전|
|2024-01-25T12:37:43Z|2024-01-25T13:31:43Z|(minutes=54, ...)|54분 전|54분 전|



## Usage

```python
from datetime import datetime, timedelta
from ootd import OOTD

td = timedelta(days=100)
ootd_100days = OOTD.from_timedelta(td)

print(ootd_100days)         # 3 months later

now = datetime.utcnow()
a_week_ago = now - timedelta(days=7, hours=4, minutes=32, seconds=19)
ootd_a_week = OOTD.between(a_week_ago, now)

print(ootd_a_week)          # a week ago
```

### Language
```python
from datetime import datetime, timedelta
from ootd import OOTD

td = timedelta(days=100)
ootd_100days = OOTD.from_timedelta(td, locale="ko_KR.UTF-8")

print(ootd_100days)         # 3개월 후

import os

os.environ["OOTD_DEFAULT_LOCALE"] = "ko_KR.UTF-8"
now = datetime.utcnow()
a_week_ago = now - timedelta(days=7, hours=4, minutes=32, seconds=19)
ootd_a_week = OOTD.interval_between(a_week_ago, now)

print(ootd_a_week)          # 일주일 전
```

## Authors

👤 Woorak Park
- :octocat: GitHub: [grf53](https://www.github.com/grf53)




## License

[LICENSE.txt](https://github.com/grf53/OOTD/blob/main/LICENSE.txt): [LGPLv3](https://choosealicense.com/licenses/lgpl-3.0/)

