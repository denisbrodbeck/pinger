# pinger pings hosts

[![Builds](https://github.com/denisbrodbeck/pinger/workflows/CI/badge.svg)](https://github.com/denisbrodbeck/pinger/actions?query=workflow%3ACI)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache2 Licensed](https://img.shields.io/badge/license-Apache2-blue.svg)](./LICENSE-APACHE)

An easy-to-use cli ping app for Windows. Pinger takes a list of hosts and pings them periodically, printing the csv formatted result to stdout or file. Does not require elevated permissions.

## CLI usage

```bash
# Ping three hosts continuously until aborted using default options
pinger.exe 1.1.1.1 mozilla.org rust-lang.org

# Print the result to a tab-separated file instead of to stdout
pinger.exe -o watch.csv 1.1.1.1 mozilla.org

# Ping each host every 5 seconds using a timeout of 2 seconds
pinger.exe --interval 5 --timeout 2 rust-lang.org
```

All available flags:

```txt
USAGE:
    pinger.exe [OPTIONS] [hosts]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <interval>    Interval between pings (in seconds) [default: 10]
    -o, --output <output>        Output file, stdout if not present
    -t, --timeout <timeout>      Timeout for each ping (in seconds) [default: 3]

ARGS:
    <hosts>...    Hosts to ping
```

## CSV Output

The csv file created by `pinger.exe -i 5 -o out.csv mozilla.org rust-lang.org`:

```csv
Timestamp;Host;Address;Available;RTT;Error;Host;Address;Available;RTT;Error
2020-04-29T10:26:58.518006200+00:00;mozilla.org;63.245.208.195;true;190;;rust-lang.org;143.204.89.25;true;23;
2020-04-29T10:27:03.754666100+00:00;mozilla.org;63.245.208.195;true;188;;rust-lang.org;143.204.89.25;true;18;
2020-04-29T10:27:08.770242600+00:00;mozilla.org;63.245.208.195;true;191;;rust-lang.org;143.204.89.25;true;19;
2020-04-29T10:27:13.549704700+00:00;mozilla.org;;false;-1;"could not resolve address `""mozilla.org:53""`";rust-lang.org;;false;-1;"could not resolve address `""rust-lang.org:53""`"
2020-04-29T10:27:18.567781100+00:00;mozilla.org;;false;-1;"could not resolve address `""mozilla.org:53""`";rust-lang.org;;false;-1;"could not resolve address `""rust-lang.org:53""`"
2020-04-29T10:27:23.565042100+00:00;mozilla.org;;false;-1;"could not resolve address `""mozilla.org:53""`";rust-lang.org;;false;-1;"could not resolve address `""rust-lang.org:53""`"
2020-04-29T10:27:28.780743700+00:00;mozilla.org;63.245.208.195;true;190;;rust-lang.org;143.204.89.25;true;22;
```

Previous output rendered as table:

|Timestamp|Host|Address|Available|RTT|Error|Host|Address|Available|RTT|Error|
|---------|----|-------|---------|---|-----|----|-------|---------|---|-----|
|2020-04-29T10:26:58.518006200+00:00|mozilla.org|63.245.208.195|true|190||rust-lang.org|143.204.89.25|true|23||
|2020-04-29T10:27:03.754666100+00:00|mozilla.org|63.245.208.195|true|188||rust-lang.org|143.204.89.25|true|18||
|2020-04-29T10:27:08.770242600+00:00|mozilla.org|63.245.208.195|true|191||rust-lang.org|143.204.89.25|true|19||
|2020-04-29T10:27:13.549704700+00:00|mozilla.org||false|-1|"could not resolve address `""mozilla.org:53""`"|rust-lang.org||false|-1|"could not resolve address `""rust-lang.org:53""`"|
|2020-04-29T10:27:18.567781100+00:00|mozilla.org||false|-1|"could not resolve address `""mozilla.org:53""`"|rust-lang.org||false|-1|"could not resolve address `""rust-lang.org:53""`"|
|2020-04-29T10:27:23.565042100+00:00|mozilla.org||false|-1|"could not resolve address `""mozilla.org:53""`"|rust-lang.org||false|-1|"could not resolve address `""rust-lang.org:53""`"|
|2020-04-29T10:27:28.780743700+00:00|mozilla.org|63.245.208.195|true|190||rust-lang.org|143.204.89.25|true|22||

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
