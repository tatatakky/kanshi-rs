# kanshi
kanshi is a simple tool like [Monit](https://mmonit.com/monit/) for monitoring file timestamp.

## why kanshi ?
Japanese `kanshi(監視)` means `monitoring`.

## Usage
```rs
use kanshi_rs::kanshi::Kanshi;
use kanshi_rs::script;
fn main() {
    let kanshi = Kanshi::new("/your/target/monitored/file.pid");
    kanshi.every(3).lazy_exec(|| script("/your/execution/command"));
}
```
if timestamp of target file is changed, execute some script. <br>
In the above program, the target file is monitored every 3 seconds. <br>
And then, you can get log in `/var/log/kanshi.log` like the following.
```
$ tail -f /var/log/kanshi.log
Wed Sep 28 15:23:50 2022 Kanshi[INFO] Start to do monitoring File Timestamp!
Wed Sep 28 15:27:37 2022 Kanshi[INFO] File Timestamp is changed now, then execute script!
```

## Sample
https://github.com/tatatakky/kanshi-rs-sample
