package formats

import (
	"strings"
)

// ANSI escape codes of corresponding formats.
// For more codes refer to the ANSI table:
// https://en.wikipedia.org/wiki/ANSI_escape_code
const (
	BLACK   = "\u001b[30m"
	RED     = "\u001b[31m"
	GREEN   = "\u001b[32m"
	YELLOW  = "\u001b[33m"
	BLUE    = "\u001b[34m"
	MAGENTA = "\u001b[35m"
	CYAN    = "\u001b[36m"
	WHITE   = "\u001b[37m"

	RESET      = "\u001b[0m"
	BOLD       = "\u001b[1m"
	UNDERLINED = "\u001b[4m"
	STRIKE     = "\u001b[9m"
)

// Joins given strings together with space as seperator.
func join(text ...string) string {
	return strings.Join(text, " ")
}

// Returns painted string with chosen color.
// Automatically clears color afterwards.
// formats.Format(formats.RED, "Hello World!")
func Format(format string, text ...string) string {
	return format + join(text...) + RESET
}
