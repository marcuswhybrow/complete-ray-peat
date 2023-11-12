package ast

import (
	gmAst "github.com/yuin/goldmark/ast"
)

type Speaker struct {
	gmAst.BaseBlock

	// The short name of a speaker, e.g. "RP", used in markdown.
	ShortName string

	// Is speaking for the first time in this chat.
	IsHello bool

	// A short reponse for which it isn't worth redeclaring the speakers name if
	// the previous was directly interrupting this speaker.
	CanRetort bool
}

func NewSpeaker() *Speaker {
	return &Speaker{
		BaseBlock: gmAst.BaseBlock{},
	}
}

func (s *Speaker) Dump(source []byte, level int) {
	gmAst.DumpHelper(s, source, level, nil, nil)
}

var KindSpeaker = gmAst.NewNodeKind("Speaker")

func (s *Speaker) Kind() gmAst.NodeKind {
	return KindSpeaker
}

func (s *Speaker) IsRay() bool {
	return s.ShortName == "RP"
}

func (s *Speaker) IsRetorting(source []byte) bool {
	return s.CanRetort && len(s.Text(source)) < 50
}