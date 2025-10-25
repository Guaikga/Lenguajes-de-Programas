use super::modelo::{Tarea, Tareas};
use chrono::Local;

impl Tareas {
    pub fn añadir(&mut self, titulo: String) -> u32 {
        self.ultimo_id += 1;
        self.contador_principal += 1;
        
        let nueva = Tarea {
            id: self.ultimo_id,
            titulo,
            estado: super::modelo::EstadoTarea::Activa,
            inicio: Local::now(),
            finalizada_en: None,
            subtareas: Vec::new(),
            es_subtarea: false,
            padre_id: None,
            numero_jerarquico: self.contador_principal.to_string(),
        };
        self.lista.push(nueva);
        self.ultimo_id
    }

    pub fn añadir_subtarea(&mut self, padre_id: u32, titulo: String) -> Result<u32, String> {
        let padre_index = self.buscar_por_id(padre_id)?;
        
        if self.lista[padre_index].es_subtarea {
            return Err("No se pueden añadir subtareas a una subtarea".to_string());
        }

        self.ultimo_id += 1;
        
        let numero_padre = &self.lista[padre_index].numero_jerarquico;
        let numero_subtarea = format!("{}.{}", numero_padre, self.lista[padre_index].subtareas.len() + 1);

        let nueva_subtarea = Tarea {
            id: self.ultimo_id,
            titulo,
            estado: super::modelo::EstadoTarea::Activa,
            inicio: Local::now(),
            finalizada_en: None,
            subtareas: Vec::new(),
            es_subtarea: true,
            padre_id: Some(padre_id),
            numero_jerarquico: numero_subtarea.clone(),
        };

        self.lista[padre_index].subtareas.push(nueva_subtarea);
        self.lista[padre_index].actualizar_estado_por_subtareas();
        
        Ok(self.ultimo_id)
    }

}