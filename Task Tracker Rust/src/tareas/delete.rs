use super::modelo::Tarea;

use super::modelo::Tareas;

impl Tareas {
    pub fn eliminar(&mut self, id: u32) -> Result<(), String> {
        if let Ok(index) = self.buscar_por_id(id) {
            if !self.lista[index].es_subtarea {
                self.lista.remove(index);
                return Ok(());
            }
        }
        
        for tarea in &mut self.lista {
            if Self::eliminar_de_subtareas(&mut tarea.subtareas, id) {
                tarea.actualizar_estado_por_subtareas();
                return Ok(());
            }
        }
        
        Err(format!("ID {} no encontrado", id))
    }

    fn eliminar_de_subtareas(subtareas: &mut Vec<Tarea>, id: u32) -> bool {
        if let Some(pos) = subtareas.iter().position(|st| st.id == id) {
            subtareas.remove(pos);
            return true;
        }
        
        for subtarea in subtareas {
            if Self::eliminar_de_subtareas(&mut subtarea.subtareas, id) {
                return true;
            }
        }
        
        false
    }
}