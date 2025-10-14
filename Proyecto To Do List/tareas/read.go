package tareas

import (
	"fmt"
)

func (t *Tareas) Imprimir() {
	for _, tarea := range t.Lista {
		fmt.Println("_____________________________________________")
		fmt.Printf("Tarea #%d\n", tarea.ID)
		fmt.Printf("Título       : %s\n", tarea.Titulo)
		fmt.Printf("Estado       : %s\n", tarea.Estado.String())
		fmt.Printf("Creada en    : %s\n", tarea.Inicio.Format("02 Jan 2006 03:04 PM"))

		if tarea.Estado == Finalizada && tarea.FinalizadaEn != nil {
			fmt.Printf("Finalizada en: %s\n", tarea.FinalizadaEn.Format("02 Jan 2006 03:04 PM"))
		}
	}
	fmt.Println("_____________________________________________")
}

func (t *Tareas) ListarFinalizadas() {
	for _, tarea := range t.Lista {
		if tarea.Estado == Finalizada {
			fmt.Printf("Tarea #%d - %s [Finalizada]\n", tarea.ID, tarea.Titulo)
		}
	}
}

func (t *Tareas) ListarActivas() {
	for _, tarea := range t.Lista {
		if tarea.Estado == Activa {
			fmt.Printf("Tarea #%d - %s [Activa]\n", tarea.ID, tarea.Titulo)
		}
	}
}

func (t *Tareas) ListarSuspendidas() {
	for _, tarea := range t.Lista {
		if tarea.Estado == Suspendida {
			fmt.Printf("Tarea #%d - %s [Suspendida]\n", tarea.ID, tarea.Titulo)
		}
	}
}
