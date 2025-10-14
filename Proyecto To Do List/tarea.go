package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"time"
)

type EstadoTarea int

const (
	Activa EstadoTarea = iota
	Suspendida
	Finalizada
)

func (e EstadoTarea) String() string {
	switch e {
	case Activa:
		return "Activa"
	case Suspendida:
		return "Suspendida"
	case Finalizada:
		return "Finalizada"
	default:
		return "Desconocido"
	}
}

func (e EstadoTarea) MarshalJSON() ([]byte, error) {
	return []byte(`"` + e.String() + `"`), nil
}

func (e *EstadoTarea) UnmarshalJSON(data []byte) error {
	switch string(data) {
	case `"Activa"`:
		*e = Activa
	case `"Suspendida"`:
		*e = Suspendida
	case `"Finalizada"`:
		*e = Finalizada
	default:
		*e = Activa
	}
	return nil
}

type Tarea struct {
	Titulo       string      `json:"titulo"`
	Estado       EstadoTarea `json:"estado"`
	Inicio       time.Time   `json:"inicio"`
	FinalizadaEn *time.Time  `json:"finalizada_en"`
}

func (t Tarea) MarshalJSON() ([]byte, error) {
	type TareaJson struct {
		Titulo       string      `json:"titulo"`
		Estado       EstadoTarea `json:"estado"`
		Inicio       time.Time   `json:"inicio"`
		FinalizadaEn *time.Time  `json:"finalizada_en"`
	}
	return json.Marshal(TareaJson{
		Titulo:       t.Titulo,
		Estado:       t.Estado,
		Inicio:       t.Inicio,
		FinalizadaEn: t.FinalizadaEn,
	})
}

type Tareas []Tarea

func (tareas *Tareas) añadir(titulo string) {
	tarea := Tarea{
		Titulo:       titulo,
		Estado:       Activa,
		Inicio:       time.Now(),
		FinalizadaEn: nil,
	}
	*tareas = append(*tareas, tarea)
}

func (tareas *Tareas) validarError(index int) error {
	if index < 0 || index >= len(*tareas) {
		err := errors.New("Index inválido")
		fmt.Println(err)
		return err
	}
	return nil
}

func (tareas *Tareas) eliminar(index int) error {
	t := *tareas
	if err := t.validarError(index); err != nil {
		return err
	}
	*tareas = append(t[:index], t[index+1:]...)
	return nil
}

func (tareas *Tareas) setEstado(index int, estado EstadoTarea) error {
	if err := tareas.validarError(index); err != nil {
		return err
	}

	t := &(*tareas)[index]
	t.Estado = estado

	if estado == Finalizada {
		tiempo := time.Now()
		t.FinalizadaEn = &tiempo
	} else {
		t.FinalizadaEn = nil
	}

	return nil
}

func (tareas *Tareas) editar(index int, titulo string) error {
	if err := tareas.validarError(index); err != nil {
		return err
	}
	(*tareas)[index].Titulo = titulo
	return nil
}

func (tareas *Tareas) print() {
	for i, t := range *tareas {
		fmt.Println("_____________________________________________")
		fmt.Printf("Tarea #%d\n", i)
		fmt.Printf("Título       : %s\n", t.Titulo)
		fmt.Printf("Estado       : %s\n", t.Estado.String())
		fmt.Printf("Creada en    : %s\n", t.Inicio.Format("02 Jan 2006 03:04 PM"))
		if t.FinalizadaEn != nil {
			fmt.Printf("Finalizada en: %s\n", t.FinalizadaEn.Format("02 Jan 2006 03:04 PM"))
		} else {
			fmt.Println("Finalizada en: Sin finalizar")
		}
	}
	fmt.Println("_____________________________________________")
}
