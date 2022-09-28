# kanshi
kanshi is a simple tool like [Monit](https://mmonit.com/monit/) for monitoring file timestamp.

## why kanshi ?
Japanese `kanshi(監視)` means `monit`.

## Usage
```rs
use kanshi::kanshi::Kanshi;
use kanshi::script;
fn main() {
    let kanshi = Kanshi::new("/your/target/monitored/file.pid");
    kanshi.every(3).lazy_exec(|| script("/your/execution/command"));
}
```
And then, you can get log in `/var/log/kanshi.log` like following
```
$ tail -f /var/log/kanshi.log
Wed Sep 28 15:23:50 2022 Kanshi[INFO] Start to do monitoring File Timestamp!
Wed Sep 28 15:27:37 2022 Kanshi[INFO] File Timestamp is changed now, then execute script!
```
