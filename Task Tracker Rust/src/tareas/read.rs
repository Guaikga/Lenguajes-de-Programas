use super::modelo::{EstadoTarea, Tarea, Tareas};

impl Tareas {
    pub fn buscar_por_id(&self, id: u32) -> Result<usize, String> {
        self.lista.iter().position(|t| t.id == id).ok_or(format!("ID {} no encontrado", id))
    }

    pub fn listar(&self) {
        if self.lista.is_empty() {
            println!("No hay tareas registradas.");
            return;
        }
        
        for tarea in &self.lista {
            if !tarea.es_subtarea {
                self.imprimir_tarea_con_subtareas(tarea, 0);
            }
        }
        println!("_____________________________________________");
    }

    pub fn listar_por_estado(&self, estado: EstadoTarea) {
        let tareas_filtradas: Vec<&Tarea> = self.lista.iter()
            .filter(|t| t.estado == estado && !t.es_subtarea)
            .collect();
            
        if tareas_filtradas.is_empty() {
            let nombre_estado = match estado {
                EstadoTarea::Activa => "activas",
                EstadoTarea::Finalizada => "finalizadas",
                EstadoTarea::Suspendida => "suspendidas",
            };
            println!("No hay tareas {}.", nombre_estado);
            return;
        }

        for tarea in tareas_filtradas {
            let info_subtareas = if !tarea.subtareas.is_empty() {
                let completadas = tarea.subtareas.iter().filter(|st| st.estado == EstadoTarea::Finalizada).count();
                let total = tarea.subtareas.len();
                format!(" ({}/{})", completadas, total)
            } else {
                String::new()
            };
            println!("Tarea {} (ID: {}) - {} [{}]{}", tarea.numero_jerarquico, tarea.id, tarea.titulo, tarea.estado, info_subtareas);
        }
    }

    pub fn listar_subtareas(&self, padre_id: u32) -> Result<(), String> {
        let padre_index = self.buscar_por_id(padre_id)?;
        let padre = &self.lista[padre_index];
        
        if padre.subtareas.is_empty() {
            return Err(format!("La tarea '{}' ({}) no tiene subtareas", padre.titulo, padre.numero_jerarquico));
        }
        
        println!("Subtareas de '{}' ({}):", padre.titulo, padre.numero_jerarquico);
        for subtarea in &padre.subtareas {
            println!("  {} (ID: {}) - {} [{}]", subtarea.numero_jerarquico, subtarea.id, subtarea.titulo, subtarea.estado);
        }
        
        Ok(())
    }

    fn imprimir_tarea_con_subtareas(&self, tarea: &Tarea, nivel: usize) {
        let indent = "  ".repeat(nivel);
        
        println!("_____________________________________________");
        println!("{}Tarea {} (ID: {})", indent, tarea.numero_jerarquico, tarea.id);
        println!("{}Título       : {}", indent, tarea.titulo);
        println!("{}Estado       : {}", indent, tarea.estado);
        println!("{}Creada en    : {}", indent, tarea.inicio.format("%d %b %Y %I:%M %p"));
        
        if tarea.estado == EstadoTarea::Finalizada {
            if let Some(f) = tarea.finalizada_en {
                println!("{}Finalizada en: {}", indent, f.format("%d %b %Y %I:%M %p"));
            }
        }

        if !tarea.subtareas.is_empty() {
            let activas = tarea.subtareas.iter().filter(|st| st.estado == EstadoTarea::Activa).count();
            let finalizadas = tarea.subtareas.iter().filter(|st| st.estado == EstadoTarea::Finalizada).count();
            let suspendidas = tarea.subtareas.iter().filter(|st| st.estado == EstadoTarea::Suspendida).count();
            
            println!("{}Subtareas    : {} activas, {} finalizadas, {} suspendidas", 
                     indent, activas, finalizadas, suspendidas);
        }

        for subtarea in &tarea.subtareas {
            self.imprimir_tarea_con_subtareas(subtarea, nivel + 1);
        }
    }
}