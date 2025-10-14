package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CmdMsj struct {
	Añadir    string
	Editar    string
	Eliminar  int
	Lista     bool
	SetEstado string
	Ayuda     bool
}

func NuevoMensajeCmd() *CmdMsj {
	cm := CmdMsj{}

	flag.StringVar(&cm.Añadir, "add", "", "Añade una nueva tarea")
	flag.StringVar(&cm.Editar, "edit", "", "Edita una tarea con formato id:Nuevo título")
	flag.IntVar(&cm.Eliminar, "delete", -1, "Elimina una tarea por su índice")
	flag.BoolVar(&cm.Lista, "list", false, "Muestra todas las tareas")
	flag.StringVar(&cm.SetEstado, "setestado", "", "Cambia el estado de una tarea con formato index:estado (0=Activa, 1=Suspendida, 2=Finalizada)")
	flag.BoolVar(&cm.Ayuda, "help", false, "Muestra esta ayuda con todos los comandos disponibles")

	flag.Parse()

	return &cm
}

func (cm *CmdMsj) Ejecutar(tareas *Tareas) {
	switch {
	case cm.Ayuda:
		fmt.Printf(`
Gestor de Tareas - Comandos disponibles:

  -add "Título de la tarea"
      Añade una nueva tarea con el título indicado.

  -edit "index:Nuevo título"
      Edita el título de una tarea existente. Usa comillas y el formato index:Nuevo título.

  -delete N
      Elimina la tarea con índice N.

  -setestado "index:estado"
      Cambia el estado de una tarea directamente. Usa:
        0 = Activa
        1 = Suspendida
        2 = Finalizada

  -list
      Muestra todas las tareas en formato de tabla.

  -help
      Muestra esta ayuda con todos los comandos disponibles.
`)
	case cm.Lista:
		tareas.print()

	case cm.Añadir != "":
		tareas.añadir(cm.Añadir)

	case cm.Editar != "":
		partes := strings.SplitN(cm.Editar, ":", 2)
		if len(partes) != 2 {
			fmt.Println("Error: formato inválido. Usa comillas y el formato index:Nuevo título")
			os.Exit(1)
		}
		index, err := strconv.Atoi(partes[0])
		if err != nil {
			fmt.Println("Error: índice inválido para editar")
			os.Exit(1)
		}
		tareas.editar(index, partes[1])

	case cm.SetEstado != "":
		partes := strings.SplitN(cm.SetEstado, ":", 2)
		if len(partes) != 2 {
			fmt.Println("Error: formato inválido. Usa comillas y el formato index:estado")
			os.Exit(1)
		}
		index, err1 := strconv.Atoi(partes[0])
		estadoInt, err2 := strconv.Atoi(partes[1])
		if err1 != nil || err2 != nil || estadoInt < 0 || estadoInt > 2 {
			fmt.Println("Error: índice o estado inválido. Estado debe ser 0, 1 o 2")
			os.Exit(1)
		}
		tareas.setEstado(index, EstadoTarea(estadoInt))

	case cm.Eliminar != -1:
		tareas.eliminar(cm.Eliminar)

	default:
		fmt.Println("Comando inválido. Usa -help para ver los comandos disponibles.")
	}
}
