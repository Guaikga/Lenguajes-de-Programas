package main

import (
	"todolist/tareas"
)

func main() {
	lista := tareas.Tareas{}
	almacen := NuevoAlmacenamiento[tareas.Tareas]("tareas.json")

	almacen.Cargar(&lista)

	cmd := NuevoMensajeCmd()
	cmd.Ejecutar(&lista)

	almacen.Guardar(lista)
}
