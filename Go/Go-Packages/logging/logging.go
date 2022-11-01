package logging

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"

	"github.com/bbayazit16/Go-Packages/formats"
)

// Prints INFO <Green> [time] text<Bold>
func Info(text ...interface{}) {
	formatPrint(logFormat(formats.GREEN, "INFO"))
	formatPrint(formats.BOLD, text...)

	fmt.Println()
}

// Prints WARN <Yellow> [time] text<Bold>
func Warn(text ...interface{}) {
	formatPrint(logFormat(formats.YELLOW, "WARN"))
	formatPrint(formats.BOLD, text...)

	fmt.Println()
}

// Terminates program with panic and
// prints FATAL <Yellow> [time] text<Bold>
func Fatal(err error, text ...interface{}) {
	formatPrint(logFormat(formats.RED, "FATAL"))
	formatPrint(formats.BOLD, text...)

	fmt.Println()

	panic(err)
}

// Writes INFO [time] <text> to "w".
func WriteInfo(w io.Writer, text ...string) {
	w.Write(append(logBytes("INFO"), byteJoin(text...)...))
}

// Writes WARN [time] <text> to "w".
func WriteWarn(w io.Writer, text ...string) {
	w.Write(append(logBytes("WARN"), byteJoin(text...)...))
}

// Writes FATAL [time] <text> to "w" without terminating the program.
func WriteFatal(w io.Writer, text ...string) {
	w.Write(append(logBytes("FATAL"), byteJoin(text...)...))
}

// Opens filename in append-mode, re-usable for Write<Level>.
// Creates file if it doesn't exist. Filename must include extension.
func OpenLogFile(filename string) *os.File {
	file, err := os.OpenFile(filename, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)

	if err != nil {
		Fatal(err, "Unable to open the logfile.")
	}

	return file
}

// Internal function that returns logging format.
// Returns level<color> [dd/mm/yy hours:minutes:seconds]
func logFormat(color string, level string) string {
	return fmt.Sprintf("%s [%s] ",
		formats.Format(color, level),
		time.Now().Local().Format("02/01/2006 15:04:5.000"))
}

// Internal function that returns logging format without color as bytes.
// Returns level [dd/mm/yy hours:minutes:seconds]
func logBytes(level string) []byte {
	return []byte(fmt.Sprintf("%s [%s] ",
		level,
		time.Now().Local().Format("02/01/2006 15:04:5.000")))
}

// Internal function to print the specified format
// and text to console.
func formatPrint(format string, text ...interface{}) {
	fmt.Print(format)

	for i, str := range text {
		if i == len(text)-1 {
			fmt.Print(str)
		} else {
			fmt.Print(str, " ")
		}
	}

	fmt.Print(formats.RESET)
}

// Internal function to join given strings and
// convert them to bytes.
func byteJoin(str ...string) []byte {
	return []byte(strings.Join(str, " ") + "\n")
}
