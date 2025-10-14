package tareas

import "time"

func (t *Tareas) Añadir(titulo string) {
	t.UltimoID++
	nueva := Tarea{
		ID:           t.UltimoID,
		Titulo:       titulo,
		Estado:       Activa,
		Inicio:       time.Now(),
		FinalizadaEn: nil,
	}
	t.Lista = append(t.Lista, nueva)
}
