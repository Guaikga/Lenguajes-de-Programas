use super::modelo::{Tarea, EstadoTarea, Tareas};

impl Tareas {
    pub fn editar(&mut self, id: u32, nuevo_titulo: String) -> Result<(), String> {
        if let Ok(tarea) = self.buscar_tarea_principal_mut(id) {
            tarea.titulo = nuevo_titulo;
            return Ok(());
        }
        
        if let Some(tarea) = self.buscar_subtarea_mut(id) {
            tarea.titulo = nuevo_titulo;
            return Ok(());
        }
        
        Err(format!("ID {} no encontrado", id))
    }

    pub fn set_estado(&mut self, id: u32, nuevo: EstadoTarea) -> Result<(), String> {
        if let Ok(tarea) = self.buscar_tarea_principal_mut(id) {
            tarea.estado = nuevo;
            tarea.finalizada_en = if nuevo == EstadoTarea::Finalizada {
                Some(chrono::Local::now())
            } else {
                None
            };
            
            Self::propagar_estado_a_subtareas(&mut tarea.subtareas, nuevo);
            return Ok(());
        }
        
        if let Some(tarea) = self.buscar_subtarea_mut(id) {
            let padre_id = tarea.padre_id;
            tarea.estado = nuevo;
            tarea.finalizada_en = if nuevo == EstadoTarea::Finalizada {
                Some(chrono::Local::now())
            } else {
                None
            };

            if let Some(padre_id) = padre_id {
                if let Ok(padre) = self.buscar_tarea_principal_mut(padre_id) {
                    padre.actualizar_estado_por_subtareas();
                }
            }
            return Ok(());
        }
        
        Err(format!("ID {} no encontrado", id))
    }

    fn buscar_tarea_principal_mut(&mut self, id: u32) -> Result<&mut Tarea, String> {
        self.lista.iter_mut().find(|t| t.id == id).ok_or(format!("ID {} no encontrado", id))
    }

    fn buscar_subtarea_mut(&mut self, id: u32) -> Option<&mut Tarea> {
        for tarea in &mut self.lista {
            if let Some(subtarea) = Self::buscar_en_subtareas_mut(&mut tarea.subtareas, id) {
                return Some(subtarea);
            }
        }
        None
    }

    fn buscar_en_subtareas_mut(subtareas: &mut [Tarea], id: u32) -> Option<&mut Tarea> {
        for subtarea in subtareas {
            if subtarea.id == id {
                return Some(subtarea);
            }
            if let Some(result) = Self::buscar_en_subtareas_mut(&mut subtarea.subtareas, id) {
                return Some(result);
            }
        }
        None
    }

    fn propagar_estado_a_subtareas(subtareas: &mut [Tarea], estado: EstadoTarea) {
        for subtarea in subtareas {
            subtarea.estado = estado;
            subtarea.finalizada_en = if estado == EstadoTarea::Finalizada {
                Some(chrono::Local::now())
            } else {
                None
            };
            Self::propagar_estado_a_subtareas(&mut subtarea.subtareas, estado);
        }
    }
}