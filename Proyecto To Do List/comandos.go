package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
	"todolist/tareas"
)

type CmdMsj struct {
	Añadir          string
	Editar          string
	Eliminar        int
	Lista           bool
	SetEstado       string
	Ayuda           bool
	ListActivas     bool
	ListSuspendidas bool
	ListFinalizadas bool
}

func NuevoMensajeCmd() *CmdMsj {
	cm := CmdMsj{}

	flag.StringVar(&cm.Añadir, "add", "", "Añade una nueva tarea")
	flag.StringVar(&cm.Editar, "edit", "", "Edita una tarea con formato id:Nuevo título")
	flag.IntVar(&cm.Eliminar, "delete", -1, "Elimina una tarea por su ID")
	flag.BoolVar(&cm.Lista, "list", false, "Muestra todas las tareas")
	flag.StringVar(&cm.SetEstado, "setestado", "", "Cambia el estado de una tarea con formato id:estado")
	flag.BoolVar(&cm.Ayuda, "help", false, "Muestra esta ayuda")
	flag.BoolVar(&cm.ListActivas, "listactivas", false, "Muestra solo tareas activas")
	flag.BoolVar(&cm.ListSuspendidas, "listsuspendidas", false, "Muestra solo tareas suspendidas")
	flag.BoolVar(&cm.ListFinalizadas, "listfinalizadas", false, "Muestra solo tareas finalizadas")

	flag.Parse()

	return &cm
}

func (cm *CmdMsj) Ejecutar(t *tareas.Tareas) {
	switch {
	case cm.Ayuda:
		fmt.Printf(`
Gestor de Tareas - Comandos disponibles:

  -add "Título de la tarea"
      Añade una nueva tarea.

  -edit "id:Nuevo título"
      Edita el título de una tarea.

  -delete ID
      Elimina la tarea con ID especificado.

  -setestado "id:estado"
      Cambia el estado de una tarea. Usa:
        0 = Activa
        1 = Suspendida
        2 = Finalizada

  -list
      Muestra todas las tareas.

  -listactivas
      Muestra solo tareas activas.

  -listsuspendidas
      Muestra solo tareas suspendidas.

  -listfinalizadas
      Muestra solo tareas finalizadas.

  -help
      Muestra esta ayuda.
`)

	case cm.Lista:
		t.Imprimir()

	case cm.ListActivas:
		t.ListarActivas()

	case cm.ListSuspendidas:
		t.ListarSuspendidas()

	case cm.ListFinalizadas:
		t.ListarFinalizadas()

	case cm.Añadir != "":
		t.Añadir(cm.Añadir)

	case cm.Editar != "":
		partes := strings.SplitN(cm.Editar, ":", 2)
		if len(partes) != 2 {
			fmt.Println("Error: formato inválido. Usa id:Nuevo título")
			os.Exit(1)
		}
		id, err := strconv.Atoi(partes[0])
		if err != nil {
			fmt.Println("Error: ID inválido")
			os.Exit(1)
		}
		if err := t.Editar(id, partes[1]); err != nil {
			fmt.Println("Error al editar:", err)
			os.Exit(1)
		}

	case cm.SetEstado != "":
		partes := strings.SplitN(cm.SetEstado, ":", 2)
		if len(partes) != 2 {
			fmt.Println("Error: formato inválido. Usa id:estado")
			os.Exit(1)
		}
		id, err1 := strconv.Atoi(partes[0])
		estadoInt, err2 := strconv.Atoi(partes[1])
		if err1 != nil || err2 != nil || estadoInt < 0 || estadoInt > 2 {
			fmt.Println("Error: ID o estado inválido. Usa 0=Activa, 1=Suspendida, 2=Finalizada")
			os.Exit(1)
		}
		if err := t.SetEstado(id, tareas.EstadoTarea(estadoInt)); err != nil {
			fmt.Println("Error al cambiar estado:", err)
			os.Exit(1)
		}

	case cm.Eliminar != -1:
		if err := t.Eliminar(cm.Eliminar); err != nil {
			fmt.Println("Error al eliminar:", err)
			os.Exit(1)
		}

	default:
		fmt.Println("Comando inválido. Usa -help para ver los comandos disponibles.")
	}
}
