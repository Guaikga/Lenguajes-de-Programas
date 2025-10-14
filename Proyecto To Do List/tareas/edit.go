package tareas

import "errors"

func (t *Tareas) buscarPorID(id int) (int, error) {
	for i, tarea := range t.Lista {
		if tarea.ID == id {
			return i, nil
		}
	}
	return -1, errors.New("ID no encontrado")
}

func (t *Tareas) Editar(id int, titulo string) error {
	index, err := t.buscarPorID(id)
	if err != nil {
		return err
	}
	t.Lista[index].Titulo = titulo
	return nil
}
