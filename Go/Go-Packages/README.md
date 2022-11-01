These are the packages I use in my GO projects. It's a personal repository, but feel free to use these packages in yours.

- [Logging](#logging)
- [Formats](#formats)
- [BufUtils](#bufutils)

# Logging

[Logging](logging) includes an easy-to-use logging package.

![Logging Output](https://github.com/bbayazit16/Go-Packages/blob/master/.images/logging.png)

![logs.log](https://github.com/bbayazit16/Go-Packages/blob/master/.images/logslog.png)

```go
// [dd/mm/yy h:m:s.ms]

logfile := logging.OpenLogFile("logs.log") // or logs.txt, logs.<anything>

// All functions are variadic, you can pass as many arguments as you'd like.
logging.Info("Grandma has started baking cookies.")

time.Sleep(time.Second * 2)

logging.Warn("Cookies have started burning, act with caution!")

time.Sleep(time.Second * 3)

// Only supports strings, still variadic.
logging.WriteFatal(logfile, "Looks like cookies are inedible...") // Write to logfile.

// logging.Fatal(err, additional messages)
logging.Fatal(errors.New("Cookies have burned!"), "Grandma, what have you done!")
```

# Formats

[Formats](formats) provides a way to color your console outputs.

![Formats Output](https://github.com/bbayazit16/Go-Packages/blob/master/.images/formats.png)

```go
fmt.Println(formats.Format(formats.RED, "Hello, World!", "What a wonderful day!"))
fmt.Println(formats.Format(formats.GREEN, "Hello, World!", "What a wonderful day!"))
fmt.Println(formats.Format(formats.YELLOW, "Hello, World!", "What a wonderful day!"))
fmt.Println(formats.Format(formats.BLUE, "Hello, World!", "What a wonderful day!"))
fmt.Println(formats.Format(formats.BOLD, "Hello, World!", "What a wonderful day!"))

fmt.Println(
    formats.Format(
        formats.UNDERLINED, (formats.Format(formats.MAGENTA, "Hello, World!", "What a wonderful day!"))))
// Many more...
```

# BufUtils

[BufUtils](bufutils) is a really basic library providing hex to buffer, buffer to hex, random hex and random buffer functions.

```go
// Creates a hex string with 8 characters.
// Supports odd-length.
randomHex := bufutils.RandomHex(8)
fmt.Println(randomHex) // c765458c

// 8 random bytes.
randB := bufutils.RandomBytes(8)
fmt.Println(randB) // [69 3 127 184 65 166 110 10]

// Buffer with 8 random bytes.
randBuf := bufutils.RandomBuffer(8)
fmt.Println(randBuf.Bytes()) // Buffer with [131 174 40 200 249 186 14 221]

// Removes 0x prefix if it exists, otherwise appends it.
bufutils.OX(&randomHex)
fmt.Println(randomHex) // 0xc765458c
bufutils.OX(&randomHex)
fmt.Println(randomHex) // c765458c

// Converts hex string (with or without 0x) to bytes buffer.
buf, _ := bufutils.HexBuf(randomHex)
fmt.Println(buf.Bytes()) // [199 101 69 140]

// Converts buffer to hex.
h := bufutils.BufHex(buf)
fmt.Println(h) // c765458c
```

# License
[GNU AGPL 3.0](LICENSE)