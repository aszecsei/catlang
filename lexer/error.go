package lexer

import "fmt"

type Error struct {
	pos Position
	msg string
}

func (e Error) Error() string {
	return fmt.Sprint(e.pos, " ", e.msg)
}

// ErrorList is a slice of Error pointers
type ErrorList []*Error

func (el ErrorList) Count() int {
	return len(el)
}

func (el *ErrorList) Add(p Position, args ...interface{}) {
	*el = append(*el, &Error{pos: p, msg: fmt.Sprint(args...)})
}

func (el *ErrorList) cleanup() {
	var last Position
	i := 0
	for _, v := range *el {
		if v.pos != last {
			last = v.pos
			(*el)[i] = v
			i++
		}
	}
	(*el) = (*el)[:i]
}

func (el ErrorList) Error() string {
	var msg string
	el.cleanup()
	for i, err := range el {
		if i >= 10 {
			msg += fmt.Sprintln("More than 10 errors,", el.Count()-10, "more not shown")
			break
		}
		msg += fmt.Sprintln(err)
	}
	return msg
}

func (el ErrorList) Print() {
	for _, err := range el {
		fmt.Println(err)
	}
}
