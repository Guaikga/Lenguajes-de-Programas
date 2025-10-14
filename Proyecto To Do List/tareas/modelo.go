package tareas

import (
	"time"
)

type EstadoTarea int

const (
	Activa EstadoTarea = iota
	Finalizada
	Suspendida
)

func (e EstadoTarea) String() string {
	switch e {
	case Activa:
		return "Activa"
	case Finalizada:
		return "Finalizada"
	case Suspendida:
		return "Suspendida"
	default:
		return "Desconocido"
	}
}

type Tarea struct {
	ID           int         `json:"id"`
	Titulo       string      `json:"titulo"`
	Estado       EstadoTarea `json:"estado"`
	Inicio       time.Time   `json:"inicio"`
	FinalizadaEn *time.Time  `json:"finalizada_en"`
}

type Tareas struct {
	Lista    []Tarea
	UltimoID int
}

func (t *Tareas) SetEstado(id int, nuevo EstadoTarea) error {
	index, err := t.buscarPorID(id)
	if err != nil {
		return err
	}

	tarea := &t.Lista[index]
	tarea.Estado = nuevo

	if nuevo == Finalizada {
		now := time.Now()
		tarea.FinalizadaEn = &now
	} else {
		tarea.FinalizadaEn = nil
	}

	return nil
}
