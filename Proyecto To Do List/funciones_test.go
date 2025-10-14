package main

import (
	"testing"
	"time"

	"todolist/tareas"

	"github.com/stretchr/testify/assert"
)

func TestAñadirTarea(t *testing.T) {
	var lista tareas.Tareas
	titulo := "Estudiar partituras"

	lista.Añadir(titulo)

	assert.Equal(t, 1, len(lista.Lista))
	assert.Equal(t, titulo, lista.Lista[0].Titulo)
	assert.Equal(t, tareas.Activa, lista.Lista[0].Estado)
	assert.WithinDuration(t, time.Now(), lista.Lista[0].Inicio, time.Second)
	assert.Nil(t, lista.Lista[0].FinalizadaEn)
}

func TestEditarTarea(t *testing.T) {
	lista := tareas.Tareas{
		Lista: []tareas.Tarea{
			{ID: 1, Titulo: "Original", Estado: tareas.Activa, Inicio: time.Now()},
		},
		UltimoID: 1,
	}
	nuevoTitulo := "Título editado"

	err := lista.Editar(1, nuevoTitulo)

	assert.NoError(t, err)
	assert.Equal(t, nuevoTitulo, lista.Lista[0].Titulo)
}

func TestEliminarTarea(t *testing.T) {
	lista := tareas.Tareas{
		Lista: []tareas.Tarea{
			{ID: 1, Titulo: "Tarea 1", Estado: tareas.Activa, Inicio: time.Now()},
			{ID: 2, Titulo: "Tarea 2", Estado: tareas.Activa, Inicio: time.Now()},
		},
		UltimoID: 2,
	}

	err := lista.Eliminar(1)

	assert.NoError(t, err)
	assert.Equal(t, 1, len(lista.Lista))
	assert.Equal(t, "Tarea 2", lista.Lista[0].Titulo)
	assert.Equal(t, 2, lista.Lista[0].ID)
}

func TestEliminarTareaIDInvalido(t *testing.T) {
	var lista tareas.Tareas

	err := lista.Eliminar(5)

	assert.Error(t, err)
}
