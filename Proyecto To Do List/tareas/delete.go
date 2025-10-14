package tareas

func (t *Tareas) Eliminar(id int) error {
	index, err := t.buscarPorID(id)
	if err != nil {
		return err
	}
	t.Lista = append(t.Lista[:index], t.Lista[index+1:]...)
	return nil
}
