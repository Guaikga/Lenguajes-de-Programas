package main

func main() {
	tareas := Tareas{}
	almacenamiento := NuevoAlmacenamiento[Tareas]("tareas.json")
	almacenamiento.Cargar(&tareas)
	CmdMsj := NuevoMensajeCmd()
	CmdMsj.Ejecutar(&tareas)
	almacenamiento.Guardar(tareas)
}
