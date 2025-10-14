package main

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestAñadirTarea(t *testing.T) {
	var tareas Tareas
	titulo := "Estudiar partituras"

	tareas.añadir(titulo)

	assert.Equal(t, 1, len(tareas), "Debe haber una tarea añadida")
	assert.Equal(t, titulo, tareas[0].Titulo, "El título debe coincidir")
	assert.Equal(t, Activa, tareas[0].Estado, "El estado inicial debe ser Activa")
	assert.WithinDuration(t, time.Now(), tareas[0].Inicio, time.Second, "La fecha de inicio debe ser reciente")
	assert.Nil(t, tareas[0].FinalizadaEn, "FinalizadaEn debe ser nil al inicio")
}

func TestEditarTarea(t *testing.T) {
	tareas := Tareas{
		{Titulo: "Original", Estado: Activa, Inicio: time.Now()},
	}
	nuevoTitulo := "Título editado"

	err := tareas.editar(0, nuevoTitulo)

	assert.NoError(t, err, "No debe haber error al editar")
	assert.Equal(t, nuevoTitulo, tareas[0].Titulo, "El título debe actualizarse")
}

func TestEliminarTarea(t *testing.T) {
	tareas := Tareas{
		{Titulo: "Tarea 1", Estado: Activa, Inicio: time.Now()},
		{Titulo: "Tarea 2", Estado: Activa, Inicio: time.Now()},
	}

	err := tareas.eliminar(0)

	assert.NoError(t, err, "No debe haber error al eliminar")
	assert.Equal(t, 1, len(tareas), "Debe quedar una sola tarea")
	assert.Equal(t, "Tarea 2", tareas[0].Titulo, "La tarea restante debe ser la segunda")
}

func TestEliminarTareaIndexInvalido(t *testing.T) {
	var tareas Tareas

	err := tareas.eliminar(5)

	assert.Error(t, err, "Debe devolver error si el índice es inválido")
}
