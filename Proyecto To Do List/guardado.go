package main

import (
	"encoding/json"
	"os"
)

type Almacenamiento[T any] struct {
	NombreArchivo string
}

func NuevoAlmacenamiento[T any](nombreArchivo string) *Almacenamiento[T] {
	return &Almacenamiento[T]{NombreArchivo: nombreArchivo}
}

func (a *Almacenamiento[T]) Guardar(data T) error {
	datosArchivo, err := json.MarshalIndent(data, "", "    ")
	if err != nil {
		return err
	}
	return os.WriteFile(a.NombreArchivo, datosArchivo, 0644)
}

func (a *Almacenamiento[T]) Cargar(data *T) error {
	datosArchivo, err := os.ReadFile(a.NombreArchivo)
	if err != nil {
		return err
	}
	return json.Unmarshal(datosArchivo, data)
}
